use crate::edge::Edge;
use crate::id::NodeId;
use crate::node::Node;

//TEMP
use crate::{
    Transaction,
    TransactionHash,
};

use std::collections::HashMap;

const DEFAULT_TANGLE_CAPACITY: usize = 100000;
const DEFAULT_REDUCE_SIZE: usize = 5000; // used to remove a number of nodes with lowest `last_access` numbers
const DEFAULT_AWAITED_CAPACITY: usize = 1000; // TODO: what's a good value here?

pub struct Tangle {
    /// Total capacity of the Tangle.
    pub capacity: usize,

    /// Holds all the nodes of the Tangle.
    nodes: HashMap<NodeId, Node>,

    /// Used to update `last_access` on a node.
    counter: u64,

    /// List of awaited nodes other nodes wanted to add as trunk or branch.
    awaited: HashMap<NodeId, Vec<NodeId>>,

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
            nodes: HashMap::with_capacity(capacity),
            counter: 0,
            awaited: HashMap::with_capacity(DEFAULT_AWAITED_CAPACITY),
            reduce_size,
        }
    }

    // TODO: if that files send a request to the storage layer

    pub fn append(&mut self, transaction: &Transaction, transaction_hash: &TransactionHash, solid: bool) -> bool {
        let new_id = NodeId::new(transaction_hash);
        if self.contains(new_id) {
            return false;
        }

        if self.is_full() {
            self.reduce_size();
        }

        let mut node = Node::new(new_id, solid, self.counter);

        let trunk_id = NodeId::new(&transaction.trunk);
        let branch_id = NodeId::new(&transaction.branch);

        node.trunk = if let Some(trunk) = self.nodes.get_mut(&trunk_id) {
            trunk.referrers.push(new_id);
            Edge::With { id: trunk_id }
        } else {
            self.add_awaited(trunk_id, new_id);
            Edge::None
        };

        node.branch = if let Some(branch) = self.nodes.get_mut(&branch_id) {
            branch.referrers.push(new_id);
            Edge::With { id: branch_id }
        } else {
            self.add_awaited(branch_id, new_id);
            Edge::None
        };

        if let Some(referrers) = self.awaited.remove(&new_id) {
            node.referrers = referrers;
        }

        self.nodes.insert(new_id, node);

        self.counter += 1;

        true
    }

    pub fn contains(&self, id: NodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    pub fn is_full(&self) -> bool {
        self.size() == self.capacity
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    fn reduce_size(&mut self) {
        // 1) Iterate all nodes and collect `DEFAULT_REDUCE_SIZE` IDs of least accessed nodes
        // 2) Remove associated nodes from list
    }

    fn add_awaited(&mut self, awaited_id: NodeId, by: NodeId) {
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
