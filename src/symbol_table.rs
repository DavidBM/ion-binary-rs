use crate::binary_parser_types::SYSTEM_SYMBOL_TABLE;
use log::trace;
use std::collections::HashMap;

/// A table symbol. It can b used together with the "with_shared_table" method
/// in order to define new shared tables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    Symbol(String),
    Dummy,
}

#[derive(Eq, PartialEq, Debug)]
pub struct LocalSymbolTable {
    entries: Vec<(usize, Symbol)>,
    next_id: usize,
}

impl LocalSymbolTable {
    pub fn new() -> LocalSymbolTable {
        let entries = SYSTEM_SYMBOL_TABLE
            .iter()
            .enumerate()
            .map(|(id, text)| (id, Symbol::Symbol((*text).to_owned())))
            .collect();
        LocalSymbolTable {
            entries,
            next_id: SYSTEM_SYMBOL_TABLE.len(),
        }
    }

    pub fn append_symbols(
        &mut self,
        symbols: impl IntoIterator<Item = Symbol>,
        count: usize,
    ) -> Result<(), SymbolContextError> {
        let start = self.next_id;
        self.next_id = start
            .checked_add(count)
            .ok_or(SymbolContextError::SymbolTableTooLarge)?;
        self.entries.extend(
            symbols
                .into_iter()
                .take(count)
                .enumerate()
                .map(|(index, symbol)| (start + index, symbol)),
        );
        Ok(())
    }

    pub fn get_symbol_by_id(&self, id: usize) -> Option<&Symbol> {
        if self.entries.len() == self.next_id {
            return self.entries.get(id).map(|(_, symbol)| symbol);
        }
        if id >= self.next_id {
            return None;
        }
        // Import padding occupies symbol addresses without allocating one entry per address.
        match self
            .entries
            .binary_search_by_key(&id, |(address, _)| *address)
        {
            Ok(index) => Some(&self.entries[index].1),
            Err(_) => Some(&Symbol::Dummy),
        }
    }

    pub fn get_id_by_symbol(&self, symbol: &str) -> Option<usize> {
        self.entries.iter().find_map(|(id, value)| match value {
            Symbol::Symbol(text) if text == symbol => Some(*id),
            _ => None,
        })
    }
}

#[derive(Debug)]
pub struct SharedSymbolTable {
    _name: String,
    _version: u32,
    symbols: Vec<Symbol>,
}

impl SharedSymbolTable {
    pub fn is_superset(&self, table: &SharedSymbolTable) -> bool {
        for (index, symbol) in table.symbols.iter().enumerate() {
            match self.symbols.get(index) {
                Some(value) if value == symbol => {}
                _ => {
                    return false;
                }
            }
        }

        true
    }

    pub fn get_all_symbols(&self) -> &[Symbol] {
        &self.symbols
    }
}

#[derive(Debug)]
pub struct Import {
    pub(crate) name: String,
    pub(crate) version: Option<u32>,
    pub(crate) max_len: Option<usize>,
}

/// Errors that can happen related with the Symbol Table.
#[derive(Eq, PartialEq, Debug)]
pub enum SymbolContextError {
    TableVersionAlreadyThere,
    MaxIdNeededWhenImportingASharedTableWhereVersionIsNotAvailable,
    MaxIdNeededWhenImportingANotFoundSharedTable,
    InternalParserErrorThisIsABug,
    NewTableIsNotSuperSetOfPrevious,
    SymbolTableTooLarge,
}

#[derive(Debug)]
pub struct SymbolContext {
    current_table: LocalSymbolTable,
    shared_tables: HashMap<String, (u32, HashMap<u32, SharedSymbolTable>)>,
}

impl SymbolContext {
    pub fn new() -> SymbolContext {
        SymbolContext {
            current_table: LocalSymbolTable::new(),
            shared_tables: HashMap::new(),
        }
    }

    pub fn reset_local_table(&mut self) {
        self.current_table = LocalSymbolTable::new();
    }

    pub fn set_new_table_from_current(
        &mut self,
        symbols: Vec<Symbol>,
    ) -> Result<(), SymbolContextError> {
        let count = symbols.len();
        self.current_table.append_symbols(symbols, count)
    }

