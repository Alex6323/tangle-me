use crate::edge::Edge;
use crate::id::NodeId;
use crate::ring::RingIterator;

//TEMP
use crate::{
    Transaction,
    TransactionHash,
};

const DEFAULT_TANGLE_CAPACITY: usize = 100000;
const DEFAULT_OVERWRITE_LENGTH: usize = 5000;

pub enum Missing {
    Both { trunk: NodeId, branch: NodeId },
    Trunk { trunk: NodeId },
    Branch { branch: NodeId },
    None,
}

pub struct Tangle {
    pub size: usize,
    pub capacity: usize,
    pub iterator: RingIterator,
    nodes: Vec<NodeId>,
    trunks: Vec<Edge>,
    branches: Vec<Edge>,
    referrers: Vec<Vec<NodeId>>,
    overwrite_len: usize,
    solid_states: Vec<bool>,
    missing_links: Vec<Missing>,
}

impl Tangle {
    pub fn new() -> Self {
        Tangle::with_capacity(DEFAULT_TANGLE_CAPACITY, DEFAULT_OVERWRITE_LENGTH)
    }

    /// NOTE: make sure that `capacity` is a multiple of `overwrite_chunk_size`, otherwise
    /// this function will panic.
    pub fn with_capacity(capacity: usize, overwrite_len: usize) -> Self {
        assert!(capacity % overwrite_len == 0);

        Self {
            size: 0,
            capacity,
            iterator: RingIterator::new(0, capacity),
            nodes: Vec::with_capacity(capacity),
            trunks: Vec::with_capacity(capacity),
            branches: Vec::with_capacity(capacity),
            referrers: Vec::with_capacity(capacity),
            overwrite_len,
            solid_states: Vec::with_capacity(capacity),
            missing_links: Vec::with_capacity(capacity),
        }
    }

    // TODO: if that files send a request to the storage layer

    pub fn append(&mut self, transaction: &Transaction, transaction_hash: &TransactionHash) -> bool {
        //
        // Check if that node already exists in the Tangle
        // Disadvantage: for new transactions, we will have to check each node (100k comparisons)
        //
        let node = NodeId::new(transaction_hash);
        if self.find_node_index_from_id(node).is_some() {
            return false;
        }

        //
        //
        //
        if self.is_full() {
            self.invalidate_oldest();
        }

        //
        //
        //
        let trunk = NodeId::new(&transaction.trunk);
        let branch = NodeId::new(&transaction.branch);

        self.trunks
            .push(if let Some(index) = self.find_node_index_from_id(trunk) {
                Edge::With { node_index: index }
            } else {
                Edge::None
            });

        // TODO: if that files send a request to the storage layer
        self.branches
            .push(if let Some(index) = self.find_node_index_from_id(branch) {
                Edge::With { node_index: index }
            } else {
                Edge::None
            });

        // TODO: try to solidify older nodes
        self.referrers.push(self.get_referrers());

        self.nodes.push(node);

        self.size += 1;
        self.iterator.next();

        true
    }

    // TODO: get_tips

    // TODO: see if searching in reverse order has benefits (assumption most of the time links should be rather near to eachother)
    fn find_node_index_from_id(&self, id: NodeId) -> Option<usize> {
        let iterator = self.iterator;

        // Iteratate backwards from current position
        // NOTE: on average this should yield the node very quickly if it exists
        for index in iterator.rev().take(iterator.size) {
            if self.nodes[index] == id {
                return Some(index);
            }
        }
        None
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn current(&self) -> usize {
        self.iterator.curr
    }

    fn invalidate_oldest(&mut self) {
        //self.len -=
    }

    fn get_referrers(&self) -> Vec<NodeId> {
        unimplemented!("TODO: implement referrers method");
    }

    fn solidify(&mut self) {}

    // TODO: dump to storage layer (async + batched)
    fn reduce(&mut self) {
        //
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

        assert_eq!(0, tangle.iterator.curr);
        assert_eq!(0, tangle.size);
        assert_eq!(DEFAULT_TANGLE_CAPACITY, tangle.capacity);
    }

    #[test]
    #[ignore]
    fn append_and_get() {
        let mut tangle = Tangle::new();

        let transaction_hash = TransactionHash([0i8; 243]);
        let transaction = Transaction {
            trunk: TransactionHash([1i8; 243]),
            branch: TransactionHash([2i8; 243]),
        };
        tangle.append(&transaction, &transaction_hash);
    }
}
