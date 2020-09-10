use std::collections::HashMap;
use crate::binary_parser_types::SYSTEM_SYMBOL_TABLE;

#[derive(Debug)]
pub struct LocalSymbolTable(Vec<String>);

impl LocalSymbolTable {
    pub fn new() -> LocalSymbolTable {
        LocalSymbolTable(
            SYSTEM_SYMBOL_TABLE
                .to_vec()
                .iter()
                .map(|s| s.to_string())
                .collect(),
        )
    }

    pub fn add_symbol(&mut self, symbol: String) {
        self.0.push(symbol);
    }
}

#[derive(Debug)]
pub struct SharedSymbolTable {
    name: String,
    version: u32,
    symbols: Vec<String>,
}

impl SharedSymbolTable {
    pub fn is_superset(&self, table: &SharedSymbolTable) -> bool {
        for (index, symbol) in table.symbols.into_iter().enumerate() {
            match self.symbols.get(index) {
                Some(&value) if value == symbol => {},
                _ => { return false; }
            }
        }

        true
    }
}

#[derive(Debug)]
enum SymbolContextError {
    TableVersionAlreadyThere
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

    pub fn set_tables_from_current(&mut self, symbols: Vec<String>) {
        for symbol in symbols {
            self.current_table.add_symbol(symbol);
        }
    }

    pub fn add_shared_table(&mut self, name: String, version: u32, symbols: Vec<String>) -> Result<(), SymbolContextError>  {
        let new_table = SharedSymbolTable {
            name,
            version,
            symbols,
        };

        match self.shared_tables.get_mut(&name) {
            Some(table_collection) => match table_collection.1.get_mut(&version) {
                Some(table) => Err(SymbolContextError::TableVersionAlreadyThere),
                None => {
                    if table_collection.0 < version {
                        table_collection.0 = version;
                    }
                    table_collection.1.insert(version, new_table);
                    Ok(())
                }
            },
            None => {
                
            }
        }
    }
}

impl Default for SymbolContext {
    fn default() -> Self {
        Self::new()
    }
}
