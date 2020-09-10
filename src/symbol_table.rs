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

    pub fn get_symbol_by_id(&self, id: usize) -> Option<&String> {
        self.0.get(id)
    }

    pub fn get_id_by_symbol(&self, symbol: String) -> Option<usize> {
        match self.0.iter().enumerate().find(|(_, value)| *value == &symbol) {
            Some(value) => Some(value.0),
            None => None,
        }
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
        for (index, symbol) in table.symbols.iter().enumerate() {
            match self.symbols.get(index) {
                Some(ref value) if *value == symbol => {},
                _ => { return false; }
            }
        }

        true
    }

    pub fn get_symbols_max_len(&self, max_len: usize) -> &[String] {
        if max_len > self.symbols.len() {
            return &self.symbols;
        }

        self.symbols.split_at(max_len).0
    }

}

#[derive(Debug)]
pub enum SymbolContextError {
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
            name: name.clone(),
            version,
            symbols,
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
}

impl Default for SymbolContext {
    fn default() -> Self {
        Self::new()
    }
}
