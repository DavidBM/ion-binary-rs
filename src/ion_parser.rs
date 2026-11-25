use crate::binary_parser::IonBinaryParser;
use crate::binary_parser_types::*;
use crate::ion_parser_types::*;
use crate::symbol_table::*;
use bigdecimal::BigDecimal;
use chrono::{naive::NaiveDate, FixedOffset, TimeZone, Utc};
use log::trace;
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::ops::checked::CheckedSub;
use std::convert::{TryFrom, TryInto};
use std::{collections::HashMap, io::Read};

/// In order to use it call the new method and then the "consume_all" method.
///
/// ### Example
///
/// ```rust,no_run
///
/// use ion_binary_rs::IonParser;
///
/// // This is the response from Amazon's QLDB introduction example using Rusoto
/// let ion_test = b"\xe0\x01\0\xea\xee\xa6\x81\x83\xde\xa2\x87\xbe\x9f\x83VIN\x84Type\x84Year\x84Make\x85Model\x85Color\xde\xb9\x8a\x8e\x911C4RJFAG0FC625797\x8b\x85Sedan\x8c\"\x07\xe3\x8d\x88Mercedes\x8e\x87CLK 350\x8f\x85White";
///
/// let mut parser = IonParser::new(&ion_test[..]);
///
/// println!("Decoded Ion: {:?}", parser.consume_all().unwrap())
/// // Decoded Ion: [Struct({"Color": String("White"), "Year": Integer(2019), "VIN": String("1C4RJFAG0FC625797"), "Make": String("Mercedes"), "Model": String("CLK 350"), "Type": String("Sedan")})]
///
/// ```
#[derive(Debug)]
pub struct IonParser<T: Read> {
    parser: IonBinaryParser<T>,
    context: SymbolContext,
}

pub type ConsumerResult = Result<(IonValue, usize), IonParserError>;

impl<T: Read> IonParser<T> {
    /// Creates a new parser. It accepts anything that implements the trait
    /// [Read Trait](https://doc.rust-lang.org/stable/std/io/trait.Read.html)
    #[inline]
    pub fn new(reader: T) -> IonParser<T> {
        IonParser {
            parser: IonBinaryParser::new(reader),
            context: SymbolContext::new(),
        }
    }

    /// Allows to set up shared tables in order to define symbols that are not in the
    /// binary blob. This is useful when decoding binaries that depend of huge tables
    /// that are expected to exist in the client and not to be sent in the ion binary.
    #[inline]
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
    #[inline]
    pub fn consume_all(&mut self) -> Result<Vec<IonValue>, IonParserError> {
        let mut values = Vec::new();
        while let Some((value, _)) = self.next_value()? {
            values.push(value);
        }
        Ok(values)
    }

    /// Consumes one Ion value. Version markers, NOP padding, and top-level
    /// symbol tables are processed without yielding a value.
    #[inline]
    pub fn consume_value(&mut self) -> ConsumerResult {
        self.next_value()?.ok_or(ParsingError::NoDataToRead.into())
    }

    fn next_value(&mut self) -> Result<Option<(IonValue, usize)>, IonParserError> {
        let mut skipped = 0usize;
        loop {
            let header = match self.parser.consume_value_header(0) {
                Ok(header) => header,
                Err(ParsingError::NoDataToRead) => return Ok(None),
                Err(error) => return Err(error.into()),
            };
            if self.parser.take_symbol_reset() {
                self.context.reset_local_table();
            }

            let consumed = if header.is_nop() {
                self.consume_nop(&header)?
            } else if header.get_type() == ValueType::Annotation {
                let (symbols, value, consumed) = self.consume_annotation(&header, 0)?;
                match (symbols[0], value) {
                    (id, value @ IonValue::Struct(_))
                        if id == SystemSymbolIds::IonSymbolTable as usize =>
                    {
                        self.load_local_table(value)?;
                    }
                    (id, IonValue::Null(NullIonValue::Struct))
                        if id == SystemSymbolIds::IonSymbolTable as usize => {}
                    (id, value @ IonValue::Struct(_))
                        if id == SystemSymbolIds::IonSharedSymbolTable as usize =>
                    {
                        self.load_shared_table(value)?;
                    }
                    (_, value) => {
                        let annotated = self.construct_raw_annotation(&symbols, value)?;
                        return Ok(Some((annotated, Self::with_header(skipped, consumed)?)));
                    }
                }
                consumed
            } else {
                let (value, consumed) = self.consume_value_body(&header, 0)?;
                return Ok(Some((value, Self::with_header(skipped, consumed)?)));
            };
            skipped = Self::with_header(skipped, consumed)?;
        }
    }

