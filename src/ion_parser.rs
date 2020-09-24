use crate::binary_parser::IonBinaryParser;
use crate::binary_parser_types::*;
use crate::ion_parser_types::*;
use crate::symbol_table::*;
use bigdecimal::BigDecimal;
use chrono::{naive::NaiveDate, DateTime, FixedOffset, Utc};
use log::trace;
use num_bigint::{BigInt, BigUint};
use num_traits::ops::checked::CheckedSub;
use std::convert::{TryFrom, TryInto};
use std::{collections::HashMap, io::Read};

/// The library entry point.
///
/// In order to use it, call the new method and then the "consume_all" method.
///
/// ### Example
///
#[derive(Debug)]
pub struct IonParser<T: Read> {
    parser: IonBinaryParser<T>,
    context: SymbolContext,
}

pub type ConsumerResult = Result<(IonValue, usize), IonParserError>;

impl<T: Read> IonParser<T> {
    /// Creates a new parser. It accepts anything that implements the trait
    /// [Read Trait](https://doc.rust-lang.org/stable/std/io/trait.Read.html)
    pub fn new(reader: T) -> IonParser<T> {
        IonParser {
            parser: IonBinaryParser::new(reader),
            context: SymbolContext::new(),
        }
    }

    /// Allows to set up shared tables in order to define symbols that are not in the
    /// binary blob. This is useful when decoding binaries that depend of huge tables
    /// that are expected to exist in the client and not to be sent in the ion binary.
    pub fn with_shared_table(
        &mut self,
        name: String,
        version: u32,
        symbols: &[String],
    ) -> Result<(), SymbolContextError> {
        let symbols: Vec<Symbol> = symbols
            .iter()
            .map(|s| Symbol::Symbol(s.to_string()))
            .collect();

        self.context.add_shared_table(name, version, &symbols)
    }

    /// Consumes all the IonValues in the binary blob and returns an array with them.
    pub fn consume_all(&mut self) -> Result<Vec<IonValue>, IonParserError> {
        let mut values = vec![];

        loop {
            match self.consume_value() {
                Err(IonParserError::BinaryError(ParsingError::NoDataToRead)) => break,
                Ok((value, _)) => values.push(value),
                Err(e) => return Err(e),
            }
        }

        Ok(values)
    }

    /// Consumes **one** IonValue and stops. This function will automatically process
    /// NOP Padding, Shared Tables and Local Tables, automatically continuing in case
    /// that any of them are found.
    pub fn consume_value(&mut self) -> ConsumerResult {
        let value_header = self.parser.consume_value_header()?;

        let mut value = self.consume_value_body(&value_header)?;

        let already_consumed_value_header = 1;
        value.1 += already_consumed_value_header;

        Ok(value)
    }

