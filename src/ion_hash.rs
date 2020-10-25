use crate::ion_hash_encoder::encode_value_for_hash;
use crate::IonValue;
use std::cmp::{Ordering, PartialEq};
use digest::Digest;
use sha2::Sha256;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct IonHash<D: Digest = Sha256> {
    buffer: Vec<u8>,
    hasher_type: PhantomData<D>,
}

impl <D: Digest>IonHash<D> {
    pub fn new() -> IonHash<D> {
        IonHash {
            buffer: vec![],
            hasher_type: PhantomData,
        }
    }

    pub fn from_bytes(buf: &[u8]) -> IonHash<D> {
        let hased_bytes = D::digest(buf);
        IonHash::<D>::from_hashes_bytes(&hased_bytes)
    }

    pub fn from_hashes_bytes(buf: &[u8]) -> IonHash<D> {
        IonHash {
            buffer: buf.to_vec(),
            hasher_type: PhantomData
        }
    }

    pub fn from_ion_vaue(value: &IonValue) -> IonHash<D> {
        let mut hash = IonHash::<D>::new();

        hash.add_ion_value(value);

        hash
    }

    pub fn with_hasher<D1: Digest>() -> IonHash<D1> {
        IonHash {
            buffer: vec![],
            hasher_type: PhantomData,
        }
    }

    pub fn get(&self) -> &[u8] {
        &self.buffer
    }

    pub fn add_bytes(&mut self, value: &[u8]) {
        let value = IonHash::<D>::from_bytes(value);

        self.dot(&value);
    }

    pub fn add_hashed_bytes(&mut self, value: &[u8]) {
        let value = IonHash::<D>::from_hashes_bytes(value);

        self.dot(&value);
    }

    pub fn add_ion_value(&mut self, value: &IonValue) {
        let buffer = encode_value_for_hash::<D>(value);

        println!("{:X?}", buffer);

        let value = IonHash::<D>::from_bytes(&buffer);

        self.dot(&value);
    }

    pub fn dot(&mut self, value: &IonHash<D>) -> &mut Self {
        if value.buffer.is_empty() {
            return self;
        }

        let mut buffer:Vec<u8> = vec![];

        if *self < *value {
            buffer.extend(self.get());
            buffer.extend(value.get());
        } else {
            buffer.extend(value.get());
            buffer.extend(self.get());
        }

        self.buffer = D::digest(&buffer).to_vec();

        self
    }
}

impl Default for IonHash<Sha256> {
    fn default() -> IonHash<Sha256> {
        IonHash::<Sha256>::new()
    }
}

impl <D: Digest>PartialEq for IonHash<D> {
    fn eq(&self, _: &IonHash<D>) -> bool {
        self.buffer == self.get()
    }
}

impl <D: Digest>PartialOrd for IonHash<D> {
    fn partial_cmp(&self, value: &IonHash<D>) -> Option<Ordering> {
        self.buffer.iter().rev().partial_cmp(value.get().iter().rev())
    }
}
