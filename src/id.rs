//! For performance reasons we introduce `TransactionId` to HashMap doesn't have to rehash `TransactionHash`.
use crate::TransactionHash;

/// T5B1 encoded first 13 Trytes of a `TransactionHash`, that just fits into `u64`.
/// NOTE: the vertix_id is the first 13 trytes of a transaction hash which fits into an `u64`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TransactionId(u64);

impl TransactionId {
    pub fn new(_hash: &TransactionHash) -> Self {
        // 5 trits * 8 = 40 trits ~ 13 Trytes
        // the first 13 Trytes would need to be equal for a hash collision (TANGLE_CAPACITY/3^40 or TANGLE_CAPACITY/10^19) which is **very** unlikely to happen for an in-memory Tangle
        let hash = 0;

        Self(hash)
    }
}

impl From<TransactionHash> for TransactionId {
    fn from(_hash: TransactionHash) -> Self {
        // TODO: derive ID from hash
        Self(0)
    }
}
