use crate::edge::Edge;
use crate::vertex::{
    Vertex,
    VertexId,
};

//TEMP
use crate::{
    Transaction,
    TransactionHash,
};

use std::collections::HashMap;

const DEFAULT_TANGLE_CAPACITY: usize = 100000;
const DEFAULT_REDUCE_SIZE: usize = 5000; // used to remove a number of nodes with lowest `last_access` numbers
const DEFAULT_AWAITED_CAPACITY: usize = 1000; // TODO: what's a good value here?

//.insert_transaction(Hash, Transaction)
//.get_transaction(Hash)
//.insert_milestone(Index, Hash), .get_milestone(Index), .get_latest_milestone()
//.add_sep(Hash),
//.is_sep(Hash)

pub struct Tangle {
    /// Total capacity of the Tangle.
    pub capacity: usize,

    /// Holds all the nodes of the Tangle.
    vertices: HashMap<VertexId, Vertex>,

    /// Used to update `last_access` on a node.
    counter: u64,

    /// List of awaited nodes other nodes wanted to add as trunk or branch.
    awaited: HashMap<VertexId, Vec<VertexId>>,

    /// The number of nodes removed during a Tangle reduction procedure.
    reduce_size: usize,
}

impl Tangle {
    pub fn new() -> Self {
        Tangle::with_capacity(DEFAULT_TANGLE_CAPACITY, DEFAULT_REDUCE_SIZE)
    }

    pub fn with_capacity(capacity: usize, reduce_size: usize) -> Self {
        Self {
            capacity,
            vertices: HashMap::with_capacity(capacity),
            counter: 0,
            awaited: HashMap::with_capacity(DEFAULT_AWAITED_CAPACITY),
            reduce_size,
        }
    }

    // TODO: if that files send a request to the storage layer

    pub fn append(&mut self, transaction: &Transaction, transaction_hash: &TransactionHash, solid: bool) -> bool {
        // Prevent duplicates
        let new_id = VertexId::new(transaction_hash);
        if self.contains(new_id) {
            return false;
        }

        // Make some space if the Tangle has reached its capacity
        if self.is_full() {
            self.reduce_size();
        }

        let mut vertex = Vertex::new(new_id, solid, self.counter);

        let trunk_id = VertexId::new(&transaction.trunk);
        let branch_id = VertexId::new(&transaction.branch);

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

        if let Some(referrers) = self.awaited.remove(&new_id) {
            vertex.referrers = referrers;
        }

        self.vertices.insert(new_id, vertex);

        self.counter += 1;

        true
    }

    pub fn contains(&self, id: VertexId) -> bool {
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
    }

    fn add_awaited(&mut self, awaited_id: VertexId, by: VertexId) {
        // TODO: use Entry-API to update self.awaited
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Transaction,
        TransactionHash,
    };

    #[test]
    fn new() {
        let tangle = Tangle::new();

        assert_eq!(0, tangle.size());
        assert_eq!(DEFAULT_TANGLE_CAPACITY, tangle.capacity);
    }

    #[test]
    fn append_and_get() {
        let mut tangle = Tangle::new();

        let transaction_hash = TransactionHash([0i8; 243]);
        let transaction = Transaction {
            trunk: TransactionHash([1i8; 243]),
            branch: TransactionHash([2i8; 243]),
        };
        tangle.append(&transaction, &transaction_hash, false);
    }
}