    fn with_header(skipped: usize, consumed: usize) -> Result<usize, IonParserError> {
        skipped
            .checked_add(consumed)
            .and_then(|length| length.checked_add(1))
            .ok_or(IonParserError::ValueLenTooBig)
    }

    #[inline]
    fn consume_value_body(
        &mut self,
        value_header: &ValueHeader,
        nested_level: u64,
    ) -> ConsumerResult {
        match value_header.get_type() {
            ValueType::Bool => self.consume_bool(value_header),
            ValueType::Annotation => {
                let (symbols, value, consumed) =
                    self.consume_annotation(value_header, nested_level)?;
                Ok((self.construct_raw_annotation(&symbols, value)?, consumed))
            }
            ValueType::Struct => self.consume_struct(value_header, nested_level, false),
            ValueType::List => self.consume_list(value_header, nested_level),
            ValueType::Symbol => self.consume_symbol(value_header),
            ValueType::PositiveInt => self.consume_int(value_header, false),
            ValueType::NegativeInt => self.consume_int(value_header, true),
            ValueType::String => self.consume_string(value_header),
            ValueType::Timestamp => self.consume_timestamp(value_header),
            ValueType::Null => Ok((IonValue::Null(NullIonValue::Null), 0)),
            ValueType::Float => self.consume_float(value_header),
            ValueType::Decimal => self.consume_decimal(value_header),
            ValueType::Clob => self.consume_clob(value_header),
            ValueType::Blob => self.consume_blob(value_header),
            ValueType::SExpr => self.consume_sexpr(value_header, nested_level),
            ValueType::Reserved => Err(IonParserError::InvalidReservedTypeDescriptor),
        }
    }

    #[inline]
    fn consume_nop(&mut self, header: &ValueHeader) -> Result<usize, IonParserError> {
        trace!("Consuming Nop Padding");
        let (length, _, total) = self.consume_value_len(header)?;

        trace!("Nop Padding with length {}", length);

        self.parser.skip_bytes(length)?;

        Ok(total)
    }

    #[inline]
    fn consume_bool(&mut self, header: &ValueHeader) -> ConsumerResult {
        if header.is_len_null_value() {
            return Ok((IonValue::Null(NullIonValue::Bool), 0));
        }

        let len = header.get_len();

        if len == 1 {
            return Ok((IonValue::Bool(true), 0));
        }

        if len == 0 {
            return Ok((IonValue::Bool(false), 0));
        }

        Err(IonParserError::InvalidBoolLength(len))
    }

    #[inline]
    fn consume_string(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming String");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::String), 0));
        }

        if header.get_len() == 0 {
            return Ok((IonValue::String("".into()), 0));
        }

        let (length, _, total) = self.consume_value_len(header)?;
        let buffer = self.parser.read_value_bytes(length)?;

        let text = match String::from_utf8(buffer) {
            Ok(text) => text,
            Err(_) => return Err(IonParserError::NonUtf8String),
        };

