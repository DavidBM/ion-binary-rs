use std::collections::HashMap;
use crate::binary_parser_types::SYSTEM_SYMBOL_TABLE;
use log::trace;

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Symbol(String),
    Dummy,
}

#[derive(Debug)]
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

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.0.push(symbol);
    }

    pub fn add_symbols(&mut self, slice: &[Symbol]) {
        for symbol in slice {
            self.add_symbol(symbol.clone());
        }
    }

    pub fn get_symbol_by_id(&self, id: usize) -> Option<&Symbol> {
        self.0.get(id)
    }

    /*pub fn get_id_by_symbol(&self, symbol: String) -> Option<usize> {
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
    }*/

    pub fn insert_dummy_symbols(&mut self, max_len: usize) {
        for _ in 0..max_len {
            self.add_symbol(Symbol::Dummy);
        }
    }
}

#[derive(Debug)]
pub struct SharedSymbolTable {
    name: String,
    version: u32,
    symbols: Vec<Symbol>,
}

impl SharedSymbolTable {
    // TODO: Apply this function
    /*pub fn is_superset(&self, table: &SharedSymbolTable) -> bool {
        for (index, symbol) in table.symbols.iter().enumerate() {
            match self.symbols.get(index) {
                Some(ref value) if *value == symbol => {},
                _ => { return false; }
            }
        }

        true
    }*/

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

pub struct Import {
    pub(crate) name: String,
    pub(crate) version: Option<u32>,
    pub(crate) max_len: Option<usize>,
}

#[derive(Debug)]
pub enum SymbolContextError {
    TableVersionAlreadyThere,
    MaxIdNeededWhenImportingASharedTableWhereVersionIsNotAvailable,
    MaxIdNeededWhenImportingANotFoundSharedTable,
    InternalParserErrorThisIsABug
}

#[derive(Debug)]
pub struct SymbolContext {
    current_table: LocalSymbolTable,
    shared_tables: HashMap<String, (u32, HashMap<u32, SharedSymbolTable>)>
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

    pub fn add_shared_table(&mut self, name: String, version: u32, symbols: &[Symbol]) -> Result<(), SymbolContextError>  {
        let new_table = SharedSymbolTable {
            name: name.clone(),
            version,
            symbols: symbols.to_vec(),
        };

        match self.shared_tables.get_mut(&name) {
            Some(table_collection) => match table_collection.1.get_mut(&version) {
                Some(_) => Err(SymbolContextError::TableVersionAlreadyThere),
                None => {
                    if table_collection.0 < version {
                        table_collection.0 = version;
                    }
                    table_collection.1.insert(version, new_table);
                    Ok(())
                }
            },
            None => {
                let mut new_hashmap = HashMap::new();
                new_hashmap.insert(version, new_table);
                let new_tuple = (version, new_hashmap);
                self.shared_tables.insert(name, new_tuple);
                Ok(())
            }
        }
    }

    pub fn set_new_table(&mut self, imports: &[Import], symbols: &[Symbol]) -> Result<(), SymbolContextError> {
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
                Some(table_collection) => {
                    match table_collection.1.get(&version) {
                        Some(table) => {
                            let symbols = match import.max_len {
                                Some(len) => {
                                    table.get_symbols_max_len(len)
                                },
                                None => {
                                    table.get_all_symbols()
                                }
                            };

                            new_table.add_symbols(symbols);
                        },
                        None => {
                            if let Some(max_len) = import.max_len {
                                let table = match table_collection.1.get(&table_collection.0) {
                                    Some(table) => {
                                        table
                                    },
                                    None => {
                                        return Err(SymbolContextError::InternalParserErrorThisIsABug)
                                    }
                                };

                                let symbols = table.get_symbols_max_len(max_len);
                                new_table.add_symbols(symbols);
                            } else {
                                return Err(SymbolContextError::MaxIdNeededWhenImportingASharedTableWhereVersionIsNotAvailable)
                            }
                        }
                    }
                }, 
                None => {
                    if let Some(len) = import.max_len {
                        new_table.insert_dummy_symbols(len);
                    } else {
                        return Err(SymbolContextError::MaxIdNeededWhenImportingANotFoundSharedTable);
                    }
                }
            }
        }

        trace!("Add symbols to Local table {:?}", symbols);

        new_table.add_symbols(&symbols);

        self.current_table = new_table;

        Ok(())
    }

    pub fn  get_symbol_by_id(&self, id: usize) -> Option<&Symbol> {
        self.current_table.get_symbol_by_id(id)
    }

    /*pub fn  get_id_by_symbol(&self, symbol: String) -> Option<usize> {
        self.current_table.get_id_by_symbol(symbol)
    }*/
}

impl Default for SymbolContext {
    fn default() -> Self {
        Self::new()
    }
}
