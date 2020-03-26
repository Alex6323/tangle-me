use crate::edge::Edge;
use crate::id::TransactionId;
use crate::milestone::MilestoneIndex;
use crate::vertex::Vertex;

//TEMP
use crate::{
    Transaction,
    TransactionHash,
};

use std::collections::{
    HashMap,
    HashSet,
};

const DEFAULT_TANGLE_CAPACITY: usize = 100000;
const DEFAULT_FREE_SIZE: usize = 5000; // used to remove a number of vertices with lowest `last_access` numbers
const DEFAULT_AWAITED_CAPACITY: usize = 1000; // TODO: what's a good value here?
const DEFAULT_SEP_CAPACITY: usize = 5000; // TODO: find good value
const DEFAULT_MS_CAPACITY: usize = 5000; // TODO: find good value

#[derive(Debug)]
pub enum FailureCause {
    TransactionAlreadyInserted,
    MilestoneAlreadyInserted,
    TransactionDoesNotExist { hash: TransactionHash },
    VertexDoesNotExist { id: TransactionId },
}

#[derive(Debug)]
pub enum TangleError {
    InsertionFailure { cause: FailureCause },
    RetreiveFailure { cause: FailureCause },
}

pub type Result<T> = std::result::Result<T, TangleError>;
pub type VoidResult = Result<()>;

use self::Result as TRes;
use self::VoidResult as Res;

use FailureCause::*;
use TangleError::*;

pub struct Tangle {
    /// Total capacity of the Tangle.
    pub capacity: usize,

    /// Holds all the vertices of the Tangle.
    vertices: HashMap<TransactionId, Vertex>,

    /// List of awaited vertices other vertices wanted to add as trunk or branch.
    awaited: HashMap<TransactionId, Vec<TransactionId>>,

    /// List of solid entry points.
    solid_entry_points: HashSet<TransactionId>,

    /// Transaction cache.
    transactions: HashMap<TransactionId, (TransactionHash, Transaction)>,

    /// Milestone cache.
    milestones: HashMap<MilestoneIndex, TransactionId>,

    /// Used to update `last_access` on a vertex.
    counter: u64,

    /// The number of vertices removed during a Tangle reduction procedure.
    free_size: usize,
}

impl Tangle {
    pub fn new() -> Self {
        Tangle::with_capacity(DEFAULT_TANGLE_CAPACITY, DEFAULT_FREE_SIZE)
    }

    pub fn with_capacity(capacity: usize, free_size: usize) -> Self {
        Self {
            capacity,
            vertices: HashMap::with_capacity(capacity),
            counter: 0,
            awaited: HashMap::with_capacity(DEFAULT_AWAITED_CAPACITY),
            solid_entry_points: HashSet::with_capacity(DEFAULT_SEP_CAPACITY),
            transactions: HashMap::with_capacity(DEFAULT_TANGLE_CAPACITY),
            milestones: HashMap::with_capacity(DEFAULT_MS_CAPACITY),
            free_size,
        }
    }

    pub fn insert_transaction(&mut self, hash: &TransactionHash, transaction: &Transaction) -> Res {
        let new_id = TransactionId::new(hash);
        if self.contains(new_id) {
            return Err(InsertionFailure {
                cause: TransactionAlreadyInserted,
            });
        }

        if self.is_full() {
            self.reduce_size();
        }

        let mut vertex = Vertex::new(new_id, self.counter);

        let trunk_id = TransactionId::new(&transaction.trunk);
        let branch_id = TransactionId::new(&transaction.branch);

        vertex.trunk = if let Some(trunk) = self.vertices.get_mut(&trunk_id) {
            trunk.referrers.push(new_id);
            Edge::With { id: trunk_id }
        } else {
            self.add_awaited(trunk_id, new_id);
            Edge::None
        };

        vertex.branch = if let Some(branch) = self.vertices.get_mut(&branch_id) {
            branch.referrers.push(new_id);
            Edge::With { id: branch_id }
        } else {
            self.add_awaited(branch_id, new_id);
            Edge::None
        };

        // NOTE: the awaited transaction arrived
        if let Some(referrers) = self.awaited.remove(&new_id) {
            vertex.referrers = referrers;

            // TODO: update the referrer's trunk and branch
        }

        self.vertices.insert(new_id, vertex);

        self.counter += 1;

        Ok(())
    }

    pub fn get_transaction(&self, _hash: &TransactionHash) -> Option<&Transaction> {
        unimplemented!("get_transaction")
    }

    pub fn insert_milestone(&mut self, _index: MilestoneIndex, _hash: TransactionHash) -> Res {
        unimplemented!("insert_milestone")
    }

    pub fn get_milestone(&self, _index: MilestoneIndex) -> Option<TransactionHash> {
        unimplemented!("get_milestone")
    }

    pub fn get_latest_milestone(&self) -> Option<TransactionHash> {
        unimplemented!("get_latest_milestone")
    }

    pub fn add_solid_entry_point(&mut self, hash: TransactionHash) -> Res {
        //unimplemented!("add_solid_entry_point")
        let _id: TransactionId = hash.into();

        //self.solid_entry_points.get_or_insert(value)
        Ok(())
    }

    pub fn is_solid_entry_point(&self, _hash: TransactionHash) -> TRes<bool> {
        unimplemented!("is_solid_entry_point")
    }

    pub fn contains(&self, id: TransactionId) -> bool {
        self.vertices.contains_key(&id)
    }

    pub fn is_full(&self) -> bool {
        self.size() == self.capacity
    }

    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    fn reduce_size(&mut self) {
        // 1) Iterate all vertices and collect `DEFAULT_REDUCE_SIZE` IDs of least accessed vertices
        // 2) Remove associated vertices from list
        // 3) Remove associated transactions from store
        unimplemented!("reduce_size")
    }

    fn add_awaited(&mut self, _awaited_id: TransactionId, _by: TransactionId) {
        // TODO: use Entry-API to update self.awaited
        unimplemented!("add_awaited")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Transaction,
        TransactionHash,
    };
    use minitri::T3B1;

    #[test]
    fn new() {
        let tangle = Tangle::new();

        assert_eq!(0, tangle.size());
        assert_eq!(DEFAULT_TANGLE_CAPACITY, tangle.capacity);
    }

    #[test]
    fn insert_and_get() {
        let mut tangle = Tangle::new();
        let trytes: T3B1 = "TANGLE9OLE".into();
        let hash = TransactionHash(trytes);
        let transaction = Transaction {
            trunk: TransactionHash("TRUNK9HASH".into()),
            branch: TransactionHash("BRANCH9HASH".into()),
        };
        assert!(
            tangle.insert_transaction(&hash, &transaction).is_ok(),
            "insertion failed"
        );
    }
}