        Ok((IonValue::String(text), total))
    }

    #[inline]
    fn consume_int(&mut self, header: &ValueHeader, negative: bool) -> ConsumerResult {
        trace!("Consuming Integer");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::Integer), 0));
        }

        if header.get_len() == 0 {
            if negative {
                return Err(IonParserError::InvalidNegativeInt);
            } else {
                return Ok((IonValue::Integer(0), 0));
            }
        }

        let (length, _, total) = self.consume_value_len(header)?;
        let value = self.parser.consume_uint(length)?;

        // i64::MIN as u64 is not a "correct" transformation. It just binary cast
        // the value to a u64, so the most negative number in i64 becomes a huge
        // positive one un u64. We do this here knowingly as it is exactly what we
        // want in order to avoid a .clone().
        if negative && value == BigUint::from(i64::MIN as u64) {
            return Ok((IonValue::Integer(i64::MIN), total));
        }

        let value = match i64::try_from(&value) {
            Ok(mut value) => {
                if negative {
                    if value == 0 {
                        return Err(IonParserError::InvalidNegativeInt);
                    } else {
                        value = -value;
                    }
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

    #[inline]
    fn consume_struct(
        &mut self,
        header: &ValueHeader,
        nested_level: u64,
        local_symbol_table: bool,
    ) -> ConsumerResult {
        trace!("Consuming Struct");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::Struct), 0));
        }

        let (length, _, total) = self.consume_value_len_for_struct(header)?;
        let mut consumed_bytes = 0;
        let mut values: HashMap<String, IonValue> = HashMap::new();

        while consumed_bytes < length {
            let (key_id, key_bytes) = self.parser.consume_varuint()?;
            consumed_bytes += key_bytes;
            if consumed_bytes >= length {
                return Err(IonParserError::ListLengthWasTooShort);
            }

            let value_header = self.parser.consume_value_header(nested_level + 1)?;
            let (key, value_len) = if value_header.is_nop() {
                (None, self.consume_nop(&value_header)?)
            } else {
                let key_id = key_id
                    .try_into()
                    .map_err(|_| IonParserError::SymbolIdTooBig)?;
                let key = match self.context.get_symbol_by_id(key_id) {
                    Some(Symbol::Symbol(text)) => text.clone(),
                    _ => return Err(IonParserError::SymbolNotFoundInTable),
                };
                if local_symbol_table
                    && matches!(key.as_str(), "imports" | "symbols")
                    && values.contains_key(&key)
                {
                    return Err(IonParserError::DuplicateLocalSymbolTableField(key));
                }
                let (value, value_len) =
                    self.consume_value_body(&value_header, nested_level + 1)?;
                (Some((key, value)), value_len)
            };
            consumed_bytes = Self::with_header(consumed_bytes, value_len)?;
            if consumed_bytes > length {
                return Err(IonParserError::ListLengthWasTooShort);
            }
            if let Some((key, value)) = key {
                values.insert(key, value);
            }
        }

        trace!("End consuming struct");

        Ok((IonValue::Struct(values), total))
    }

    #[inline]
    fn consume_list(&mut self, header: &ValueHeader, nested_level: u64) -> ConsumerResult {
        trace!("Consuming List");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::List), 0));
        }

        let (length, _, total) = self.consume_value_len(header)?;
        let mut consumed_bytes = 0;
        let mut values = vec![];

        while consumed_bytes < length {
            let value_header = self.parser.consume_value_header(nested_level + 1)?;
            let value_len = if value_header.is_nop() {
                self.consume_nop(&value_header)?
            } else {
                let (value, value_len) =
                    self.consume_value_body(&value_header, nested_level + 1)?;
                values.push(value);
                value_len
            };
            consumed_bytes = Self::with_header(consumed_bytes, value_len)?;
            if consumed_bytes > length {
                return Err(IonParserError::ListLengthWasTooShort);
            }
        }

        trace!("End consuming list with {:}", values.len());

        Ok((IonValue::List(values), total))
    }

    #[inline]
    fn consume_sexpr(&mut self, header: &ValueHeader, nested_level: u64) -> ConsumerResult {
        trace!("Consuming SExpr");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::SExpr), 0));
        }

        let list_value = self.consume_list(header, nested_level)?;

        if let (IonValue::List(list), len) = list_value {
            Ok((IonValue::SExpr(list), len))
        } else {
            Err(IonParserError::DidNotGetAListConsumingAListThisIsABug)
        }
    }

    #[inline]
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

    #[inline]
    fn consume_timestamp(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming Timestamp");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::DateTime), 0));
        }

        let (length, _, total) = self.consume_value_len(header)?;

        let (offset, mut consumed_bytes) = self.parser.consume_varint()?;
        let (year, consumed) = self.parser.consume_varuint()?;
        consumed_bytes += consumed;

        let year: i32 = year
            .try_into()
            .map_err(|_| IonParserError::DateValueTooBig)?;

        let mut components = [1u32, 1, 0, 0, 0];
        let mut component_counter = 0;

        for component in &mut components {
            if consumed_bytes >= length {
                break;
            }

            let (value, consumed) = self.parser.consume_varuint()?;
            consumed_bytes += consumed;
            *component = value
                .try_into()
                .map_err(|_| IonParserError::DateValueTooBig)?;
            component_counter += 1;
        }

        if component_counter == 3 {
            return Err(IonParserError::DateWithHourButNoMinutes);
        }

        let [month, day, hour, minute, second] = components;

        if consumed_bytes > length {
            return Err(IonParserError::DateValueTooBig);
        }
        let second_fraction = if consumed_bytes < length {
            let (exponent, exponent_len) = self.parser.consume_varint()?;
            let exponent: i64 = exponent
                .try_into()
                .map_err(|_| IonParserError::DateValueTooBig)?;
            let coefficient_len = (length - consumed_bytes)
                .checked_sub(exponent_len)
                .ok_or(IonParserError::DateValueTooBig)?;
            if coefficient_len == 0 {
                0
            } else {
                let coefficient = self.parser.consume_int(coefficient_len)?;
                Self::fraction_to_nanoseconds(coefficient, exponent)?
            }
        } else {
            0
        };

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

        let offset = offset
            .checked_mul(60)
            .and_then(FixedOffset::east_opt)
            .ok_or(IonParserError::InvalidDate(
                year,
                month,
                day,
                hour,
                minute,
                second,
                second_fraction,
            ))?;

        let datetime = Utc.from_utc_datetime(&datetime);

        let datetime = datetime.with_timezone(&offset);

        Ok((IonValue::DateTime(datetime), total))
    }

    fn fraction_to_nanoseconds(coefficient: BigInt, exponent: i64) -> Result<u32, IonParserError> {
        match coefficient.sign() {
            Sign::NoSign => return Ok(0),
            Sign::Minus => return Err(IonParserError::DateNegativeSecondFraction),
            Sign::Plus => {}
        }
        if exponent >= 0 {
            return Err(IonParserError::DateSecondFractionOverflow);
        }
        let scale = exponent.unsigned_abs();
        let digits = coefficient.magnitude().to_radix_be(10);
        let leading_zeroes = scale
            .checked_sub(digits.len() as u64)
            .ok_or(IonParserError::DateSecondFractionOverflow)?;
        if leading_zeroes >= 9 {
            return Ok(0);
        }
        let significant_places = (9 - leading_zeroes) as usize;
        // The native value model uses Chrono. Keep its nine places exactly,
        // truncating any remaining digits without floating-point rounding.
        Ok(digits
            .into_iter()
            .chain(std::iter::repeat(0))
            .take(significant_places)
            .fold(0, |value, digit| value * 10 + u32::from(digit)))
    }

    #[inline]
    fn consume_float(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming float");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::Float), 0));
        }

        const FOUR_BYTES: usize = 4;
        const EIGHT_BYTES: usize = 8;

        if header.is_len_null_value() {
            return Ok((IonValue::Null(NullIonValue::Float), 1));
        }

        if header.is_len_long_len() {
            return Err(IonParserError::NotValidLengthFloat);
        }

        let len = header.get_len();

        Ok(match len {
            0 => (IonValue::Float(0f64), 0),
            4 => {
                let mut buffer = [0u8; FOUR_BYTES];
                self.parser.read_bytes(&mut buffer)?;
                (
                    IonValue::Float(f32::from_be_bytes(buffer).into()),
                    FOUR_BYTES,
                )
            }
            8 => {
                let mut buffer = [0u8; EIGHT_BYTES];
                self.parser.read_bytes(&mut buffer)?;
                (IonValue::Float(f64::from_be_bytes(buffer)), EIGHT_BYTES)
            }
            _ => return Err(IonParserError::NotValidLengthFloat),
        })
    }

    #[inline]
    fn consume_decimal(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming decimal");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::Decimal), 0));
        }

        if header.get_len() == 0 {
            return Ok((IonValue::Decimal(BigDecimal::from(0u8)), 0));
        }

        let (length, _, total) = self.consume_value_len(header)?;

        let (exponent, consumed_bytes) = self.parser.consume_varint()?;
        let coefficient_size = length
            .checked_sub(consumed_bytes)
            .ok_or(IonParserError::DecimalExponentTooBig)?;

        let coefficient = if coefficient_size > 0 {
            self.parser.consume_int(coefficient_size)?
        } else {
            BigInt::from(0u8)
        };

        let exponent: i64 = exponent
            .try_into()
            .map_err(|_| IonParserError::DecimalExponentTooBig)?;
        let scale = exponent
            .checked_neg()
            .ok_or(IonParserError::DecimalExponentTooBig)?;

        Ok((
            IonValue::Decimal(BigDecimal::new(coefficient, scale)),
            total,
        ))
    }

    #[inline]
    fn consume_clob(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming clob");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::Clob), 0));
        }

        if header.get_len() == 0 {
            return Ok((IonValue::Clob(Vec::new()), 0));
        }

        let (length, _, total) = self.consume_value_len(header)?;
        let buffer = self.parser.read_value_bytes(length)?;

        Ok((IonValue::Clob(buffer), total))
    }

    #[inline]
    fn consume_blob(&mut self, header: &ValueHeader) -> ConsumerResult {
        trace!("Consuming blob");

        if self.is_value_null(header) {
            return Ok((IonValue::Null(NullIonValue::Blob), 0));
        }

        if header.get_len() == 0 {
            return Ok((IonValue::Blob(Vec::new()), 0));
        }

        let (length, _, total) = self.consume_value_len(header)?;
        let buffer = self.parser.read_value_bytes(length)?;

        Ok((IonValue::Blob(buffer), total))
    }

    #[inline]
    fn consume_annotation(
        &mut self,
        header: &ValueHeader,
        nested_level: u64,
    ) -> Result<(Vec<usize>, IonValue, usize), IonParserError> {
        trace!("Consuming Annotation");

        if self.is_value_null(header) {
            return Err(IonParserError::NullAnnotationFound);
        }

        let (length, _, total_consumed_bytes) = self.consume_value_len(header)?;
        let (mut remaining_annot_bytes, mut consumed_bytes) = self.parser.consume_varuint()?;

        if remaining_annot_bytes == BigUint::from(0u8) {
            return Err(IonParserError::NullAnnotationFound);
        }

        let mut symbols: Vec<usize> = Vec::new();

        while remaining_annot_bytes > BigUint::from(0u8) {
            let (annot, last_consumed_bytes) = self.parser.consume_varuint()?;
            consumed_bytes += last_consumed_bytes;

            let id_u64 = annot
                .try_into()
                .map_err(|_| IonParserError::SymbolIdTooBig)?;

            symbols.push(id_u64);

            remaining_annot_bytes = match BigUint::checked_sub(
                &remaining_annot_bytes,
                &BigUint::from(last_consumed_bytes),
            ) {
                Some(result) => result,
                None => return Err(IonParserError::BadAnnotationLength),
            }
        }

        trace!("Annotations found: {:?}", symbols);

        let value_header = self.parser.consume_value_header(nested_level + 1)?;
        if value_header.is_nop() {
            return Err(IonParserError::BadAnnotationLength);
        }
        if value_header.get_type() == ValueType::Annotation {
            return Err(IonParserError::NestedAnnotations);
        }

        let (value, value_len) = if value_header.get_type() == ValueType::Struct {
            let local_symbol_table =
                nested_level == 0 && symbols[0] == SystemSymbolIds::IonSymbolTable as usize;
            self.consume_struct(&value_header, nested_level + 1, local_symbol_table)?
        } else {
            self.consume_value_body(&value_header, nested_level + 1)?
        };
        if Self::with_header(consumed_bytes, value_len)? != length {
            return Err(IonParserError::BadAnnotationLength);
        }
        Ok((symbols, value, total_consumed_bytes))
    }

    #[inline]
    fn is_value_null(&self, header: &ValueHeader) -> bool {
        header.is_len_null_value()
    }

    #[inline]
    fn consume_value_len(
        &mut self,
        header: &ValueHeader,
    ) -> Result<(usize, usize, usize), IonParserError> {
        let mut consumed_bytes: usize = 0;
        let null_length = 15;

        let length: usize = if header.is_len_long_len() {
            let len = self.parser.consume_varuint()?;
            consumed_bytes += len.1;
            usize::try_from(len.0).map_err(|_| IonParserError::ValueLenTooBig)?
        } else if header.is_len_null_value() {
            null_length
        } else {
            header.get_len().into()
        };

        let total = consumed_bytes
            .checked_add(length)
            .ok_or(IonParserError::ValueLenTooBig)?;

        Ok((length, consumed_bytes, total))
    }

    #[inline]
    fn consume_value_len_for_struct(
        &mut self,
        header: &ValueHeader,
    ) -> Result<(usize, usize, usize), IonParserError> {
        let mut consumed_bytes: usize = 0;
        let null_length = 15;

        let header_len = header.get_len();

        let length: usize = if header.is_len_long_len() || header_len == 1 {
            let len = self.parser.consume_varuint()?;
            if header_len == 1 && len.0 == BigUint::from(0u8) {
                return Err(IonParserError::EmptyOrderedStruct);
            }
            consumed_bytes += len.1;
            usize::try_from(len.0).map_err(|_| IonParserError::ValueLenTooBig)?
        } else if header.is_len_null_value() {
            null_length
        } else {
            header.get_len().into()
        };

        let total = consumed_bytes
            .checked_add(length)
            .ok_or(IonParserError::ValueLenTooBig)?;

        Ok((length, consumed_bytes, total))
    }

    #[inline]
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

    #[inline]
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

    #[inline]
    fn load_local_table(&mut self, table: IonValue) -> Result<(), IonParserError> {
        trace!("Loading Local Table");

        let table = self.get_parsed_struct_hashmap(&table)?;

        let symbols = self.get_symbols_string(table);

        let imports = table.get(self.get_symbol_name_by_type(SystemSymbolIds::Imports));

        let imports = match imports {
            Some(IonValue::Symbol(symbol))
                if symbol == self.get_symbol_name_by_type(SystemSymbolIds::IonSymbolTable) =>
            {
                return self
                    .context
                    .set_new_table_from_current(symbols)
                    .map_err(IonParserError::ErrorAddingCreatingLocal);
            }
            Some(IonValue::List(list)) => self.decode_imports(list)?,
            _ => Vec::new(),
        };

        self.context
            .set_new_table(&imports, &symbols)
            .map_err(IonParserError::ErrorAddingCreatingLocal)?;

        Ok(())
    }

    #[inline]
    fn decode_imports(&self, values: &[IonValue]) -> Result<Vec<Import>, IonParserError> {
        let mut imports = Vec::new();

        for value in values {
            let value = match value {
                IonValue::Struct(value) => value,
                _ => continue,
            };

            let name = match value.get(self.get_symbol_name_by_type(SystemSymbolIds::Name)) {
                Some(IonValue::String(name)) if !name.is_empty() => name.clone(),
                _ => continue,
            };

            let version: u32 =
                match value.get(self.get_symbol_name_by_type(SystemSymbolIds::Version)) {
                    Some(IonValue::Integer(version)) if *version > 0 => (*version)
                        .try_into()
                        .map_err(|_| IonParserError::TableVersionTooBig)?,
                    _ => 1,
                };

            let max_len: Option<usize> =
                match value.get(self.get_symbol_name_by_type(SystemSymbolIds::MaxId)) {
                    Some(IonValue::Integer(max_id)) if *max_id >= 0 => Some(
                        (*max_id)
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

    #[inline]
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

        let symbols = self.get_symbols_string(table);

        self.context
            .add_shared_table(name, version, &symbols)
            .map_err(IonParserError::ErrorAddingSharedTableToContext)?;

        Ok(())
    }

    #[inline]
    fn get_symbol_name_by_type(&self, symbol: SystemSymbolIds) -> &'static str {
        SYSTEM_SYMBOL_TABLE[symbol as usize]
    }

    #[inline]
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

    #[inline]
    fn get_symbol_name(&self, symbol_id: usize) -> Result<String, IonParserError> {
        match self.context.get_symbol_by_id(symbol_id) {
            Some(Symbol::Symbol(name)) => Ok(name.clone()),
            Some(Symbol::Dummy) | None => Err(IonParserError::SymbolIdNotDefined),
        }
    }
}
