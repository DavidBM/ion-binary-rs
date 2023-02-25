use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ion_binary_rs::{IonEncoder, IonParser};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ion decode simple", |b| {
        let ion_test = b"\xe0\x01\0\xea\xee\xa6\x81\x83\xde\xa2\x87\xbe\x9f\x83VIN\x84Type\x84Year\x84Make\x85Model\x85Color\xde\xb9\x8a\x8e\x911C4RJFAG0FC625797\x8b\x85Sedan\x8c\"\x07\xe3\x8d\x88Mercedes\x8e\x87CLK 350\x8f\x85White";

        b.iter(|| {
            IonParser::new(black_box(&ion_test[..])).consume_all()
        })
    });

    c.bench_function("bson decode simple", |b| {

        let bson_test = b"\x71\x00\x00\x00\x02\x4d\x6f\x64\x65\x6c\x00\x08\x00\x00\x00\x43\x4c\x4b\x20\x33\x35\x30\x00\x02\x54\x79\x70\x65\x00\x06\x00\x00\x00\x53\x65\x64\x61\x6e\x00\x02\x43\x6f\x6c\x6f\x72\x00\x06\x00\x00\x00\x57\x68\x69\x74\x65\x00\x02\x56\x49\x4e\x00\x12\x00\x00\x00\x31\x43\x34\x52\x4a\x46\x41\x47\x30\x46\x43\x36\x32\x35\x37\x39\x37\x00\x02\x4d\x61\x6b\x65\x00\x09\x00\x00\x00\x4d\x65\x72\x63\x65\x64\x65\x73\x00\x10\x59\x65\x61\x72\x00\xe3\x07\x00\x00\x00";

        b.iter(|| {
            bson::Document::from_reader(black_box(&mut bson_test.clone().as_slice()))
        })
    });

    c.bench_function("ion encode simple", |b| {
        let ion_test = b"\xe0\x01\0\xea\xee\xa6\x81\x83\xde\xa2\x87\xbe\x9f\x83VIN\x84Type\x84Year\x84Make\x85Model\x85Color\xde\xb9\x8a\x8e\x911C4RJFAG0FC625797\x8b\x85Sedan\x8c\"\x07\xe3\x8d\x88Mercedes\x8e\x87CLK 350\x8f\x85White";
        let value = IonParser::new(black_box(&ion_test[..])).consume_value().unwrap();

        b.iter(|| {
            let mut encoder = IonEncoder::new();
            encoder.add(value.0.clone());
            encoder.encode()
        })
    });

    c.bench_function("bson encode simple", |b| {
        let doc = bson::doc! {
           "Model": "CLK 350",
           "Type": "Sedan",
           "Color": "White",
           "VIN": "1C4RJFAG0FC625797",
           "Make": "Mercedes",
           "Year": 2019,
        };

        b.iter(|| {
            bson::raw::RawDocumentBuf::from_document(&doc.clone())
                .unwrap()
                .into_bytes()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