    fn consume_value_body(&mut self, value_header: &ValueHeader) -> ConsumerResult {
        match value_header.r#type {
            ValueType::Bool => Ok(self.consume_bool(&value_header)?),
            ValueType::Annotation => match self.consume_annotation(value_header)? {
                (Some(annotation), consumed_bytes) => Ok((annotation, consumed_bytes)),
                (None, consumed_bytes) => {
                    let value = self.consume_value()?;
                    Ok((value.0, value.1 + consumed_bytes))
                }
            },
            ValueType::Struct => Ok(self.consume_struct(value_header)?),
            ValueType::List => Ok(self.consume_list(value_header)?),
            ValueType::Symbol => Ok(self.consume_symbol(value_header)?),
            ValueType::PositiveInt => Ok(self.consume_int(value_header, false)?),
            ValueType::NegativeInt => Ok(self.consume_int(value_header, true)?),
            ValueType::String => Ok(self.consume_string(value_header)?),
            ValueType::Timestamp => Ok(self.consume_timestamp(value_header)?),
            ValueType::Null => Ok((IonValue::Null(NullIonValue::Null), 0)),
            ValueType::Nop => {
                let consumed_bytes = self.consume_nop(value_header)?;
                let value = self.consume_value()?;
                Ok((value.0, value.1 + consumed_bytes))
            }
            ValueType::Float => Ok(self.consume_float(value_header)?),
            ValueType::Decimal => Ok(self.consume_decimal(value_header)?),
            ValueType::Clob => Ok(self.consume_clob(value_header)?),
            ValueType::Blob => Ok(self.consume_blob(value_header)?),
            ValueType::SExpr => Ok(self.consume_sexpr(value_header)?),
            ValueType::Reserved => Err(IonParserError::InvalidReservedTypeDescriptor),
        }
    }

    fn consume_nop(&mut self, header: &ValueHeader) -> Result<usize, IonParserError> {
        trace!("Consuming Nop Padding");
        let (length, _, total) = self.consume_value_len(header)?;

        trace!("Nop Padding with length {}", length);

        if length > 0 {
            let mut buffer = vec![0; length as usize];
            self.parser.read_bytes(&mut buffer)?;
        }

        Ok(total)
    }

    fn consume_bool(&mut self, header: &ValueHeader) -> ConsumerResult {
        Ok(match &header.length {
            ValueLength::NullValue => (IonValue::Null(NullIonValue::Bool), 0),
            ValueLength::ShortLength(1) => (IonValue::Bool(true), 0),
            ValueLength::ShortLength(0) => (IonValue::Bool(false), 0),
            _ => return Err(IonParserError::InvalidBoolLength(header.length.clone())),
        })
    }

    fn consume_string(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming String");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::String), 0));
        }

        if let ValueLength::ShortLength(0) = header.length {
            return Ok((IonValue::String("".into()), 0));
        }

        let (length, _, total) = self.consume_value_len(header)?;
        let mut buffer = vec![0; length as usize];
        self.parser.read_bytes(&mut buffer)?;

        let text = match String::from_utf8(buffer) {
            Ok(text) => text,
            Err(_) => return Err(IonParserError::NonUtf8String),
        };

        Ok((IonValue::String(text), total))
    }

    fn consume_int(&mut self, header: &ValueHeader, negative: bool) -> ConsumerResult {
        trace!("Consuming Integer");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::Integer), 0));
        }

        if let ValueLength::ShortLength(0) = header.length {
            return Ok((IonValue::Integer(0), 0));
        }

        let (length, _, total) = self.consume_value_len(header)?;
        let value = self.parser.consume_uint(length)?;

        let value = match i64::try_from(&value) {
            Ok(mut value) => {
                if negative {
                    value = -value;
                }

                IonValue::Integer(value)
            }
            Err(_) => {
                let mut value = BigInt::from(value);

                if negative {
                    value = -value;
                }

                IonValue::BigInteger(value)
            }
        };

        Ok((value, total))
    }

    fn consume_struct(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming Struct");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::Struct), 0));
        }

        let (length, _, total) = self.consume_value_len_for_struct(header)?;
        let mut consumed_bytes = 0;
        let mut values: HashMap<String, IonValue> = HashMap::new();

        while length - consumed_bytes > 0 {
            let key = self.parser.consume_varuint()?;
            consumed_bytes += key.1;

            let key = match self.context.get_symbol_by_id(
                key.0
                    .try_into()
                    .map_err(|_| IonParserError::SymbolIdTooBig)?,
            ) {
                Some(Symbol::Symbol(text)) => text.clone(),
                _ => return Err(IonParserError::SymbolNotFoundInTable),
            };

            trace!("Struct key field: {:?}", key);

            let value_header = self.parser.consume_value_header()?;

            consumed_bytes += 1;

            if let ValueType::Nop = value_header.r#type {
                let consumed = self.consume_nop(&value_header)?;
                trace!("Found NOP Padding in Struct of {:} bytes", consumed + 1);
                consumed_bytes += consumed;
                continue;
            }

            let value = self.consume_value_body(&value_header)?;

            consumed_bytes += value.1;

            trace!("Struct field -> Key: {:?}, Value: {:?}", key, value.0);

            values.insert(key, value.0);
        }

        if length.checked_sub(consumed_bytes).is_none() {
            return Err(IonParserError::ListLengthWasTooShort);
        }

        trace!("End consuming struct");

        Ok((IonValue::Struct(values), total))
    }

    fn consume_list(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming List");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::List), 0));
        }

        let (length, _, total) = self.consume_value_len(header)?;
        let mut consumed_bytes = 0;
        let mut values = vec![];

        while length - consumed_bytes > 0 {
            let value_header = self.parser.consume_value_header()?;

            consumed_bytes += 1;

            if let ValueType::Nop = value_header.r#type {
                let consumed = self.consume_nop(&value_header)?;
                trace!("Found NOP Padding in List of {:} bytes", consumed + 1);
                consumed_bytes += consumed;
                continue;
            }

            let value = self.consume_value_body(&value_header)?;

            consumed_bytes += value.1;
            values.push(value.0);
        }

        if length.checked_sub(consumed_bytes).is_none() {
            return Err(IonParserError::ListLengthWasTooShort);
        }

        trace!("End consuming list with {:}", values.len());

        Ok((IonValue::List(values), total))
    }

    fn consume_sexpr(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming SExpr");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::SExpr), 0));
        }

        let list_value = self.consume_list(header)?;

        if let (IonValue::List(list), len) = list_value {
            Ok((IonValue::SExpr(list), len))
        } else {
            Err(IonParserError::DidNotGetAListConsumingAListThisIsABug)
        }
    }

    fn consume_symbol(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming Symbol");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::Symbol), 0));
        }

        let (length, _, total_consumed_bytes) = self.consume_value_len(header)?;

        let symbol_id = if length == 0 {
            BigUint::from(SystemSymbolIds::Zero as u8)
        } else {
            self.parser.consume_uint(length)?
        };

        let symbol = self.context.get_symbol_by_id(
            symbol_id
                .try_into()
                .map_err(|_| IonParserError::SymbolIdTooBig)?,
        );

        let text = match symbol {
            Some(Symbol::Symbol(text)) => text.clone(),
            _ => return Err(IonParserError::SymbolNotFoundInTable),
        };

        Ok((IonValue::Symbol(text), total_consumed_bytes))
    }

    fn consume_timestamp(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming Timestamp");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::DateTime), 0));
        }

        let (length, mut consumed_bytes, _) = self.consume_value_len(header)?;

        let (offset, consumed) = self.parser.consume_varint()?;
        consumed_bytes += consumed;
        let (year, consumed) = self.parser.consume_varuint()?;
        consumed_bytes += consumed;

        let year: i32 = year
            .try_into()
            .map_err(|_| IonParserError::DateValueTooBig)?;

        let mut components = [1u32, 1, 0, 0, 0];

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

        let fraction_coefficient: i64 = if (consumed_bytes) < length {
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

        let datetime = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or(IonParserError::InvalidDate(
                year,
                month,
                day,
                hour,
                minute,
                second,
                second_fraction,
            ))?
            .and_hms_nano_opt(hour, minute, second, second_fraction)
            .ok_or(IonParserError::InvalidDate(
                year,
                month,
                day,
                hour,
                minute,
                second,
                second_fraction,
            ))?;

        let offset: i32 = offset
            .try_into()
            .map_err(|_| IonParserError::DateValueTooBig)?;

        let offset = FixedOffset::east_opt(offset * 60).ok_or_else(|| {
            IonParserError::InvalidDate(year, month, day, hour, minute, second, second_fraction)
        })?;

        let datetime = DateTime::<Utc>::from_utc(datetime, Utc);

        let datetime = datetime.with_timezone(&offset);

        Ok((IonValue::DateTime(datetime), consumed_bytes))
    }

    fn consume_float(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming float");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::Float), 0));
        }

        const FOUR_BYTES: usize = 4;
        const EIGHT_BYTES: usize = 8;

        Ok(match header.length {
            ValueLength::ShortLength(len) => match len {
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
                }
                _ => return Err(IonParserError::NotValidLengthFloat),
            },
            ValueLength::NullValue => return Ok((IonValue::Null(NullIonValue::Float), 1)),
            ValueLength::LongLength => return Err(IonParserError::NotValidLengthFloat),
        })
    }

    fn consume_decimal(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming decimal");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::Decimal), 0));
        }

        if let ValueLength::ShortLength(0) = header.length {
            return Ok((IonValue::Decimal(BigDecimal::from(0)), 0));
        }

        let (length, _, total) = self.consume_value_len(header)?;

        let (exponent, consumed_bytes) = self.parser.consume_varint()?;
        let coefficient_size = length - consumed_bytes;

        let coefficient = if coefficient_size > 0 {
            self.parser.consume_int(coefficient_size)?
        } else {
            BigInt::from(0)
        };

        let exponent: i64 = exponent
            .try_into()
            .map_err(|_| IonParserError::DecimalExponentTooBig)?;

        Ok((
            IonValue::Decimal(BigDecimal::new(coefficient, -exponent)),
            total,
        ))
    }

    fn consume_clob(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming clob");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::Clob), 0));
        }

        if let ValueLength::ShortLength(0) = header.length {
            return Ok((IonValue::Clob(Vec::new()), 0));
        }

        let (length, _, total) = self.consume_value_len(header)?;
        let mut buffer = vec![0; length as usize];
        self.parser.read_bytes(&mut buffer)?;

        Ok((IonValue::Clob(buffer), total))
    }

    fn consume_blob(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming blob");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::Blob), 0));
        }

        if let ValueLength::ShortLength(0) = header.length {
            return Ok((IonValue::Blob(Vec::new()), 0));
        }

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

        if self.is_value_null(header) {
            return Err(IonParserError::NullAnnotationFound);
        }

        let (_, _, total_consumed_bytes) = self.consume_value_len(header)?;
        let mut remaining_annot_bytes = self.parser.consume_varuint()?.0;
        let mut symbols: Vec<usize> = Vec::new();

        while remaining_annot_bytes > BigUint::from(0u8) {
            let (annot, consumed_bytes) = self.parser.consume_varuint()?;

            let id_u64 = annot
                .try_into()
                .map_err(|_| IonParserError::SymbolIdTooBig)?;

            symbols.push(id_u64);

            remaining_annot_bytes =
                match BigUint::checked_sub(&remaining_annot_bytes, &BigUint::from(consumed_bytes)) {
                    Some(result) => result,
                    None => return Err(IonParserError::BadFormatLengthFound),
                }
        }

        trace!("Annotations found: {:?}", symbols);

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

    fn is_value_null(&self, header: &ValueHeader) -> bool {
        header.length == ValueLength::NullValue
    }

    fn consume_value_len(
        &mut self,
        header: &ValueHeader,
    ) -> Result<(usize, usize, usize), IonParserError> {
        let mut consumed_bytes: usize = 0;
        let null_length = 15;

        let length: usize = match header.length {
            ValueLength::LongLength => {
                let len = self.parser.consume_varuint()?;
                consumed_bytes += len.1;
                usize::try_from(len.0).map_err(|_| IonParserError::ValueLenTooBig)?
            }
            ValueLength::ShortLength(len) => len.into(),
            ValueLength::NullValue => null_length,
        };

        let total = consumed_bytes + length;

        Ok((length, consumed_bytes, total))
    }

    fn consume_value_len_for_struct(
        &mut self,
        header: &ValueHeader,
    ) -> Result<(usize, usize, usize), IonParserError> {
        let mut consumed_bytes: usize = 0;
        let null_length = 15;

        let length: usize = match header.length {
            ValueLength::LongLength | ValueLength::ShortLength(1) => {
                let len = self.parser.consume_varuint()?;
                consumed_bytes += len.1;
                usize::try_from(len.0).map_err(|_| IonParserError::ValueLenTooBig)?
            }
            ValueLength::ShortLength(len) => len.into(),
            ValueLength::NullValue => null_length,
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
        symbols: &[usize],
        value: IonValue,
    ) -> Result<IonValue, IonParserError> {
        let mut symbols_names = Vec::new();

        for symbol in symbols.iter() {
            let name = self.get_symbol_name(*symbol)?;
            symbols_names.push(name);
        }

        Ok(IonValue::Annotation(symbols_names, Box::new(value)))
    }

    fn contains_system_symbol(&self, symbols: &[usize], symbol: SystemSymbolIds) -> bool {
        symbols.iter().any(|&s| s == symbol as usize)
    }

    fn get_symbol_name(&self, symbol_id: usize) -> Result<String, IonParserError> {
        match self.context.get_symbol_by_id(symbol_id) {
            Some(Symbol::Symbol(name)) => Ok(name.clone()),
            Some(Symbol::Dummy) | None => Err(IonParserError::SymbolIdNotDefined),
        }
    }
}
