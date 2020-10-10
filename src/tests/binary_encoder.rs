use crate::binary_encoder::encode_ion_value;
use crate::{IonParser, IonValue};
use num_bigint::BigInt;

#[test]
fn encode_integer_i64() {
    #[allow(overflowing_literals)]
    let values: Vec<i64> = vec![
        0b_0000_0000_0000_0000_0000_0000_0000_0000_1000_0000_0000_0000_0000_0000_0000_0000,
        0b_1000_0000_0000_0000_0000_0000_0000_0000_1000_0000_0000_0000_0000_0000_0000_0000,
        0b_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        0b_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001,
        0b_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
        -9271905709435714,
        -23874324,
        -234,
        -1,
        0,
        1,
        41234,
        12342151456,
        123165237231415,
        0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001,
        0b_0100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        0b_0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
    ];

    for ion_value in values {
        let ion_value = IonValue::Integer(ion_value);

        let bytes = encode_ion_value(&ion_value);

        let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

        assert_eq!(ion_value, resulting_ion_value);
    }
}

#[test]
fn encode_integer_big() {
    let ion_value = b"18173238162219679736857031944447898744767430095109316084451026048678348094928854458274167288816962557611640075817315237016025726423548207924331642028847993938530524659112028449811515920726159569583847554301932799584192974700038250645135419704389244690214111003505621818033044965879076306690914532152840279256440975668846810694285470204245958782248405612488959069641454132691581386219910938587286910894148564397155066367399697230287047229035630842240888106685623631032505806388903066971508775182055551847210338095961815021030725796281642316166745051164958432783938535334657296749823645911331793861360616240344479015948";

    let ion_value = IonValue::BigInteger(BigInt::parse_bytes(ion_value, 10).unwrap());

    let bytes = encode_ion_value(&ion_value);

    println!("{:X?}", bytes);

    let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

    assert_eq!(ion_value, resulting_ion_value);
}

#[test]
fn encode_integer_big_negative() {
    let ion_value = b"-18173238162219679736857031944447898744767430095109316084451026048678348094928854458274167288816962557611640075817315237016025726423548207924331642028847993938530524659112028449811515920726159569583847554301932799584192974700038250645135419704389244690214111003505621818033044965879076306690914532152840279256440975668846810694285470204245958782248405612488959069641454132691581386219910938587286910894148564397155066367399697230287047229035630842240888106685623631032505806388903066971508775182055551847210338095961815021030725796281642316166745051164958432783938535334657296749823645911331793861360616240344479015948";

    let ion_value = IonValue::BigInteger(BigInt::parse_bytes(ion_value, 10).unwrap());

    let bytes = encode_ion_value(&ion_value);

    println!("{:X?}", bytes);

    let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

    assert_eq!(ion_value, resulting_ion_value);
}

#[test]
fn encode_integer_float32() {
    let values: Vec<f32> = vec![
        f32::MIN,
        f32::MAX,
        -0.0,
        0.0,
        0.000000000121324,
        341234123412341234.123412534437,
    ];

    for ion_value in values {
        let ion_value = IonValue::Float32(ion_value);

        let bytes = encode_ion_value(&ion_value);

        let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

        assert_eq!(ion_value, resulting_ion_value);
    }
}

#[test]
fn encode_integer_float32_nan() {
    let ion_value = IonValue::Float32(f32::NAN);

    let bytes = encode_ion_value(&ion_value);

    let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

    if let IonValue::Float32(value) = resulting_ion_value {
        if !value.is_nan() {
            panic!("It is not a nan");
        }
    } else {
        panic!("Not a float!")
    }
}

#[test]
fn encode_integer_float64() {
    let values: Vec<f64> = vec![
        f64::MIN,
        f64::MAX,
        -0.0,
        0.0,
        0.000000000121324,
        341234123412341234.123412534437,
    ];

    for ion_value in values {
        let ion_value = IonValue::Float64(ion_value);

        let bytes = encode_ion_value(&ion_value);

        let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

        assert_eq!(ion_value, resulting_ion_value);
    }
}

#[test]
fn encode_integer_float64_nan() {
    let ion_value = IonValue::Float64(f64::NAN);

    let bytes = encode_ion_value(&ion_value);

    let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

    if let IonValue::Float64(value) = resulting_ion_value {
        if !value.is_nan() {
            panic!("It is not a nan");
        }
    } else {
        panic!("Not a float!")
    }
}
