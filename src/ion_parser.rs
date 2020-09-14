use crate::binary_parser::IonBinaryParser;
use crate::binary_parser_types::*;
use crate::ion_parser_types::*;
use crate::symbol_table::*;
use std::convert::TryInto;
use std::io::Read;

#[derive(Debug)]
pub struct IonParser<T: Read> {
    parser: IonBinaryParser<T>,
    context: SymbolContext,
}

impl<T: Read> IonParser<T> {
    pub fn new(reader: T) -> IonParser<T> {
        IonParser {
            parser: IonBinaryParser::new(reader),
            context: SymbolContext::new(),
        }
    }

    pub fn consume_value(&mut self) -> Result<IonValue, IonParserError> {
        let value_header = self.parser.consume_value_header()?;

        match value_header.r#type {
            ValueType::Bool(value) => Ok(IonValue::Bool(value)),
            ValueType::Annotation => self.consume_annotation(&value_header),
            ValueType::Struct => self.consume_struct(&value_header),
            ValueType::List => self.consume_list(&value_header),
            ValueType::Symbol => self.consume_symbol(&value_header),
            _ => Err(IonParserError::Unimplemented),
        }
    }

    pub fn consume_struct(&mut self, header: &ValueHeader) -> Result<IonValue, IonParserError> {
        unimplemented!()
    }

    pub fn consume_list(&mut self, header: &ValueHeader) -> Result<IonValue, IonParserError> {
        unimplemented!()
    }

    pub fn consume_annotation(
        &mut self,
        header: &ValueHeader,
    ) -> Result<Option<IonValue>, IonParserError> {
        let length = match header.length {
            ValueLength::LongLength => self.parser.consume_varuint()?.0,
            ValueLength::ShortLength(len) => len.into(),
            ValueLength::NullValue => return Err(IonParserError::NullAnnotationFound),
        };

        let mut remaining_annot_bytes = self.parser.consume_varuint()?.0;

        let mut symbols = Vec::new();

        while remaining_annot_bytes > 0 {
            let (annot, consumed_bytes) = self.parser.consume_varuint()?;

            symbols.push(annot);

            remaining_annot_bytes = match remaining_annot_bytes.checked_sub(consumed_bytes as u64) {
                Some(result) => result,
                None => return Err(IonParserError::BadFormatLengthFound),
            }
        }

        let is_shared_table_declaration =
            self.contains_system_symbol(&symbols, SystemSymbolIds::IonSharedSymbolTable);

        let is_local_table_declaration =
            self.contains_system_symbol(&symbols, SystemSymbolIds::IonSymbolTable);

        let value = self.consume_value()?;

        match (is_shared_table_declaration, is_local_table_declaration) {
            (true, true) => {
                return Err(IonParserError::SharedTableAndLocalTableDeclarationIntTheSameAnnotation)
            }
            (true, false) => {
                self.load_shared_table(value);
                Ok(None)
            },
            (false, true) => {
                self.load_local_table(value);
                Ok(None)
            },
            (false, false) => return Ok(Some(self.construct_raw_annotation(&symbols, value)?)),
        }

        //TODO: Check annotation symbols in order to know what to do with the content. It can be a symtem table, shared table, etc
    }

    fn load_local_table(&self, table: IonValue) -> Result<(), IonParserError> {
        let table = if let IonValue::Struct(table) = table {
            table
        } else {
            return Err(IonParserError::LocalTableWithoutInternalStruct);
        };

        let imports = table.get(self.get_symbol_name_by_type(SystemSymbolIds::Imports))
        .ok_or(IonParserError::LocalTableDefinitionWIthoutImportsField)?;

        let imports = match imports {
            IonValue::Symbol(symbol ) if symbol == self.get_symbol_name_by_type(SystemSymbolIds::IonSymbolTable) => {
                Vec::new()
            },
            IonValue::List(list) => {
                
            }
            _ => {
                return Err(IonParserError::LocalSymbolTableWithoutValidImport)
            }
        }

        let symbols = table.get(self.get_symbol_name_by_type(SystemSymbolIds::Symbols));
        
        let symbols: Vec<Symbol> = if let Some(IonValue::List(symbols)) = symbols {
            let symbols_string = Vec::new();

            for symbol in symbols {
                if let IonValue::String(text) = symbol {
                    symbols_string.push(Symbol::Symbol(text.clone()));
                } else {
                    symbols_string.push(Symbol::Dummy);
                }
            }

            symbols_string
        } else {
            Vec::new()
        };

        unimplemented!()
    }

    fn load_shared_table(&self, table: IonValue) -> Result<(), IonParserError> {
        let table = if let IonValue::Struct(table) = table {
            table
        } else {
            return Err(IonParserError::LocalTableWithoutInternalStruct);
        };

        let name = table
            .get(self.get_symbol_name_by_type(SystemSymbolIds::Name))
            .ok_or(IonParserError::SharedTableDefinitionWithoutName)?;

        let version = table.get(self.get_symbol_name_by_type(SystemSymbolIds::Version));

        let symbols = table.get(self.get_symbol_name_by_type(SystemSymbolIds::Symbols));

        let version: u32 = if let Some(IonValue::Integer(version)) = version {
            (*version)
                .try_into()
                .expect("Shared table version too big. It doesn't fit into a u32")
        } else {
            1
        };

        let name: String = if let IonValue::String(name) = name {
            name.clone()
        } else {
            return Err(IonParserError::SharedTableDefinitionWithoutName);
        };

        let symbols: Vec<Symbol> = if let Some(IonValue::List(symbols)) = symbols {
            let symbols_string = Vec::new();

            for symbol in symbols {
                if let IonValue::String(text) = symbol {
                    symbols_string.push(Symbol::Symbol(text.clone()));
                } else {
                    symbols_string.push(Symbol::Dummy);
                }
            }

            symbols_string
        } else {
            Vec::new()
        };

        self.context
            .add_shared_table(name, version, &symbols)
            .map_err(|e| IonParserError::ErrorAddingSharedTableToContext(e))?;

        Ok(())
    }

    fn get_symbol_name_by_type(&self, symbol: SystemSymbolIds) -> &'static str {
        SYSTEM_SYMBOL_TABLE[symbol as usize]
    }

    fn construct_raw_annotation(
        &self,
        symbols: &[u64],
        value: IonValue,
    ) -> Result<IonValue, IonParserError> {
        let symbols_names = Vec::new();

        for symbol in symbols.into_iter() {
            let name = self.get_symbol_name(*symbol)?;
            symbols_names.push(name);
        }

        Ok(IonValue::Annotation((symbols_names, Box::new(value))))
    }

    fn contains_system_symbol(&self, symbols: &[u64], symbol: SystemSymbolIds) -> bool {
        symbols.iter().find(|&&s| s == symbol as u64).is_some()
    }

    fn get_symbol_name(&self, symbol_id: u64) -> Result<String, IonParserError> {
        match self.context.get_symbol_by_id(symbol_id.try_into().unwrap()) {
            Some(Symbol::Symbol(name)) => Ok(name.clone()),
            Some(Symbol::Dummy) | None => Err(IonParserError::SymbolIdNotDefined),
        }
    }
}

/*

Basically, for QLDB, the first value that the DB sends is:
Annotation:
    notation: 3 ($ion_symbol_table which means that is a local symbol table)
    Struct
        Symbols (via the id 7)
        List: Which contains the list of new Symbols

... After consuming the annotation header
Annotation Length: Ok(38)
Annotation annot_length: Ok(1)
Annotation annot: Ok(3)
Annotation value header: Ok(ValueHeader { type: Struct, length: LongLength })
Annotation value length: Ok(34)
Annotation value first key: Ok(7)
Annotation value first value header: Ok(ValueHeader { type: List, length: LongLength })

In the list, symbols are added in consecutive IDs following their insert order.
A symbol cannot replace an already existing symbol. So, the system symbols come first,
later the imported symbols, and finally, the local symbols.
 */
