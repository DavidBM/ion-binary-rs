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
pub struct LocalSymbolTable(Vec<Symbol>);

impl LocalSymbolTable {
    pub fn new() -> LocalSymbolTable {
        LocalSymbolTable(
            SYSTEM_SYMBOL_TABLE
                .to_vec()
                .iter()
                .map(|s| Symbol::Symbol(s.to_string()))
                .collect(),
        )
    }

    pub fn add_symbol(&mut self, symbol: Symbol) -> usize {
        let id = self.0.len();
        self.0.push(symbol);
        id
    }

    pub fn add_symbols(&mut self, slice: &[Symbol]) {
        for symbol in slice {
            self.add_symbol(symbol.clone());
        }
    }

    pub fn get_symbol_by_id(&self, id: usize) -> Option<&Symbol> {
        self.0.get(id)
    }

    pub fn get_id_by_symbol(&self, symbol: &str) -> Option<usize> {
        match self.0.iter().enumerate().find(|(_, value)| {
            if let Symbol::Symbol(value) = value {
                *value == symbol
            } else {
                false
            }
        }) {
            Some(value) => Some(value.0),
            None => None,
        }
    }

    pub fn insert_dummy_symbols(&mut self, max_len: usize) {
        for _ in 0..max_len {
            self.add_symbol(Symbol::Dummy);
        }
    }

    pub fn list_all_symbols(&self) -> &[Symbol] {
        &self.0
    }
}

#[derive(Debug)]
pub struct SharedSymbolTable {
    name: String,
    version: u32,
    symbols: Vec<Symbol>,
}

impl SharedSymbolTable {
    pub fn is_superset(&self, table: &SharedSymbolTable) -> bool {
        for (index, symbol) in table.symbols.iter().enumerate() {
            match self.symbols.get(index) {
                Some(ref value) if *value == symbol => {}
                _ => {
                    return false;
                }
            }
        }

        true
    }

    pub fn get_symbols_max_len(&self, max_len: usize) -> &[Symbol] {
        if max_len > self.symbols.len() {
            return &self.symbols;
        }

        self.symbols.split_at(max_len).0
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

    pub fn set_new_table_from_current(&mut self, symbols: Vec<Symbol>) {
        for symbol in symbols.into_iter() {
            self.current_table.add_symbol(symbol);
        }
    }

    pub fn add_shared_table(
        &mut self,
        name: String,
        version: u32,
        symbols: &[Symbol],
    ) -> Result<(), SymbolContextError> {
        let new_table = SharedSymbolTable {
            name: name.clone(),
            version,
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
        for index in (*version - 1)..=0 {
            if let Some(existing_table) = tables.get(&index) {
                if !table.is_superset(existing_table) {
                    return Err(SymbolContextError::NewTableIsNotSuperSetOfPrevious);
                } else {
                    return Ok(());
                }
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

        let symbols: Vec<Symbol> = symbols.to_vec();

        for import in imports {
            if import.name == "$ion" {
                continue;
            }

            let version = if let Some(version) = import.version {
                std::cmp::max(1, version)
            } else {
                1
            };

            match self.shared_tables.get(&import.name) {
                Some(table_collection) => match table_collection.1.get(&version) {
                    Some(table) => {
                        let symbols = match import.max_len {
                            Some(len) => table.get_symbols_max_len(len),
                            None => table.get_all_symbols(),
                        };

                        new_table.add_symbols(symbols);
                    }
                    None => {
                        if let Some(max_len) = import.max_len {
                            let table = match table_collection.1.get(&table_collection.0) {
                                Some(table) => table,
                                None => {
                                    return Err(SymbolContextError::InternalParserErrorThisIsABug)
                                }
                            };

                            let symbols = table.get_symbols_max_len(max_len);
                            new_table.add_symbols(symbols);
                        } else {
                            return Err(SymbolContextError::MaxIdNeededWhenImportingASharedTableWhereVersionIsNotAvailable);
                        }
                    }
                },
                None => {
                    if let Some(len) = import.max_len {
                        new_table.insert_dummy_symbols(len);
                    } else {
                        return Err(
                            SymbolContextError::MaxIdNeededWhenImportingANotFoundSharedTable,
                        );
                    }
                }
            }
        }

        new_table.add_symbols(&symbols);

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
        match self.current_table.get_id_by_symbol(symbol) {
            Some(id) => id,
            None => self
                .current_table
                .add_symbol(Symbol::Symbol(symbol.to_string())),
        }
    }

    pub fn dump_all_local_symbols(&self) -> Vec<String> {
        self.current_table.list_all_symbols()[10..]
            .iter()
            .map(|s| match s {
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
