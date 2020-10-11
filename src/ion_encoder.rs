use crate::binary_encoder::ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED;
use crate::binary_encoder::{
    encode_blob, encode_bool, encode_datetime, encode_decimal, encode_float32, encode_float64,
    encode_integer, encode_null, encode_uint, encode_varuint,
};
use crate::symbol_table::SymbolContext;
use crate::IonValue;
use num_bigint::{BigInt, BigUint};
use std::convert::TryFrom;

/// It allows to binary encode IonValues. Given the nature of the Ion binary format
/// it has two working modes. One is the method `encode_all` which takes an array
/// and encodes all. If there is any symbol it will create a initial symbol table
/// for all the values. Then you have the "add_value" which adds more and more
/// IonValues to the buffer, but it doesn't encode them, it is not until you call
/// `reset_symbol_table1` or `encode` that it does the encoding. This is because you
/// may want some control over the symbol tables in the case you have several secitions
/// in your structure.
#[derive(Debug)]
pub struct IonEncoder {
    current_buffer: Vec<u8>,
    symbol_table: SymbolContext,
}

impl Default for IonEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl IonEncoder {
    pub fn new() -> IonEncoder {
        IonEncoder {
            current_buffer: vec![],
            symbol_table: SymbolContext::new(),
        }
    }

    pub(crate) fn encode_value(&mut self, value: &IonValue) -> Vec<u8> {
        match value {
            IonValue::Null(value) => encode_null(value),
            IonValue::Bool(value) => encode_bool(value),
            IonValue::Integer(value) => encode_integer(&BigInt::from(*value)),
            IonValue::BigInteger(value) => encode_integer(value),
            IonValue::Float32(value) => encode_float32(value),
            IonValue::Float64(value) => encode_float64(value),
            IonValue::Decimal(value) => encode_decimal(value),
            IonValue::String(value) => encode_blob(8, value.as_bytes()),
            IonValue::Clob(value) => encode_blob(9, value),
            IonValue::Blob(value) => encode_blob(10, value),
            IonValue::DateTime(value) => encode_datetime(value),
            IonValue::List(value) => self.encode_list(value, false),
            IonValue::SExpr(value) => self.encode_list(value, true),
            IonValue::Symbol(symbol) => self.encode_symbol(symbol),
            IonValue::Struct(_) => panic!(),
            IonValue::Annotation(_, _) => panic!(),
        }
    }

    pub(crate) fn encode_symbol(&mut self, symbol: &str) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![];

        let mut header: u8 = 0x70;

        let id = self.symbol_table.insert_symbol(symbol);

        let mut id_bytes = encode_uint(&BigUint::from(id));
        let id_bytes_len = id_bytes.len();
        let has_len_field = id_bytes_len >= ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED.into();

        if has_len_field {
            header += ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED;
            buffer.push(header);
            let mut id_bytes_len_bytes = encode_varuint(&id_bytes_len.to_be_bytes());
            buffer.append(&mut id_bytes_len_bytes);
            buffer.append(&mut id_bytes);
        } else {
            header += u8::try_from(id_bytes_len).unwrap();
            buffer.push(header);
            buffer.append(&mut id_bytes);
        };

        buffer
    }

    pub(crate) fn encode_list(&mut self, values: &[IonValue], is_sexp: bool) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![];

        for value in values {
            let mut bytes = self.encode_value(value);

            buffer.append(&mut bytes);
        }

        let buffer_len = buffer.len();
        let has_len_field = buffer_len >= ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED.into();

        let mut header: u8 = if is_sexp { 0xC0 } else { 0xB0 };

        if has_len_field {
            header += ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED;
        } else {
            header += u8::try_from(buffer_len).unwrap();
        }

        let mut buffer = if has_len_field {
            let mut buffer_len_bytes = encode_varuint(&buffer_len.to_be_bytes());
            buffer_len_bytes.append(&mut buffer);
            buffer_len_bytes
        } else {
            buffer
        };

        buffer.insert(0, header);

        buffer
    }
}
