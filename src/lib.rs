mod graph;

pub use crate::graph::tangle::Tangle;

//use bee_bundle::Hash;
type Hash = u64;

#[derive(Clone, Copy, Debug)]
pub struct BundledTransaction {
    hash: Hash,        // transaction hash
    index: usize,      // position in the bundle
    last_index: usize, // size of the bundle
    bundle_hash: Hash, // Kerl bundle hash
    solid: bool,       // past cone fully available and referencing solid entry points
}

impl BundledTransaction {
    pub fn size(&self) -> usize {
        self.last_index + 1
    }
}

impl Eq for BundledTransaction {}
impl PartialEq for BundledTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}
impl std::hash::Hash for BundledTransaction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl<'a> Tangle<'a, BundledTransaction> {
    //
    pub fn walk_bundle(&self, bundle_tail: &'a BundledTransaction) -> Option<&'a BundledTransaction> {
        // NOTE: ensure that this function only gets called for tails
        assert_eq!(0, bundle_tail.index);
        let bundle_size = bundle_tail.size();

        // tail == head
        if bundle_size == 1 {
            return Some(bundle_tail);
        }

        let mut node = bundle_tail;
        for _ in 0..bundle_tail.last_index {
            node = match self.get_trunk(node) {
                Ok(has_trunk) => match has_trunk {
                    Some(trunk) => trunk,
                    None => return None,
                },
                Err(()) => return None,
            }
        }
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_bundle() {
        let mut tangle: Tangle<BundledTransaction> = Tangle::new();

        let parent_bundle_tail_a = BundledTransaction {
            hash: 2000,
            index: 0,
            last_index: 0,
            bundle_hash: 7777,
            solid: false,
        };
        let parent_bundle_tail_b = BundledTransaction {
            hash: 2001,
            index: 0,
            last_index: 0,
            bundle_hash: 8888,
            solid: false,
        };
        let bundle_head = BundledTransaction {
            hash: 1002,
            index: 2,
            last_index: 2,
            bundle_hash: 9999,
            solid: false,
        };
        let bundle_body = BundledTransaction {
            hash: 1001,
            index: 1,
            last_index: 2,
            bundle_hash: 9999,
            solid: false,
        };
        let bundle_tail = BundledTransaction {
            hash: 1000,
            index: 0,
            last_index: 2,
            bundle_hash: 9999,
            solid: false,
        };

        // Let's add the nodes in the order that might have happened
        tangle.add_node(parent_bundle_tail_a);
        tangle.add_node(parent_bundle_tail_b);
        tangle.add_node(bundle_head);
        tangle.add_node(bundle_body);
        tangle.add_node(bundle_tail);

        tangle.add_trunk(&bundle_head, &parent_bundle_tail_a);
        tangle.add_branch(&bundle_head, &parent_bundle_tail_b);

        tangle.add_trunk(&bundle_body, &bundle_head);
        tangle.add_branch(&bundle_body, &parent_bundle_tail_a);

        tangle.add_trunk(&bundle_tail, &bundle_body);
        tangle.add_branch(&bundle_tail, &parent_bundle_tail_a);

        assert_eq!(&bundle_head, tangle.walk_bundle(&bundle_tail).unwrap());
    }
}
