use std::cmp;
use std::rc::Rc;

#[derive(Debug)]
pub struct TableImport {
    table: Rc<SymbolTable>,
    max_id: usize,
    dummy: bool,
}

impl TableImport {
    pub fn len(&self) -> usize {
        cmp::min(self.max_id, self.table.total_len)
    }
}

pub enum SymbolTableError {
    NotInIndex,
    TableImportNotFound,
}

#[derive(Debug)]
pub struct SymbolTable {
    name: String,
    shared: bool,
    version: u64,
    //Note: First table must be the system table
    imported_tables: Vec<TableImport>,
    values: Vec<String>,
    total_len: usize,
}

impl SymbolTable {
    pub fn new(
        name: String,
        shared: bool,
        version: u64,
        imported_tables: Vec<TableImport>,
        values: Vec<String>,
    ) -> SymbolTable {
        let mut total_len = 0;

        if !shared {
            for imported_table in &imported_tables {
                total_len += imported_table.len();
            }
        }

        total_len += values.len();

        SymbolTable {
            name,
            version,
            imported_tables,
            values,
            shared,
            total_len,
        }
    }

    fn substract_to_index(&self, index: &mut usize, value: usize) -> Result<(), SymbolTableError> {
        match index.checked_sub(value) {
            Some(value) => {
                *index = value;
                Ok(())
            }
            None => Err(SymbolTableError::NotInIndex),
        }
    }

    fn search_in_imported_table(
        &self,
        imported_table: &TableImport,
        mut index: &mut usize,
        id: usize,
    ) -> Result<Option<String>, SymbolTableError> {
        match imported_table.table.find(id) {
            Ok(value) => {
                if imported_table.dummy {
                    Err(SymbolTableError::TableImportNotFound)
                } else {
                    Ok(Some(value))
                }
            }
            Err(SymbolTableError::NotInIndex) => {
                self.substract_to_index(&mut index, imported_table.max_id)?;
                Ok(None)
            }
            Err(SymbolTableError::TableImportNotFound) => {
                Err(SymbolTableError::TableImportNotFound)
            }
        }
    }

    pub fn find(&self, id: usize) -> Result<String, SymbolTableError> {
        let mut index = id.clone();

        if !self.shared {
            for imported_table in &self.imported_tables {
                if index > imported_table.max_id {
                    self.substract_to_index(&mut index, imported_table.max_id)?;
                    continue;
                }

                if let Some(value) =
                    self.search_in_imported_table(imported_table, &mut index, id)?
                {
                    return Ok(value);
                }
            }
        }

        self.substract_to_index(&mut index, 1)?;

        match self.values.get(index).map(|value| value.clone()) {
            Some(value) => Ok(value),
            None => Err(SymbolTableError::NotInIndex),
        }
    }
}
