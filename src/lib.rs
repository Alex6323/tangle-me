//! Properties of this crate:
//! * uses cache-friendly `Vec`s to represent vertices internally, transactions received relatively close to eachother will also most likely be stored close to eachother (which is why we have to be able to start the search at a certain index)
//! * one-time allocation of the `Vec`s
//! * u64 node identifiers

#![allow(dead_code)]

pub use tangle::Tangle;

mod edge;
mod id;
mod node;
mod ring;
mod tangle;

// TEMP: import bee_bundle::Transaction
pub struct TransactionHash(pub [i8; 243]);

pub struct Transaction {
    pub trunk: TransactionHash,
    pub branch: TransactionHash,
}
