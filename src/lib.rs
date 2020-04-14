mod graph;

pub use crate::graph::tangle::Tangle;

//use bee_bundle::Hash;
type Hash = u64;

#[derive(Clone, Copy, Debug)]
pub struct BundledTransaction {
    hash: Hash,        // transaction hash
    index: usize,      // position in the bundle
    last: usize,       // size of the bundle
    bundle_hash: Hash, // Kerl bundle hash
    solid: bool,       // past cone fully available and referencing solid entry points
}

impl BundledTransaction {
    pub fn size(&self) -> usize {
        self.last + 1
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
    // TODO: this function should return an iterator `BundleWalk`.
    pub fn walk_bundle(&self, bundle_tail: &'a BundledTransaction) -> Option<&'a BundledTransaction> {
        // NOTE: ensure that this function only gets called for tails
        assert_eq!(0, bundle_tail.index);
        let bundle_size = bundle_tail.size();
        let bundle_hash = bundle_tail.bundle_hash;
        let bundle_last = bundle_tail.last;

        // tail == head?
        if bundle_size == 1 {
            return Some(bundle_tail);
        }

        let mut node = bundle_tail;
        let mut index = node.index;

        for _ in 0..bundle_last {
            node = match self.get_trunk(node) {
                Some(trunk) => {
                    // NOTE: make sure all transactions in the bundle have the same bundle hash
                    if trunk.bundle_hash != bundle_hash {
                        return None;
                    }
                    // NOTE: make sure `last_index` is the same
                    if trunk.last != bundle_last {
                        return None;
                    }

                    // NOTE: make sure its `index` is increasing
                    if trunk.index != index + 1 {
                        return None;
                    }

                    index = trunk.index;
                    trunk
                }
                None => return None,
            }
        }
        Some(node)
    }

    // Tries to eagerly solidify `root` and its approvers.
    //
    // NOTE: this  method is called whenever new information arrives.
    fn try_solidify(&mut self, root: &'a mut BundledTransaction) {
        if root.solid {
            return; // already solid
        }

        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if self.get_trunk(node).filter(|n| n.solid).is_none() || self.get_branch(node).filter(|n| n.solid).is_none()
            {
                continue; // not all approvees are solid
            } else {
                //root.solid = true;
            }

            //if let Some(approvers) = self.get_approvers_mut(&node) {}
        }

        /*
        let vertices = &mut self.vertices;
        let txs_to_approvers = &self.txs_to_approvers;

        let mut stack = vec![root];
        while let Some(current_vert) = stack.pop() {
            if let Some(approvee_hashes) = vertices
                .get(&current_vert)
                .filter(|v| !v.is_solid())
                .map(|v| v.approvee_hashes())
            {
                if approvee_hashes
                    // For each of the current root's approvees...
                    .iter()
                    // ...ensure that they are all solid...
                    .all(|a| {
                        vertices.get(&a).map(|a| a.is_solid()).unwrap_or(false) || a.is_genesis()
                    })
                {
                    // We can now solidify the current root since we know all approvees are solid
                    vertices
                        .get_mut(&current_vert)
                        .unwrap() // Can't fail
                        .set_solid();
                    // Now, propagate this information to the approvers of the current root by
                    // running the algorithm again for each of them
                    for approver in txs_to_approvers
                        .get(&current_vert)
                        .iter()
                        .map(|approvers| approvers.iter())
                        .flatten()
                    {
                        // Push the approver to the stack as the next vertex to consider
                        stack.push(*approver);
                    }
                }
            }
        }
        */
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
            last: 0,
            bundle_hash: 7777,
            solid: false,
        };
        let parent_bundle_tail_b = BundledTransaction {
            hash: 2001,
            index: 0,
            last: 0,
            bundle_hash: 8888,
            solid: false,
        };
        let bundle_head = BundledTransaction {
            hash: 1002,
            index: 2,
            last: 2,
            bundle_hash: 9999,
            solid: false,
        };
        let bundle_body = BundledTransaction {
            hash: 1001,
            index: 1,
            last: 2,
            bundle_hash: 9999,
            solid: false,
        };
        let bundle_tail = BundledTransaction {
            hash: 1000,
            index: 0,
            last: 2,
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
