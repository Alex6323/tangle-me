//! Properties of this crate:
//! * uses HashMap's
//! * u64 node identifiers

#![allow(dead_code)]

pub use tangle::Tangle;

use minitri::{
    Encoding,
    T3B1,
    T5B1,
};

mod edge;
mod id;
mod milestone;
mod tangle;
mod vertex;

use std::fmt;

pub struct TransactionHash(pub T3B1);

impl TransactionHash {
    // TEMP: this is less secure but enough for prototyping
    pub fn get_t3b1_id(&self) -> u64 {
        assert!(self.0.len() >= 8);

        let mut id = 0u64;
        for i in 0..8 {
            id |= (self.0.get_as_i8(i) as u64) << i;
        }
        id
    }
    // TODO: implement this function to achieve better collision resistence
    // NOTE: 5 trits/byte * 8 = 40 trits or ~13 trytes per u64, which should
    // be collision resitent enough for an in-memory Tangle.
    pub fn get_t5b1_id(&self) -> u64 {
        let t5b1: T5B1 = self.0.clone().into();
        assert!(t5b1.len() >= 8);

        let mut id = 0u64;
        for i in 0..8 {
            id |= (self.0.get_as_i8(i) as u64) << i;
        }
        id
    }
}

// convert the first 8 trytes to `TransactionHash` to T5B1 encoding
impl fmt::Debug for TransactionHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct Transaction {
    pub trunk: TransactionHash,
    pub branch: TransactionHash,
}
