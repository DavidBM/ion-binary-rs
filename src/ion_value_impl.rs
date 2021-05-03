use crate::{IonExtractionError, IonParserError, IonValue, NullIonValue, SerdeJsonParseError};
use bigdecimal::BigDecimal;
use chrono::{DateTime, FixedOffset, Utc};
use num_bigint::{BigInt, BigUint};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use serde_json::Value;
use IonParserError::ValueExtractionFailure;

impl TryFrom<IonValue> for std::collections::HashMap<String, IonValue> {
    type Error = IonParserError;
    fn try_from(value: IonValue) -> Result<Self, Self::Error> {
        match value {
            IonValue::Struct(value) => Ok(value),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for String {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::String(value) | IonValue::Symbol(value) => Ok(value),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for u64 {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => value.try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            IonValue::BigInteger(value) => value.try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for i64 {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => Ok(value),
            IonValue::BigInteger(value) => value.try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for u32 {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => value.try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            IonValue::BigInteger(value) => value.try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for i32 {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => value.try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            IonValue::BigInteger(value) => value.try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for BigUint {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => value.try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            IonValue::BigInteger(value) => value.try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for BigInt {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => Ok(BigInt::from(value)),
            IonValue::BigInteger(value) => Ok(value),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for BigDecimal {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Decimal(value) => Ok(value),
            IonValue::Integer(value) => Ok(BigDecimal::from(value)),
            IonValue::BigInteger(value) => Ok(BigDecimal::from(value)),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for f64 {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => Ok(value as f64),
            IonValue::BigInteger(value) => i64::try_from(value)
                .map_err(|e| {
                    ValueExtractionFailure(IonExtractionError::NumericTransformationError(
                        Box::new(e),
                    ))
                })
                .map(|value| value as f64),
            IonValue::Float(value) => Ok(value),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for f32 {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => Ok(value as f32),
            IonValue::BigInteger(value) => i64::try_from(value)
                .map_err(|e| {
                    ValueExtractionFailure(IonExtractionError::NumericTransformationError(
                        Box::new(e),
                    ))
                })
                .map(|value| value as f32),
            IonValue::Float(value) => Ok(value as f32),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for DateTime<Utc> {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::DateTime(value) => Ok(value.with_timezone(&Utc)),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for DateTime<FixedOffset> {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::DateTime(value) => Ok(value),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for bool {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Bool(value) => Ok(value),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for Vec<u8> {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Clob(value) => Ok(value),
            IonValue::Blob(value) => Ok(value),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value),
            )),
        }
    }
}

impl TryFrom<IonValue> for serde_json::Value {
    type Error = IonParserError;

    fn try_from(value: IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Null(_) => Ok(Value::Null),
            IonValue::Bool(value) => Ok(Value::Bool(value)),
            IonValue::Integer(value) => Ok(Value::from(value.into())),
            IonValue::BigInteger(value) => Ok(Value::Number(i64::try_from(value)?.into())),
            ion_value @ IonValue::Decimal(_) => {
                let number = f64::try_from(ion_value)?;

                let json_number = serde_json::Number::from_f64(number)
                    .ok_or(IonParserError::DecimalNotANumericValue(number))?;

                Ok(Value::Number(json_number))
            }
            IonValue::Float(value) => Ok(Value::from(value)),
            IonValue::String(value)
            | IonValue::Symbol(value) => Ok(Value::from(value)),
            IonValue::List(_) => Ok(serde_json::Value::try_from(value.clone())?),
            IonValue::Struct(ref values) => {
                let mut result_map = serde_json::Map::new();
                for (key, ion_value) in values {
                    result_map.insert(key.to_string(), Value::try_from(ion_value)?);
                }
                Ok(Value::from(result_map))
            }
            _ => Err(IonParserError::TypeNotSupported(value)),
        }
    }
}

impl TryFrom<&IonValue> for Vec<IonValue> {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::List(value) => Ok(value.to_vec()),
            IonValue::SExpr(value) => Ok(value.to_vec()),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value.clone()),
            )),
        }
    }
}

impl TryFrom<&IonValue> for HashMap<String, IonValue> {
    type Error = ();
    fn try_from(value: &IonValue) -> Result<Self, Self::Error> {
        if let IonValue::Struct(value) = value {
            Ok(value.clone())
        } else {
            Err(())
        }
    }
}

impl TryFrom<&IonValue> for String {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::String(value) | IonValue::Symbol(value) => Ok(value.clone()),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value.clone()),
            )),
        }
    }
}

impl TryFrom<&IonValue> for u64 {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => (*value).try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            IonValue::BigInteger(value) => value.try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value.clone()),
            )),
        }
    }
}

impl TryFrom<&IonValue> for i64 {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => Ok(*value),
            IonValue::BigInteger(value) => value.try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value.clone()),
            )),
        }
    }
}

