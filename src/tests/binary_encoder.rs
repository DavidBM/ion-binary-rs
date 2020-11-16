use crate::binary_encoder::encode_ion_value;
use crate::{IonParser, IonValue};
use bigdecimal::BigDecimal;
use chrono::{DateTime, FixedOffset};
use num_bigint::BigInt;
use std::str::FromStr;

#[test]
fn encode_integer_i64() {
    #[allow(overflowing_literals)]
    let values: Vec<i64> = vec![
        0b_1000_0000_0000_0000_0000_0000_0000_0000_1000_0000_0000_0000_0000_0000_0000_0000,
        0b_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        0b_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001,
        0b_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
        -9071905709435714,
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
        0b_0000_0000_0000_0000_0000_0000_0000_0000_1000_0000_0000_0000_0000_0000_0000_0000,
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

    let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

    assert_eq!(ion_value, resulting_ion_value);
}

#[test]
fn encode_integer_big_negative() {
    let ion_value = b"-18173238162219679736857031944447898744767430095109316084451026048678348094928854458274167288816962557611640075817315237016025726423548207924331642028847993938530524659112028449811515920726159569583847554301932799584192974700038250645135419704389244690214111003505621818033044965879076306690914532152840279256440975668846810694285470204245958782248405612488959069641454132691581386219910938587286910894148564397155066367399697230287047229035630842240888106685623631032505806388903066971508775182055551847210338095961815021030725796281642316166745051164958432783938535334657296749823645911331793861360616240344479015948";

    let ion_value = IonValue::BigInteger(BigInt::parse_bytes(ion_value, 10).unwrap());

    let bytes = encode_ion_value(&ion_value);

    let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

    assert_eq!(ion_value, resulting_ion_value);
}

/*#[test]
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
}*/

#[test]
fn encode_integer_float64() {
    let values: Vec<f64> = vec![
        f64::MIN,
        f64::MAX,
        -0.0,
        // Omiting positive 0.0 as it is an special encoding
        // case that will always get compiled to 0.0
        0.000000000121324,
        341234123412341234.123412534437,
    ];

    for ion_value in values {
        let ion_value = IonValue::Float(ion_value);

        let bytes = encode_ion_value(&ion_value);

        let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

        assert_eq!(ion_value, resulting_ion_value);
    }
}

#[test]
fn encode_integer_float64_nan() {
    let ion_value = IonValue::Float(f64::NAN);

    let bytes = encode_ion_value(&ion_value);

    let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

    if let IonValue::Float(value) = resulting_ion_value {
        if !value.is_nan() {
            panic!("It is not a nan");
        }
    } else {
        panic!("Not a float!")
    }
}

#[test]
fn encode_integer_decimal() {
    let values: Vec<BigDecimal> = vec![
        BigDecimal::from_str(&"-0").unwrap(),
        BigDecimal::from_str(&"0").unwrap(),
        BigDecimal::from_str(&"1").unwrap(),
        BigDecimal::from_str(&"-1").unwrap(),
        BigDecimal::from_str(&"-0.0").unwrap(),
        BigDecimal::from_str(&"0.0").unwrap(),
        BigDecimal::from_str(&"0.").unwrap(),
        BigDecimal::from_str(&"3297102945745762396524398765238765234876592134160293123875692584562347659243216549875569856324869856966985698696.32842368523654574562654544756435443456544435455432358454565748576554235445562514525565245").unwrap(),
    ];

    for ion_value in values {
        let ion_value = IonValue::Decimal(ion_value);

        let bytes = encode_ion_value(&ion_value);

        let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

        assert_eq!(ion_value, resulting_ion_value);
    }
}

#[test]
fn encode_integer_string() {
    let values: Vec<String> = vec![
        String::from("hola"),
        String::from(""),
        String::from(" "),
        String::from("‎"),
        String::from("ʰ ʱ ʲ ʳ ʴ ʵ ʶ ʷ ʸ ʹ ʺ ʻ ʼ ʽ ʾ ʿ ˀ ˁ ˂ ˃ ˄ ˅ ˆ ˇ ˈ ˉ ˊ ˋ ˌ ˍ ˎ ˏ ː ˑ ˒ ˓ ˔ ˕ ˖ ˗ ˘ ˙ ˚ ˛ ˜ ˝ ˞ ˠ ˡ ˢ ˣ ˤ ˥ ˦ ˧ ˨ ˩ "),
        String::from("ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨"),
        String::from("ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨ംഃഅആഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങചഛജഝഞടഠഡഢണതഥദധനപഫബഭമയരറലളഴവശഷസഹാിീുൂൃെേൈൊോൌ്ൗൠൡ൦൧൨"),
    ];

    for ion_value in values {
        let ion_value = IonValue::String(ion_value);

        let bytes = encode_ion_value(&ion_value);

        let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

        assert_eq!(ion_value, resulting_ion_value);
    }
}

#[test]
fn encode_integer_clob() {
    let values: Vec<Vec<u8>> = vec![
        vec![129, 34, 0, 0, 0, 0, 23, 123, 6, 57, 2, 1, 0, 0],
        vec![0, 0, 129, 34, 0, 0, 0, 0, 23, 123, 6, 57, 2, 1, 0, 0],
        vec![0, 0],
        vec![0],
        vec![],
    ];

    for ion_value in values {
        let ion_value = IonValue::Clob(ion_value);

        let bytes = encode_ion_value(&ion_value);

        let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

        assert_eq!(ion_value, resulting_ion_value);
    }
}

#[test]
fn encode_integer_blob() {
    let values: Vec<Vec<u8>> = vec![
        vec![129, 34, 0, 0, 0, 0, 23, 123, 6, 57, 2, 1, 0, 0],
        vec![0, 0, 129, 34, 0, 0, 0, 0, 23, 123, 6, 57, 2, 1, 0, 0],
        vec![0, 0],
        vec![0],
        vec![],
    ];

    for ion_value in values {
        let ion_value = IonValue::Blob(ion_value);

        let bytes = encode_ion_value(&ion_value);

        let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

        assert_eq!(ion_value, resulting_ion_value);
    }
}

#[test]
fn encode_integer_datetime() {
    let values: Vec<DateTime<FixedOffset>> = vec![
        DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap(),
        DateTime::parse_from_rfc3339("1996-12-19T16:39:57-00:00").unwrap(),
        DateTime::parse_from_rfc3339("1996-12-19T16:39:57-16:00").unwrap(),
        DateTime::parse_from_rfc3339("1996-12-19T16:39:57+16:00").unwrap(),
        DateTime::parse_from_rfc3339("2200-01-01T00:00:00-00:00").unwrap(),
        DateTime::parse_from_rfc3339("2200-01-01T00:00:00-08:00").unwrap(),
        DateTime::parse_from_rfc3339("0000-01-01T00:00:00-08:00").unwrap(),
    ];

    for ion_value in values {
        let ion_value = IonValue::DateTime(ion_value);

        let bytes = encode_ion_value(&ion_value);

        let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

        assert_eq!(ion_value, resulting_ion_value);
    }
}
