use crate::TransactionHash;

/// T5B1 encoded first 13 Trytes of a `TransactionHash`, that just fits into `u64`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NodeId(u64);

impl NodeId {
    pub fn new(_hash: &TransactionHash) -> Self {
        // 5 trits * 8 = 40 trits ~ 13 Trytes
        // the first 13 Trytes would need to be equal for a hash collision (TANGLE_CAPACITY/3^40 or TANGLE_CAPACITY/10^19) which is **very** unlikely to happen for an in-memory Tangle
        let hash = 0;

        Self(hash)
    }
}
