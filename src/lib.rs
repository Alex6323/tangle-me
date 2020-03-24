//! Properties of this crate:
//! * uses HashMap's
//! * u64 node identifiers

#![allow(dead_code)]

pub use tangle::Tangle;

mod edge;
mod tangle;
mod vertex;

use trits_module_preview::T1B1;

// TEMP: import bee_bundle::Transaction
pub struct TransactionHash(pub [i8; 243]);

pub struct Transaction {
    pub trunk: TransactionHash,
    pub branch: TransactionHash,
}
