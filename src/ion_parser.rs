use crate::binary_parser::IonBinaryParser;
use crate::binary_parser_types::*;
use crate::ion_parser_types::*;
use crate::symbol_table::*;
use chrono::{
    naive::{NaiveDate, NaiveDateTime},
    Duration,
};
use log::trace;
use std::convert::{TryFrom, TryInto};
use std::{collections::HashMap, io::Read};

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

    pub fn consume_value(&mut self) -> Result<(IonValue, usize), IonParserError> {
        let value_header = self.parser.consume_value_header()?;

        let mut value = match value_header.r#type {
            ValueType::Bool(value) => (IonValue::Bool(value), 1),
            ValueType::Annotation => match self.consume_annotation(&value_header)? {
                (Some(annotation), consumed_bytes) => (annotation, consumed_bytes),
                (None, consumed_bytes) => {
                    let value = self.consume_value()?;
                    (value.0, value.1 + consumed_bytes)
                }
            },
            ValueType::Struct => self.consume_struct(&value_header)?,
            ValueType::List => self.consume_list(&value_header)?,
            ValueType::Symbol => self.consume_symbol(&value_header)?,
            ValueType::PositiveInt => self.consume_int(&value_header, false)?,
            ValueType::NegativeInt => self.consume_int(&value_header, true)?,
            ValueType::String => self.consume_string(&value_header)?,
            _ => Err(IonParserError::Unimplemented)?,
        };

        //We increase the consumed bytes count as we must count for the already consumed ValueHeader
        value.1 += 1;

        Ok(value)
    }

    fn consume_string(
        &mut self,
        header: &ValueHeader,
    ) -> Result<(IonValue, usize), IonParserError> {
        trace!("Consuming String");

        let (length, _, total) = self.consume_value_len(header)?;

        let mut buffer = vec![0; length as usize];

        self.parser.read_bytes(&mut buffer)?;

        let text = match String::from_utf8(buffer) {
            Ok(text) => text,
            Err(_) => return Err(IonParserError::NonUtf8String),
        };

        Ok((IonValue::String(text), total))
    }

    fn consume_int(
        &mut self,
        header: &ValueHeader,
        negative: bool,
    ) -> Result<(IonValue, usize), IonParserError> {
        trace!("Consuming Integer");

        let (length, _, total) = self.consume_value_len(header)?;

        let value = self
            .parser
            .consume_uint(length.try_into().expect("Symbol length too big for usize"))?;

        let value: i64 = if negative {
            -(i64::try_from(value).expect("integer doesn't fit into i64"))
        } else {
            value.try_into().expect("integer doesn't fit into i64")
        };

        Ok((IonValue::Integer(value), total))
    }

    fn consume_struct(
        &mut self,
        header: &ValueHeader,
    ) -> Result<(IonValue, usize), IonParserError> {
        trace!("Consuming Struct");

        let (length, _, total) = self.consume_value_len(header)?;

        let mut consumed_bytes = 0;
        let mut values: HashMap<String, IonValue> = HashMap::new();

        while length - consumed_bytes > 0 {
            let key = self.parser.consume_varuint()?;
            let value = self.consume_value()?;

            consumed_bytes = consumed_bytes + value.1 as u64;
            consumed_bytes = consumed_bytes + key.1 as u64;

            println!("Struct field -> Key: {:?}, Values: {:?}", key, values);

            let key = match self
                .context
                .get_symbol_by_id(key.0.try_into().expect("Struct key doesn't fir into usize"))
            {
                Some(Symbol::Symbol(text)) => text.clone(),
                _ => return Err(IonParserError::SymbolNotFoundInTable),
            };

            values.insert(key, value.0);
        }

        if let None = length.checked_sub(consumed_bytes) {
            return Err(IonParserError::ListLengthWasTooShort);
        }

        Ok((IonValue::Struct(values), total))
    }

    fn consume_list(&mut self, header: &ValueHeader) -> Result<(IonValue, usize), IonParserError> {
        trace!("Consuming List");

        let (length, _, total) = self.consume_value_len(header)?;

        let mut consumed_bytes = 0;
        let mut values = vec![];

        while length - consumed_bytes > 0 {
            let value = self.consume_value()?;

            consumed_bytes = consumed_bytes + value.1 as u64;
            values.push(value.0);
        }

        if let None = length.checked_sub(consumed_bytes) {
            return Err(IonParserError::ListLengthWasTooShort);
        }

        Ok((IonValue::List(values), total))
    }

    fn consume_symbol(
        &mut self,
        header: &ValueHeader,
    ) -> Result<(IonValue, usize), IonParserError> {
        trace!("Consuming Symbol");

        let (length, _, total_consumed_bytes) = self.consume_value_len(header)?;

        let symbol_id = self
            .parser
            .consume_uint(length.try_into().expect("Symbol length too big for usize"))?;

        let symbol = self
            .context
            .get_symbol_by_id(symbol_id.try_into().expect("Symbol id too big for usize"));

        let text = match symbol {
            Some(Symbol::Symbol(text)) => text.clone(),
            _ => return Err(IonParserError::SymbolNotFoundInTable),
        };

        Ok((IonValue::Symbol(text), total_consumed_bytes))
    }

    fn consume_timestamp(
        &mut self,
        header: &ValueHeader,
    ) -> Result<(IonValue, usize), IonParserError> {
        trace!("Consuming Timestamp");

        let (length, _, mut total_consumed_bytes) = self.consume_value_len(header)?;

        let (offset, consumed) = self.parser.consume_varint()?;
        total_consumed_bytes += consumed;
        let (year, consumed) = self.parser.consume_varuint()?;
        total_consumed_bytes += consumed;

        let mut values = vec![];

        for _ in 0..6 {
            if total_consumed_bytes as u64 >= length {
                break;
            }

            let (value, consumed) = self.parser.consume_varuint()?;
            total_consumed_bytes += consumed;
            values.push(value);
        }

        let month: u32 = values
            .get(0)
            .unwrap_or(&0)
            .clone()
            .try_into()
            .expect("Month doesn't fit into i32");
        let day: u32 = values
            .get(1)
            .unwrap_or(&0)
            .clone()
            .try_into()
            .expect("Day doesn't fit into i32");
        let hour: u32 = values
            .get(2)
            .unwrap_or(&0)
            .clone()
            .try_into()
            .expect("Hour doesn't fit into i32");
        let minute: u32 = values
            .get(3)
            .unwrap_or(&0)
            .clone()
            .try_into()
            .expect("Minute doesn't fit into i32");
        let second: u32 = values
            .get(4)
            .unwrap_or(&0)
            .clone()
            .try_into()
            .expect("Second doesn't fit into i32");

        let fraction_exponent: i32 = if (total_consumed_bytes as u64) < length {
            let value = self.parser.consume_varint()?;
            total_consumed_bytes += value.1;
            value.0
                .try_into()
                .expect("fraction_exponent length doesn't fit in a u32")
        } else {
            0
        };

        let fraction_coefficient: i32 = if (total_consumed_bytes as u64) < length {
            let length: usize = length
                .try_into()
                .expect("Timestamp length doesn't fit in a i32");
            let remaining_bytes = length - total_consumed_bytes;
            let value = self.parser.consume_int(remaining_bytes)?;
            total_consumed_bytes += remaining_bytes;
            value
                .try_into()
                .expect("fraction_exponent length doesn't fit in a u32")
        } else {
            0
        };

        let second_fraction: f64 = ((10f64.powi(fraction_exponent)) * fraction_coefficient as f64) * 1e9;

        let date = NaiveDate::from_ymd(year.try_into().expect("Year too bug for i32"), month, day)
            .and_hms_nano(hour, minute, second, second_fraction as u32);

        // TODO handle the time zone
        // https://docs.rs/chrono/0.4.15/chrono/struct.DateTime.html#method.from_utc
        // https://docs.rs/chrono/0.4.15/chrono/struct.DateTime.html#method.with_timezone

        unimplemented!()
    }

    fn consume_annotation(
        &mut self,
        header: &ValueHeader,
    ) -> Result<(Option<IonValue>, usize), IonParserError> {
        trace!("Consuming Annotation");

        let (_, _, total_consumed_bytes) = self.consume_value_len(header)?;

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
                self.load_shared_table(value.0)?;
                Ok((None, total_consumed_bytes))
            }
            (false, true) => {
                self.load_local_table(value.0)?;
                Ok((None, total_consumed_bytes))
            }
            (false, false) => {
                return Ok((
                    Some(self.construct_raw_annotation(&symbols, value.0)?),
                    total_consumed_bytes,
                ))
            }
        }
    }

    fn consume_value_len(
        &mut self,
        header: &ValueHeader,
    ) -> Result<(u64, usize, usize), IonParserError> {
        let mut consumed_bytes: usize = 0;

        let length = match header.length {
            ValueLength::LongLength => {
                let len = self.parser.consume_varuint()?;
                consumed_bytes += len.1;
                len.0
            }
            ValueLength::ShortLength(len) => len.into(),
            ValueLength::NullValue => return Err(IonParserError::NullAnnotationFound),
        };

        let total = consumed_bytes
            .checked_add(
                length
                    .try_into()
                    .expect("Value length doesn't fit in usize"),
            )
            .unwrap();

        Ok((length, consumed_bytes, total))
    }

    fn get_parsed_struct_hashmap<'a>(
        &self,
        table: &'a IonValue,
    ) -> Result<&'a HashMap<String, IonValue>, IonParserError> {
        if let IonValue::Struct(table) = table {
            Ok(table)
        } else {
            Err(IonParserError::LocalTableWithoutInternalStruct)
        }
    }

    fn get_symbols_string(&self, table: &HashMap<String, IonValue>) -> Vec<Symbol> {
        let symbols = table.get(self.get_symbol_name_by_type(SystemSymbolIds::Symbols));

        if let Some(IonValue::List(symbols)) = symbols {
            let mut symbols_string = Vec::new();

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
        }
    }

    fn load_local_table(&mut self, table: IonValue) -> Result<(), IonParserError> {
        trace!("Loading Local Table");

        let table = self.get_parsed_struct_hashmap(&table)?;

        let symbols = self.get_symbols_string(&table);

        let imports = table.get(self.get_symbol_name_by_type(SystemSymbolIds::Imports));

        let imports = match imports {
            None => Vec::new(),
            Some(IonValue::Symbol(symbol))
                if symbol == self.get_symbol_name_by_type(SystemSymbolIds::IonSymbolTable) =>
            {
                self.context.set_new_table_from_current(symbols);
                return Ok(());
            }
            Some(IonValue::List(list)) => self.decode_imports(list)?,
            _ => return Err(IonParserError::LocalSymbolTableWithoutValidImport),
        };

        self.context
            .set_new_table(&imports, &symbols)
            .map_err(|e| IonParserError::ErrorAddingCreatingLocal(e))?;

        Ok(())
    }

    fn decode_imports(&self, values: &[IonValue]) -> Result<Vec<Import>, IonParserError> {
        let mut imports = Vec::new();

        for value in values {
            let value = match value {
                IonValue::Struct(value) => value,
                _ => continue,
            };

            let name = match value.get(self.get_symbol_name_by_type(SystemSymbolIds::Name)) {
                Some(IonValue::String(name)) => name.clone(),
                _ => continue,
            };

            let version: u32 =
                match value.get(self.get_symbol_name_by_type(SystemSymbolIds::Version)) {
                    Some(IonValue::Integer(version)) => (*version)
                        .try_into()
                        .expect("Import version in Local table import is bigger than u32."),
                    _ => 1,
                };

            let max_len: Option<usize> =
                match value.get(self.get_symbol_name_by_type(SystemSymbolIds::MaxId)) {
                    Some(IonValue::Integer(version)) => Some(
                        (*version)
                            .try_into()
                            .expect("Import version in Local table import is bigger than u32."),
                    ),
                    _ => None,
                };

            imports.push(Import {
                name,
                version: Some(version),
                max_len,
            })
        }

        Ok(imports)
    }

    fn load_shared_table(&mut self, table: IonValue) -> Result<(), IonParserError> {
        trace!("Loading Shared Table");

        let table = self.get_parsed_struct_hashmap(&table)?;

        let name = table
            .get(self.get_symbol_name_by_type(SystemSymbolIds::Name))
            .ok_or(IonParserError::SharedTableDefinitionWithoutName)?;

        let version = table.get(self.get_symbol_name_by_type(SystemSymbolIds::Version));

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

        let symbols = self.get_symbols_string(&table);

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
        let mut symbols_names = Vec::new();

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
