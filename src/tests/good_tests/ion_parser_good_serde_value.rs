use std::{collections::HashMap, convert::TryInto, str::FromStr};

use num_bigint::BigInt;
use serde_json::{json, Value};

use crate::{IonValue, NullIonValue};

#[test]
fn serde_from_ion_null() {
    let null_ion = IonValue::Null(NullIonValue::Null);
    let serde_null: Value = null_ion.try_into().unwrap();

    assert_eq!(serde_null, json!(null));
}

#[test]
fn serde_from_ion_bool() {
    let bool_ion = IonValue::Bool(false);
    let serde_bool: Value = bool_ion.try_into().unwrap();

    assert_eq!(serde_bool, json!(false));
}

#[test]
fn serde_from_ion_integer() {
    let integer_ion = IonValue::Integer(18);
    let serde_integer: Value = integer_ion.try_into().unwrap();

    assert_eq!(serde_integer, json!(18));
}

#[test]
fn serde_from_ion_big_integer() {
    let big_int = BigInt::from_str("-201545002204320").unwrap();
    let big_int_ion = IonValue::BigInteger(big_int);
    let serde_big_int: Value = big_int_ion.try_into().unwrap();
    let number = -201545002204320 as i64;

    assert_eq!(serde_big_int, Value::Number(number.into()));
}

#[test]
fn serde_from_ion_float() {
    let float_ion = IonValue::Float(36.432);
    let serde_float: Value = float_ion.try_into().unwrap();

    assert_eq!(serde_float, json!(36.432));
}

#[test]
fn serde_from_ion_string() {
    let string_ion = IonValue::String("Argumentum baculinum".to_string());
    let serde_string: Value = string_ion.try_into().unwrap();

    assert_eq!(serde_string, json!("Argumentum baculinum"));
}

#[test]
fn serde_from_ion_list() {
    let internal_vector = vec![IonValue::Float(2.2), IonValue::Float(1.2)];

    let mut internal_hashmap = HashMap::<String, IonValue>::new();
    internal_hashmap.insert("first".to_string(), IonValue::Bool(true));

    let mut json_map = HashMap::<String, Value>::new();
    json_map.insert("first".to_string(), json!(true));

    let vector = vec![
        IonValue::Bool(true),
        IonValue::Integer(2),
        IonValue::Float(3.2),
        IonValue::List(internal_vector),
        IonValue::Struct(internal_hashmap),
    ];
    let ion_list = IonValue::List(vector);
    let serde_list: Value = ion_list.try_into().unwrap();

    assert_eq!(serde_list[0], json!(true));
    assert_eq!(serde_list[1], json!(2));
    assert_eq!(serde_list[2], json!(3.2));
    assert_eq!(serde_list[3], json!(vec!(2.2, 1.2)));
    assert_eq!(serde_list[4], json!(json_map));
}

#[test]
fn serde_from_ion_struct() {
    let mut hash_map = HashMap::<String, IonValue>::new();
    hash_map.insert("bool".to_string(), IonValue::Bool(true));
    hash_map.insert("int".to_string(), IonValue::Integer(3));
    hash_map.insert("float".to_string(), IonValue::Float(12.3));
    hash_map.insert(
        "vector".to_string(),
        IonValue::List(vec![IonValue::Integer(1), IonValue::Integer(2)]),
    );

    let ion_struct = IonValue::Struct(hash_map);
    let serde_struct: Value = ion_struct.try_into().unwrap();

    assert_eq!(serde_struct["bool"], json!(true));
    assert_eq!(serde_struct["int"], json!(3));
    assert_eq!(serde_struct["float"], json!(12.3));
    assert_eq!(serde_struct["vector"], json!(vec!(1, 2)));
}

#[test]
fn ion_from_serde_null() {
    let null_value = json!(null);
    let ion_null: IonValue = null_value.try_into().unwrap();

    assert_eq!(ion_null, IonValue::Null(NullIonValue::Null));
}

#[test]
fn ion_from_serde_bool() {
    let bool_value = json!(true);
    let ion_bool: IonValue = bool_value.try_into().unwrap();

    assert_eq!(ion_bool, IonValue::Bool(true));
}

#[test]
fn ion_from_serde_integer() {
    let integer_value = json!(3);
    let ion_integer: IonValue = integer_value.try_into().unwrap();

    assert_eq!(ion_integer, IonValue::Integer(3));
}

#[test]
fn ion_from_serde_big_integer() {
    let big_int = i64::max_value();
    let big_int_value = json!(big_int);
    let ion_big_int: IonValue = big_int_value.try_into().unwrap();

    assert_eq!(ion_big_int, IonValue::Integer(big_int));
}

#[test]
fn ion_from_serde_float() {
    let float_value = json!(23.432);
    let ion_float: IonValue = float_value.try_into().unwrap();

    assert_eq!(ion_float, IonValue::Float(23.432));
}

#[test]
fn ion_from_serde_string() {
    let string_value = json!("Lorem ipsum");
    let ion_string: IonValue = string_value.try_into().unwrap();

    assert_eq!(ion_string, IonValue::String("Lorem ipsum".to_string()));
}

#[test]
fn ion_from_serde_list() {
    let value_list = json!(vec!(true, false, true));
    let ion_list: IonValue = value_list.try_into().unwrap();

    assert_eq!(
        ion_list,
        IonValue::List(vec!(
            IonValue::Bool(true),
            IonValue::Bool(false),
            IonValue::Bool(true)
        ))
    );
}

#[test]
fn ion_from_serde_struct() {
    let mut internal_hashmap = HashMap::<String, Value>::new();
    internal_hashmap.insert("first".to_string(), json!(2));

    let mut ion_internal_hashmap = HashMap::<String, IonValue>::new();
    ion_internal_hashmap.insert("first".to_string(), IonValue::Integer(2));

    let mut hashmap = HashMap::<String, Value>::new();
    hashmap.insert("bool".to_string(), json!(true));
    hashmap.insert("int".to_string(), json!(2));
    hashmap.insert("float".to_string(), json!(5.8));
    hashmap.insert("vector".to_string(), json!(vec!(true, false)));
    hashmap.insert("map".to_string(), json!(internal_hashmap));
    let value_struct = json!(hashmap);
    let ion_struct: IonValue = value_struct.try_into().unwrap();

    let mut ion_hashmap = HashMap::<String, IonValue>::new();
    ion_hashmap.insert("bool".to_string(), IonValue::Bool(true));
    ion_hashmap.insert("int".to_string(), IonValue::Integer(2));
    ion_hashmap.insert("float".to_string(), IonValue::Float(5.8));
    ion_hashmap.insert(
        "vector".to_string(),
        IonValue::List(vec![IonValue::Bool(true), IonValue::Bool(false)]),
    );
    ion_hashmap.insert("map".to_string(), IonValue::Struct(ion_internal_hashmap));

    assert_eq!(ion_struct, IonValue::Struct(ion_hashmap));
}
