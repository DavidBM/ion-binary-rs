use crate::binary_parser::IonBinaryParser;
use crate::binary_parser_types::*;
use crate::ion_parser_types::*;
use crate::symbol_table::*;
use bigdecimal::BigDecimal;
use chrono::{naive::NaiveDate, DateTime, FixedOffset};
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
            ValueType::Timestamp => self.consume_timestamp(&value_header)?,
            ValueType::Null => (IonValue::Null, 1),
            ValueType::Float => self.consume_float(&value_header)?,
            ValueType::Decimal => self.consume_decimal(&value_header)?,
            ValueType::Clob => self.consume_clob(&value_header)?,
            ValueType::Blob => self.consume_blob(&value_header)?,
            ValueType::SExpr => self.consume_sexpr(&value_header)?,
            ValueType::Reserved => return Err(IonParserError::Unimplemented),
        };

        // We increase the consumed bytes count as we must count for the already consumed ValueHeader
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

        let value = self.parser.consume_uint(length)?;

        let mut value: i64 = value
            .try_into()
            .map_err(|_| IonParserError::IntegerTooBig)?;

        if negative {
            value = -value;
        }

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

            consumed_bytes += value.1;
            consumed_bytes += key.1;

            println!("Struct field -> Key: {:?}, Values: {:?}", key, values);

            let key = match self.context.get_symbol_by_id(
                key.0
                    .try_into()
                    .map_err(|_| IonParserError::SymbolIdTooBigForUsize)?,
            ) {
                Some(Symbol::Symbol(text)) => text.clone(),
                _ => return Err(IonParserError::SymbolNotFoundInTable),
            };

            values.insert(key, value.0);
        }

        if length.checked_sub(consumed_bytes).is_none() {
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

            consumed_bytes += value.1;
            values.push(value.0);
        }

        if length.checked_sub(consumed_bytes).is_none() {
            return Err(IonParserError::ListLengthWasTooShort);
        }

        Ok((IonValue::List(values), total))
    }

    fn consume_sexpr(&mut self, header: &ValueHeader) -> Result<(IonValue, usize), IonParserError> {
        trace!("Consuming SExpr");

        let list_value = self.consume_list(header)?;

        if let (IonValue::List(list), len) = list_value {
            Ok((IonValue::SExpr(list), len))
        } else {
            Err(IonParserError::DidNotGetAListConsumingAListThisIsABug)
        }
    }

    fn consume_symbol(
        &mut self,
        header: &ValueHeader,
    ) -> Result<(IonValue, usize), IonParserError> {
        trace!("Consuming Symbol");

        let (length, _, total_consumed_bytes) = self.consume_value_len(header)?;

        let symbol_id = self.parser.consume_uint(length)?;

        let symbol = self.context.get_symbol_by_id(
            symbol_id
                .try_into()
                .map_err(|_| IonParserError::SymbolIdTooBigForUsize)?,
        );

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

        let (length, mut consumed_bytes, _) = self.consume_value_len(header)?;

        let (offset, consumed) = self.parser.consume_varint()?;
        consumed_bytes += consumed;
        let (year, consumed) = self.parser.consume_varuint()?;
        consumed_bytes += consumed;

        let year: i32 = year
            .try_into()
            .map_err(|_| IonParserError::DateValueTooBig)?;

        let mut components = [0u32, 0, 0, 0, 0];

        for component in &mut components {
            if consumed_bytes >= length {
                break;
            }

            let (value, consumed) = self.parser.consume_varuint()?;
            consumed_bytes += consumed;
            *component = value
                .try_into()
                .map_err(|_| IonParserError::DateValueTooBig)?;
        }

        let [month, day, hour, minute, second] = components;

        let fraction_exponent: i32 = if consumed_bytes < length {
            let value = self.parser.consume_varint()?;
            consumed_bytes += value.1;
            value
                .0
                .try_into()
                .map_err(|_| IonParserError::DateValueTooBig)?
        } else {
            0
        };

        let fraction_coefficient: i32 = if (consumed_bytes) < length {
            let length: usize = length
                .try_into()
                .map_err(|_| IonParserError::DateValueTooBig)?;
            let remaining_bytes = length - consumed_bytes;
            let value = self.parser.consume_int(remaining_bytes)?;
            consumed_bytes += remaining_bytes;
            value
                .try_into()
                .map_err(|_| IonParserError::DateValueTooBig)?
        } else {
            0
        };

        let second_fraction: f64 =
            ((10f64.powi(fraction_exponent)) * fraction_coefficient as f64) * 1e9;

        let second_fraction =
            u32::try_from(second_fraction as u64).map_err(|_| IonParserError::DateValueTooBig)?;

        let datetime = NaiveDate::from_ymd(year, month, day).and_hms_nano(
            hour,
            minute,
            second,
            second_fraction,
        );

        let offset: i32 = offset
            .try_into()
            .map_err(|_| IonParserError::DateValueTooBig)?;

        let datetime = DateTime::<FixedOffset>::from_utc(datetime, FixedOffset::east(offset * 60));

        Ok((IonValue::DateTime(datetime), consumed_bytes))
    }

    fn consume_float(&mut self, header: &ValueHeader) -> Result<(IonValue, usize), IonParserError> {
        trace!("Consuming float");

        const FOUR_BYTES: usize = 4;
        const EIGHT_BYTES: usize = 8;

        Ok(match header.length {
            ValueLength::ShortLength(len) => {
                match len {
                    0 => (IonValue::Float32(0f32), 0),
                    4 => {
                        let mut buffer = [0u8; FOUR_BYTES];
                        self.parser.read_bytes(&mut buffer)?;
                        (IonValue::Float32(f32::from_be_bytes(buffer)), FOUR_BYTES)
                    }
                    8 => {
                        let mut buffer = [0u8; EIGHT_BYTES];
                        self.parser.read_bytes(&mut buffer)?;
                        (IonValue::Float64(f64::from_be_bytes(buffer)), EIGHT_BYTES)
                    },
                    _ => return Err(IonParserError::NotValidLengthFloat),
                }
            }, 
            ValueLength::NullValue =>  return Err(IonParserError::Unimplemented),
            ValueLength::LongLength => return Err(IonParserError::NotValidLengthFloat),
        })
    }

    fn consume_decimal(
        &mut self,
        header: &ValueHeader,
    ) -> Result<(IonValue, usize), IonParserError> {
        trace!("Consuming decimal");

        let (length, _, total) = self.consume_value_len(header)?;

        if length > 0 {
            let (exponent, consumed_bytes) = self.parser.consume_varint()?;
            let coefficient_size = length - consumed_bytes;

            let coefficient = if coefficient_size > 0 {
                self.parser.consume_int(coefficient_size)?
            } else {
                0
            };

            Ok((
                IonValue::Decimal(BigDecimal::new(coefficient.into(), -exponent)),
                total,
            ))
        } else {
            Ok((IonValue::Decimal(BigDecimal::new(0.into(), 0)), total))
        }
    }

    fn consume_clob(&mut self, header: &ValueHeader) -> Result<(IonValue, usize), IonParserError> {
        trace!("Consuming clob");

        let (length, _, total) = self.consume_value_len(header)?;
        let mut buffer = vec![0; length as usize];
        self.parser.read_bytes(&mut buffer)?;

        Ok((IonValue::Clob(buffer), total))
    }

    fn consume_blob(&mut self, header: &ValueHeader) -> Result<(IonValue, usize), IonParserError> {
        trace!("Consuming blob");

        let (length, _, total) = self.consume_value_len(header)?;
        let mut buffer = vec![0; length as usize];
        self.parser.read_bytes(&mut buffer)?;

        Ok((IonValue::Blob(buffer), total))
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
                Err(IonParserError::SharedTableAndLocalTableDeclarationIntTheSameAnnotation)
            }
            (true, false) => {
                self.load_shared_table(value.0)?;
                Ok((None, total_consumed_bytes))
            }
            (false, true) => {
                self.load_local_table(value.0)?;
                Ok((None, total_consumed_bytes))
            }
            (false, false) => Ok((
                Some(self.construct_raw_annotation(&symbols, value.0)?),
                total_consumed_bytes,
            )),
        }
    }

    fn consume_value_len(
        &mut self,
        header: &ValueHeader,
    ) -> Result<(usize, usize, usize), IonParserError> {
        let mut consumed_bytes: usize = 0;

        let length: usize = match header.length {
            ValueLength::LongLength => {
                let len = self.parser.consume_varuint()?;
                consumed_bytes += len.1;
                usize::try_from(len.0).map_err(|_| IonParserError::ValueLenTooBig)?
            }
            ValueLength::ShortLength(len) => len.into(),
            ValueLength::NullValue => return Err(IonParserError::NullAnnotationFound),
        };

        let total = consumed_bytes + length;

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
            .map_err(IonParserError::ErrorAddingCreatingLocal)?;

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
                        .map_err(|_| IonParserError::TableVersionTooBig)?,
                    _ => 1,
                };

            let max_len: Option<usize> =
                match value.get(self.get_symbol_name_by_type(SystemSymbolIds::MaxId)) {
                    Some(IonValue::Integer(version)) => Some(
                        (*version)
                            .try_into()
                            .map_err(|_| IonParserError::TableVersionTooBig)?,
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
                .map_err(|_| IonParserError::TableVersionTooBig)?
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
            .map_err(IonParserError::ErrorAddingSharedTableToContext)?;

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

        for symbol in symbols.iter() {
            let name = self.get_symbol_name(*symbol)?;
            symbols_names.push(name);
        }

        Ok(IonValue::Annotation((symbols_names, Box::new(value))))
    }

    fn contains_system_symbol(&self, symbols: &[u64], symbol: SystemSymbolIds) -> bool {
        symbols.iter().any(|&s| s == symbol as u64)
    }

    fn get_symbol_name(&self, symbol_id: u64) -> Result<String, IonParserError> {
        match self.context.get_symbol_by_id(symbol_id.try_into().map_err(|_| IonParserError::SymbolIdTooBigForUsize)?) {
            Some(Symbol::Symbol(name)) => Ok(name.clone()),
            Some(Symbol::Dummy) | None => Err(IonParserError::SymbolIdNotDefined),
        }
    }
}