impl TryFrom<&IonValue> for u32 {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => (*value).try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            IonValue::BigInteger(value) => value.try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value.clone()),
            )),
        }
    }
}

impl TryFrom<&IonValue> for i32 {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => (*value).try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            IonValue::BigInteger(value) => value.try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value.clone()),
            )),
        }
    }
}

impl TryFrom<&IonValue> for BigUint {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => (*value).try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            IonValue::BigInteger(value) => value.try_into().map_err(|e| {
                ValueExtractionFailure(IonExtractionError::NumericTransformationError(Box::new(e)))
            }),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value.clone()),
            )),
        }
    }
}

impl TryFrom<&IonValue> for BigInt {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => Ok(BigInt::from(*value)),
            IonValue::BigInteger(value) => Ok(value.clone()),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value.clone()),
            )),
        }
    }
}

impl TryFrom<&IonValue> for f64 {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => Ok(*value as f64),
            IonValue::BigInteger(value) => i64::try_from(value)
                .map_err(|e| {
                    ValueExtractionFailure(IonExtractionError::NumericTransformationError(
                        Box::new(e),
                    ))
                })
                .map(|value| value as f64),
            IonValue::Float(value) => Ok(*value),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value.clone()),
            )),
        }
    }
}

impl TryFrom<&IonValue> for f32 {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Integer(value) => Ok(*value as f32),
            IonValue::BigInteger(value) => i64::try_from(value)
                .map_err(|e| {
                    ValueExtractionFailure(IonExtractionError::NumericTransformationError(
                        Box::new(e),
                    ))
                })
                .map(|value| value as f32),
            IonValue::Float(value) => Ok(*value as f32),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value.clone()),
            )),
        }
    }
}

impl TryFrom<&IonValue> for DateTime<Utc> {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::DateTime(value) => Ok(value.with_timezone(&Utc)),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value.clone()),
            )),
        }
    }
}

impl TryFrom<&IonValue> for DateTime<FixedOffset> {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::DateTime(value) => Ok(*value),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value.clone()),
            )),
        }
    }
}

impl TryFrom<&IonValue> for bool {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Bool(value) => Ok(*value),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value.clone()),
            )),
        }
    }
}

impl TryFrom<&IonValue> for Vec<u8> {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Clob(value) => Ok(value.to_vec()),
            IonValue::Blob(value) => Ok(value.to_vec()),
            _ => Err(ValueExtractionFailure(
                IonExtractionError::TypeNotSupported(value.clone()),
            )),
        }
    }
}

impl TryFrom<&IonValue> for serde_json::Value {
    type Error = IonParserError;

    fn try_from(value: &IonValue) -> Result<Self, IonParserError> {
        match value {
            IonValue::Null(_) => Ok(Value::Null),
            IonValue::Bool(value) => Ok(Value::Bool(*value)),
            IonValue::Integer(value) => Ok(Value::from(value.clone())),
            IonValue::BigInteger(value) => Ok(Value::Number(i64::try_from(value.clone())?.into())),
            ion_value @ IonValue::Decimal(_) => {
                let number = f64::try_from(ion_value)?;

                let json_number = serde_json::Number::from_f64(number)
                    .ok_or(IonParserError::DecimalNotANumericValue(number))?;

                Ok(Value::Number(json_number))
            }
            IonValue::Float(value) => Ok(Value::from(*value)),
            IonValue::String(value)
            | IonValue::Symbol(value) => Ok(Value::from(value.to_string())),
            IonValue::List(values) => {
                let mut result_vec: Vec<Value> = vec![];

                for value in values.iter() {
                    result_vec.push(Value::try_from(value.clone())?)
                }

                Ok(Value::Array(result_vec))
            }
            IonValue::Struct(values) => {
                let mut result_map = serde_json::Map::new();
                for (key, ion_value) in values {
                    result_map.insert(key.to_string(), serde_json::Value::try_from(ion_value.clone())?);
                }

                Ok(Value::from(result_map))
            }
            _ => Err(IonParserError::TypeNotSupported(value.clone())),
        }
    }
}

impl From<String> for IonValue {
    fn from(value: String) -> IonValue {
        IonValue::String(value)
    }
}

impl From<&str> for IonValue {
    fn from(value: &str) -> IonValue {
        IonValue::String(value.to_string())
    }
}

impl From<u16> for IonValue {
    fn from(value: u16) -> IonValue {
        IonValue::Integer(value.into())
    }
}

impl From<i16> for IonValue {
    fn from(value: i16) -> IonValue {
        IonValue::Integer(value.into())
    }
}

impl From<u32> for IonValue {
    fn from(value: u32) -> IonValue {
        IonValue::Integer(value.into())
    }
}

impl From<i32> for IonValue {
    fn from(value: i32) -> IonValue {
        IonValue::Integer(value.into())
    }
}