    pub fn add_shared_table(
        &mut self,
        name: String,
        version: u32,
        symbols: &[Symbol],
    ) -> Result<(), SymbolContextError> {
        let new_table = SharedSymbolTable {
            _name: name.clone(),
            _version: version,
            symbols: symbols.to_vec(),
        };

        match self.shared_tables.get_mut(&name) {
            Some(table_collection) => match table_collection.1.get_mut(&version) {
                Some(_) => Err(SymbolContextError::TableVersionAlreadyThere),
                None => {
                    SymbolContext::assert_new_table_is_superset(
                        &new_table,
                        &version,
                        &table_collection.1,
                    )?;

                    if table_collection.0 < version {
                        table_collection.0 = version;
                    }

                    trace!("New shared table imported {:?}", new_table);

                    table_collection.1.insert(version, new_table);
                    Ok(())
                }
            },
            None => {
                trace!("New shared table imported {:?}", new_table);

                let mut new_hashmap = HashMap::new();
                new_hashmap.insert(version, new_table);
                let new_tuple = (version, new_hashmap);
                self.shared_tables.insert(name, new_tuple);

                Ok(())
            }
        }
    }

    pub fn assert_new_table_is_superset(
        table: &SharedSymbolTable,
        version: &u32,
        tables: &HashMap<u32, SharedSymbolTable>,
    ) -> Result<(), SymbolContextError> {
        for (existing_version, existing_table) in tables {
            let (older, newer) = if existing_version < version {
                (existing_table, table)
            } else {
                (table, existing_table)
            };
            if !newer.is_superset(older) {
                return Err(SymbolContextError::NewTableIsNotSuperSetOfPrevious);
            }
        }

        Ok(())
    }

    pub fn set_new_table(
        &mut self,
        imports: &[Import],
        symbols: &[Symbol],
    ) -> Result<(), SymbolContextError> {
        let mut new_table = LocalSymbolTable::new();

        for import in imports {
            if import.name == "$ion" {
                continue;
            }

            let version = import.version.unwrap_or(1).max(1);
            let available = self.shared_tables.get(&import.name);
            let exact = available.and_then(|(_, tables)| tables.get(&version));
            let (known, count) = match (exact, import.max_len) {
                (Some(table), count) => {
                    let known = table.get_all_symbols();
                    (known, count.unwrap_or(known.len()))
                }
                (None, Some(count)) => {
                    let substitute = available.and_then(|(latest, tables)| tables.get(latest));
                    let known = substitute.map_or(&[][..], SharedSymbolTable::get_all_symbols);
                    (known, count)
                }
                (None, None) if available.is_some() => {
                    return Err(SymbolContextError::MaxIdNeededWhenImportingASharedTableWhereVersionIsNotAvailable);
                }
                (None, None) => {
                    return Err(SymbolContextError::MaxIdNeededWhenImportingANotFoundSharedTable);
                }
            };
            new_table.append_symbols(known.iter().cloned(), count)?;
        }

        new_table.append_symbols(symbols.iter().cloned(), symbols.len())?;

        trace!(
            "New local table importing {:?} resulting in: {:?}",
            imports,
            new_table
        );

        self.current_table = new_table;

        Ok(())
    }

    pub fn get_symbol_by_id(&self, id: usize) -> Option<&Symbol> {
        self.current_table.get_symbol_by_id(id)
    }

    pub fn insert_symbol(&mut self, symbol: &str) -> usize {
        if let Some(id) = self.current_table.get_id_by_symbol(symbol) {
            return id;
        }
        let id = self.current_table.next_id;
        self.current_table
            .append_symbols([Symbol::Symbol(symbol.to_owned())], 1)
            .expect("encoder symbol count fits in memory");
        id
    }

    pub fn dump_all_local_symbols(&self) -> Vec<String> {
        self.current_table.entries[SYSTEM_SYMBOL_TABLE.len()..]
            .iter()
            .map(|(_, symbol)| match symbol {
                Symbol::Symbol(name) => name.clone(),
                _ => "".to_string(),
            })
            .collect()
    }
}

impl Default for SymbolContext {
    fn default() -> Self {
        Self::new()
    }
}
