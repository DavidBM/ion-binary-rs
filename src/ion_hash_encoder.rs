use digest::Digest;
use std::collections::HashMap;
use crate::binary_parser_types::{SYSTEM_SYMBOL_TABLE, SystemSymbolIds};
use chrono::{FixedOffset, DateTime};
use crate::binary_encoder::{encode_varint, encode_int, encode_datetime_representation};
use bigdecimal::BigDecimal;
use num_bigint::{BigInt, Sign};
use bigdecimal::Zero;
use crate::{ IonValue, NullIonValue };

pub fn encode_value_for_hash<D: Digest>(value: &IonValue) -> Vec<u8> {

	let buffer = encode_value::<D>(value);

	let mut escaped_buffer = escape_buffer(&buffer);

	let mut buffer = vec![0x0B];
	buffer.append(&mut escaped_buffer);
	buffer.push(0x0E);

	buffer
}

fn encode_value<D: Digest>(value: &IonValue) -> Vec<u8> {
	match value {
		IonValue::Null(value) => encode_null_value(value),
		IonValue::Bool(value) => encode_bool_value(value),
		IonValue::Integer(value) => encode_integer_value(value),
		IonValue::BigInteger(value) => encode_big_integer_value(value),
		IonValue::Float32(value) => encode_float_value(&(*value as f64)),
		IonValue::Float64(value) => encode_float_value(value),
		IonValue::Decimal(value) => encode_decimal_value(value),
		IonValue::DateTime(value) => encode_datetime_value(value),
		IonValue::String(value) => encode_string(value, 0x08),
		IonValue::Symbol(value) => encode_symbol(value),
		IonValue::Clob(value) => encode_blob(value, 0x90),
		IonValue::Blob(value) => encode_blob(value, 0xA0),
		IonValue::List(value) => encode_list::<D>(value, 0xB0),
		IonValue::SExpr(value) => encode_list::<D>(value, 0xC0),
		IonValue::Struct(value) => encode_struct::<D>(value),
		IonValue::Annotation(annotations, value) => encode_annotation::<D>(annotations, value),
	}
}

fn encode_annotation<D: Digest>(annotations: &[String], value: &IonValue) -> Vec<u8> {
	let mut buffer = vec![];

	for annotation in annotations {
		buffer.append(&mut annotation.as_bytes().to_vec());
	}

	buffer.append(&mut encode_value::<D>(value));

	buffer

}

fn encode_struct<D: Digest>(values: &HashMap<String, IonValue>) -> Vec<u8> {
	let mut hashes: Vec<Vec<u8>> = vec![];

	for (name, value) in values {
		let mut buffer = encode_symbol(name);
		buffer.append(&mut encode_value::<D>(value));
		let hash = D::digest(&buffer).to_vec();
		hashes.push(hash);
	}

	hashes.sort();

	let hashes_buffer: Vec<u8> = hashes.into_iter().flatten().collect();

	let mut buffer: Vec<u8> = escape_buffer(&hashes_buffer);

	let mut header = vec![0xD0];
	header.append(&mut buffer);
	
	header
}

fn encode_list<D: Digest>(values: &[IonValue], header: u8) -> Vec<u8> {
	let mut buffer = vec![header];

	for value in values {
		buffer.append(&mut encode_value::<D>(value))
	}

	buffer
}

fn encode_blob(value: &[u8], header: u8) -> Vec<u8> {
	let mut buffer = vec![header];

	buffer.append(&mut value.to_owned());

	buffer
}

fn encode_symbol(value: &str) -> Vec<u8> {
	if value == SYSTEM_SYMBOL_TABLE[SystemSymbolIds::Zero as usize] {
		return encode_string(value, 0x71);
	}

	encode_string(value, 0x70)
}

fn encode_string(value: &str, header: u8) -> Vec<u8> {
	let mut buffer = vec![header];

	buffer.append(&mut value.as_bytes().to_vec());

	buffer
}

fn encode_datetime_value(value: &DateTime<FixedOffset>) -> Vec<u8> {
	let mut buffer = vec![0x06];

	buffer.append(&mut encode_datetime_representation(value));

	buffer
}

fn encode_decimal_value(value: &BigDecimal) -> Vec<u8> {
	let mut buffer = vec![0x50];

	if value.is_zero() {
		return buffer;
	}

	let (coefficient, exponent) = value.as_bigint_and_exponent();

	let mut exponent = if exponent.is_zero() {
		vec![0x80]
	} else {
		encode_varint(&exponent.abs().to_be_bytes(), exponent.is_negative())
	};

	let mut coefficient = encode_int(&coefficient);

	buffer.append(&mut exponent);
	buffer.append(&mut coefficient);

	buffer
}

fn encode_float_value(value: &f64) -> Vec<u8> {
	let mut buffer = vec![0x40];

	if value.is_nan() {
		buffer.append(&mut vec![0x7F, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
		return buffer;
	}

	if value.is_zero() && value.is_sign_positive() {
		return buffer;
	}

	if value.is_infinite() && value.is_sign_positive() {
		buffer.append(&mut vec![0x7f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
		return buffer;
	}

	if value.is_infinite() && value.is_sign_negative() {
		buffer.append(&mut vec![0xff, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
		return buffer;
	}

	if value.is_zero() && value.is_sign_negative() {
		buffer.append(&mut vec![0x80, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
		return buffer;
	}

	buffer.append(&mut value.to_be_bytes().to_vec());

	buffer
}

fn encode_big_integer_value(value: &BigInt) -> Vec<u8> {
	if value.is_zero() {
		return vec![0x20]
	}

	// It returns the bytes encoded as positive number, same as Ion
	let (sign, mut bytes) = value.to_bytes_be();

	let mut buffer = if let Sign::Minus = sign {
		vec![0x20]
	} else {
		vec![0x30]
	};

	buffer.append(&mut bytes);

	buffer
}

fn encode_integer_value(value: &i64) -> Vec<u8> {
	encode_big_integer_value(&BigInt::from(*value))
}

fn encode_bool_value(value: &bool) -> Vec<u8> {
	if *value {
		vec![0x11]
	} else {
		vec![0x10]
	}
}

fn encode_null_value(value: &NullIonValue) -> Vec<u8> {
	match value {
    NullIonValue::Null => vec![0x0F],
    NullIonValue::Bool => vec![0x1F],
    NullIonValue::Integer => vec![0x2F],
    NullIonValue::BigInteger => vec![0x2F],
    NullIonValue::Float => vec![0x4F],
    NullIonValue::Decimal => vec![0x5F],
    NullIonValue::DateTime => vec![0x6F],
    NullIonValue::Symbol => vec![0x7F],
    NullIonValue::String => vec![0x8F],
    NullIonValue::Clob => vec![0x9F],
    NullIonValue::Blob => vec![0xAF],
    NullIonValue::List => vec![0xBF],
    NullIonValue::SExpr => vec![0xCF],
    NullIonValue::Struct => vec![0xDF],
    NullIonValue::Annotation => vec![0xEF],
	}
}

fn escape_buffer(buffer: &[u8]) -> Vec<u8> {
	let mut escaped_buffer = vec![];

	for byte in buffer {
		match byte {
			0x0B | 0x0C | 0x0E => {
				escaped_buffer.push(0x0C);
				escaped_buffer.push(*byte);
			},
			_ => escaped_buffer.push(*byte),
		}
	}

	escaped_buffer
}