impl From<u64> for IonValue {
    fn from(value: u64) -> IonValue {
        match i64::try_from(value) {
            Ok(value) => IonValue::Integer(value),
            Err(_) => IonValue::BigInteger(BigInt::from(value)),
        }
    }
}

impl From<i64> for IonValue {
    fn from(value: i64) -> IonValue {
        IonValue::Integer(value)
    }
}

impl From<u128> for IonValue {
    fn from(value: u128) -> IonValue {
        IonValue::BigInteger(BigInt::from(value))
    }
}

impl From<i128> for IonValue {
    fn from(value: i128) -> IonValue {
        IonValue::BigInteger(BigInt::from(value))
    }
}

impl From<BigInt> for IonValue {
    fn from(value: BigInt) -> IonValue {
        IonValue::BigInteger(value)
    }
}

impl From<BigUint> for IonValue {
    fn from(value: BigUint) -> IonValue {
        IonValue::BigInteger(BigInt::from(value))
    }
}

impl From<DateTime<FixedOffset>> for IonValue {
    fn from(value: DateTime<FixedOffset>) -> IonValue {
        IonValue::DateTime(value)
    }
}

impl From<DateTime<Utc>> for IonValue {
    fn from(value: DateTime<Utc>) -> IonValue {
        IonValue::DateTime(value.with_timezone(&FixedOffset::east(0)))
    }
}

impl From<bool> for IonValue {
    fn from(value: bool) -> IonValue {
        IonValue::Bool(value)
    }
}

impl From<Vec<u8>> for IonValue {
    fn from(value: Vec<u8>) -> IonValue {
        IonValue::Blob(value)
    }
}

impl From<&[u8]> for IonValue {
    fn from(value: &[u8]) -> IonValue {
        IonValue::Blob(value.to_vec())
    }
}

impl From<f32> for IonValue {
    fn from(value: f32) -> IonValue {
        IonValue::Float(value.into())
    }
}

impl From<f64> for IonValue {
    fn from(value: f64) -> IonValue {
        IonValue::Float(value)
    }
}

impl From<BigDecimal> for IonValue {
    fn from(value: BigDecimal) -> IonValue {
        IonValue::Decimal(value)
    }
}

impl<I: Into<IonValue>> From<Vec<I>> for IonValue {
    fn from(values: Vec<I>) -> Self {
        let mut vec: Vec<IonValue> = vec![];

        for value in values {
            vec.push(value.into())
        }

        IonValue::List(vec)
    }
}

impl<I: Into<IonValue> + Clone> From<&[I]> for IonValue {
    fn from(values: &[I]) -> Self {
        let mut vec: Vec<IonValue> = vec![];

        for value in values.to_vec() {
            vec.push(value.into())
        }

        IonValue::List(vec)
    }
}

impl<I: Into<IonValue>, K: Into<String>> From<HashMap<K, I>> for IonValue {
    fn from(values: HashMap<K, I>) -> Self {
        let mut vec: HashMap<String, IonValue> = HashMap::new();

        for (key, value) in values.into_iter() {
            vec.insert(key.into(), value.into());
        }

        IonValue::Struct(vec)
    }
}

impl<I: Into<IonValue> + Clone> From<&I> for IonValue {
    fn from(value: &I) -> IonValue {
        value.clone().into()
    }
}

impl TryFrom<serde_json::Value> for IonValue {
    type Error = SerdeJsonParseError;

    fn try_from(value: serde_json::Value) -> Result<IonValue, SerdeJsonParseError> {
        match value {
            serde_json::Value::Null => Ok(IonValue::Null(NullIonValue::Null)),
            serde_json::Value::Bool(bool) => Ok(bool.into()),
            serde_json::Value::Number(number) => {
                if number.is_f64() {
                    number
                        .as_f64()
                        .ok_or(SerdeJsonParseError::WrongNumberType)
                        .map(Into::into)
                } else if number.is_i64() {
                    number
                        .as_i64()
                        .ok_or(SerdeJsonParseError::WrongNumberType)
                        .map(Into::into)
                } else if number.is_u64() {
                    number
                        .as_u64()
                        .ok_or(SerdeJsonParseError::WrongNumberType)
                        .map(Into::into)
                } else {
                    Err(SerdeJsonParseError::NonExistentNumberType)
                }
            }
            serde_json::Value::String(string) => Ok(string.into()),
            serde_json::Value::Array(array) => {
                let list: Result<Vec<IonValue>, SerdeJsonParseError> = array
                    .into_iter()
                    .map(|element| element.try_into())
                    .collect();
                match list {
                    Ok(list) => Ok(list.into()),
                    Err(error) => Err(error),
                }
            }
            serde_json::Value::Object(map) => {
                let mut hash_map = HashMap::<String, IonValue>::new();
                for (key, value) in map.into_iter() {
                    let ion_value = value.try_into()?;
                    hash_map.insert(key.to_string(), ion_value);
                }
                Ok(hash_map.into())
            }
        }
    }
}
