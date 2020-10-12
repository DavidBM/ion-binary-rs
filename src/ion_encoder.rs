use crate::binary_encoder::{
    encode_blob, encode_bool, encode_datetime, encode_decimal, encode_float32, encode_float64,
    encode_integer, encode_null, encode_uint, encode_varuint,
    ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED,
};
use crate::binary_parser_types::{SystemSymbolIds, SYSTEM_SYMBOL_TABLE};
use crate::symbol_table::SymbolContext;
use crate::IonValue;
use num_bigint::{BigInt, BigUint};
use std::collections::HashMap;
use std::convert::TryFrom;

/// TODO: It allows to binary encode IonValues. Given the nature of the Ion binary format
/// it has two working modes. One is the method `encode_all` which takes an array
/// and encodes all. If there is any symbol it will create a initial symbol table
/// for all the values. Then you have the "add_value" which adds more and more
/// IonValues to the buffer, but it doesn't encode them, it is not until you call
/// `reset_symbol_table1` or `encode` that it does the encoding. This is because you
/// may want some control over the symbol tables in the case you have several secitions
/// in your structure.
#[derive(Debug)]
pub struct IonEncoder {
    current_buffer: Vec<IonValue>,
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

    pub fn add(&mut self, value: IonValue) {
        self.current_buffer.push(value);
    }

    pub fn encode(&mut self) -> Vec<u8> {
        let mut values = vec![];

        values.append(&mut self.current_buffer);

        let mut values_buffer: Vec<u8> = values
            .into_iter()
            .map(|value| self.encode_value(&value))
            .flatten()
            .collect();

        let mut symbol_table = self.encode_current_symbol_table();

        let mut buffer = IonEncoder::get_ion_1_0_header();

        buffer.append(&mut symbol_table);
        buffer.append(&mut values_buffer);

        buffer
    }

    fn get_ion_1_0_header() -> Vec<u8> {
        vec![0xE0, 0x01, 0x00, 0xEA]
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
            IonValue::Struct(value) => self.encode_struct(value),
            IonValue::Annotation(annotations, value) => self.encode_annotation(annotations, value),
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

    pub(crate) fn encode_annotation(
        &mut self,
        annotations: &[String],
        value: &IonValue,
    ) -> Vec<u8> {
        let mut annot_buffer: Vec<u8> = vec![];

        for annot in annotations {
            let annot_symbol = self.symbol_table.insert_symbol(annot);
            let mut annot_symbol_bytes = encode_varuint(&annot_symbol.to_be_bytes());
            annot_buffer.append(&mut annot_symbol_bytes);
        }

        let mut annot_len_bytes = encode_varuint(&annot_buffer.len().to_be_bytes());

        let mut value_bytes = self.encode_value(value);

        annot_len_bytes.append(&mut annot_buffer);

        let mut buffer = annot_len_bytes;

        buffer.append(&mut value_bytes);

        let len = buffer.len();
        let has_len_field = len >= ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED.into();

        let mut header = 0xE0;

        let mut final_buffer: Vec<u8> = vec![];

        if has_len_field {
            header += ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED;
            final_buffer.push(header);
            let mut len_bytes = encode_varuint(&len.to_be_bytes());
            final_buffer.append(&mut len_bytes);
            final_buffer.append(&mut buffer);
        } else {
            header += u8::try_from(len).unwrap();
            final_buffer.push(header);
            final_buffer.append(&mut buffer);
        };

        final_buffer
    }

    pub(crate) fn encode_struct(&mut self, value: &HashMap<String, IonValue>) -> Vec<u8> {
        let mut content_buffer: Vec<u8> = vec![];

        for (key, value) in value {
            let symbol = self.symbol_table.insert_symbol(key);
            let mut symbol_bytes = encode_varuint(&symbol.to_be_bytes());
            let mut value_bytes = self.encode_value(value);
            content_buffer.append(&mut symbol_bytes);
            content_buffer.append(&mut value_bytes);
        }

        let content_len = content_buffer.len();
        let has_len_field = content_len >= ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED.into();

        let mut header = 0xD0;

        let mut buffer: Vec<u8> = vec![];

        if has_len_field {
            header += ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED;
            buffer.push(header);
            let mut content_len_bytes = encode_varuint(&content_len.to_be_bytes());
            buffer.append(&mut content_len_bytes);
            buffer.append(&mut content_buffer);
        } else {
            header += u8::try_from(content_len).unwrap();
            buffer.push(header);
            buffer.append(&mut content_buffer);
        };

        buffer
    }

    pub(crate) fn encode_current_symbol_table(&mut self) -> Vec<u8> {
        let symbols = self.symbol_table.dump_all_local_symbols();

        let symbols = IonValue::List(symbols.into_iter().map(IonValue::String).collect());

        let mut annotation_struct = HashMap::new();

        let symbols_symbol = SYSTEM_SYMBOL_TABLE[SystemSymbolIds::Symbols as usize].to_string();
        let local_table_annotation_symbol =
            SYSTEM_SYMBOL_TABLE[SystemSymbolIds::IonSymbolTable as usize].to_string();

        annotation_struct.insert(symbols_symbol, symbols);

        let annotation_struct = IonValue::Struct(annotation_struct);

        let annotation = IonValue::Annotation(
            vec![local_table_annotation_symbol],
            Box::new(annotation_struct),
        );

        self.encode_value(&annotation)
    }
}
