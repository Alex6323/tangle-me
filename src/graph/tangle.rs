use indexmap::IndexMap;
use smallvec::{
    smallvec,
    SmallVec,
};

use std::hash::Hash;

pub enum Approvees<'a, N> {
    None,
    Trunk(&'a N),
    Branch(&'a N),
    Both(&'a N, &'a N),
}

impl<'a, N> Approvees<'a, N> {
    // TODO: maybe return an iterable
    fn collect(&self) -> SmallVec<[&'a N; 2]> {
        use Approvees::*;
        match *self {
            None => smallvec![],
            Trunk(n) | Branch(n) => smallvec![n],
            Both(n1, n2) => smallvec![n1, n2],
        }
    }
}

pub struct Neighbors<'a, N> {
    approvees: Approvees<'a, N>,
    approvers: Vec<&'a N>,
}

impl<'a, N> Neighbors<'a, N> {
    pub fn new() -> Self {
        Self {
            approvees: Approvees::None,
            approvers: vec![],
        }
    }

    pub fn degree_in(&self) -> usize {
        self.approvers.len()
    }

    #[allow(dead_code)]
    pub fn degree_out(&self) -> usize {
        match self.approvees {
            Approvees::None => 0,
            Approvees::Trunk(_) | Approvees::Branch(_) => 1,
            Approvees::Both(_, _) => 2,
        }
    }

    pub fn is_tip(&self) -> bool {
        self.degree_in() == 0
    }
}

pub struct Tangle<'a, N>
where
    N: Eq + Hash,
{
    // TODO: use NodeMeta { node: N, solid: bool } as key
    nodes: IndexMap<N, Neighbors<'a, N>>,
}

impl<'a, N> Tangle<'a, N>
where
    N: Eq + Hash,
{
    pub fn new() -> Self {
        Self { nodes: IndexMap::new() }
    }

    pub fn add_node(&mut self, node: N) -> usize {
        let (index, _) = self.nodes.insert_full(node, Neighbors::new());
        index
    }

    pub fn remove_node(&mut self, index: usize) -> Option<N> {
        match self.nodes.swap_remove_index(index) {
            None => None,
            Some((node, neighbors)) => {
                /*
                let approvers = neighbors.approvers;
                for node in approvers {
                    //self.get
                }
                */
                Some(node)
            }
        }
    }

    pub fn get_node(&'a self, index: usize) -> Option<&'a N> {
        self.nodes.get_index(index).map(|(node, _)| node)
    }

    pub fn get_node_mut(&'a mut self, index: usize) -> Option<&'a mut N> {
        self.nodes.get_index_mut(index).map(|(node, _)| node)
    }

    pub fn get_neighbors(&'a self, index: usize) -> Option<&'a Neighbors<'a, N>> {
        self.nodes.get_index(index).map(|(_, neighbors)| neighbors)
    }

    pub fn get_neighbors_mut(&'a mut self, index: usize) -> Option<&'a mut Neighbors<'a, N>> {
        self.nodes.get_index_mut(index).map(|(_, neighbors)| neighbors)
    }

    pub fn add_trunk(&mut self, node: &'a N, trunk: &'a N) {
        let nodes = &mut self.nodes;

        // TODO: fix unwrap
        let mut node_neighbors = nodes.get_mut(node).unwrap();

        // TODO: return Result if trunk already set
        let has_branch = match node_neighbors.approvees {
            Approvees::None => None,
            Approvees::Branch(n) => Some(n),
            _ => panic!("Trunk already set"),
        };

        if let Some(branch) = has_branch {
            node_neighbors.approvees = Approvees::Both(trunk, branch);
        } else {
            node_neighbors.approvees = Approvees::Trunk(trunk);
        }

        // TODO: fix unwrap
        let trunk_neighbors = nodes.get_mut(trunk).unwrap();
        trunk_neighbors.approvers.push(node);
    }

    // TODO: same fixes as `add_trunk`
    pub fn add_branch(&mut self, node: &'a N, branch: &'a N) {
        let nodes = &mut self.nodes;

        let mut node_neighbors = nodes.get_mut(node).unwrap();

        let has_trunk = match node_neighbors.approvees {
            Approvees::None => None,
            Approvees::Trunk(n) => Some(n),
            _ => panic!("Branch already set"),
        };

        if let Some(trunk) = has_trunk {
            node_neighbors.approvees = Approvees::Both(trunk, branch);
        } else {
            node_neighbors.approvees = Approvees::Branch(branch);
        }

        let branch_neighbors = nodes.get_mut(branch).unwrap();
        branch_neighbors.approvers.push(node);
    }

    pub fn has_edge(&self, a: &'a N, b: &'a N) -> bool {
        let a_approvees = self.nodes.get(a).expect("error").approvees.collect();
        let b_approvees = self.nodes.get(b).expect("error").approvees.collect();

        if a_approvees.is_empty() && b_approvees.is_empty() {
            false
        } else {
            for n in a_approvees {
                if n == b {
                    return true;
                }
            }
            for n in b_approvees {
                if n == a {
                    return true;
                }
            }
            false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn contains(&self, node: &N) -> bool {
        self.nodes.contains_key(node)
    }

    pub fn contains_index(&self, index: usize) -> bool {
        self.nodes.get_index(index).is_some()
    }

    pub fn get_trunk(&self, node: &'a N) -> Option<&'a N> {
        match self.nodes.get(node) {
            None => None,
            Some(neighbors) => match neighbors.approvees {
                Approvees::None | Approvees::Branch(_) => None,
                Approvees::Trunk(trunk) | Approvees::Both(trunk, _) => Some(trunk),
            },
        }
    }

    pub fn get_branch(&self, node: &'a N) -> Option<&'a N> {
        match self.nodes.get(node) {
            None => None,
            Some(neighbors) => match neighbors.approvees {
                Approvees::None | Approvees::Trunk(_) => None,
                Approvees::Branch(branch) | Approvees::Both(_, branch) => Some(branch),
            },
        }
    }

    pub fn get_approvees(&self, node: &'a N) -> Option<impl Iterator<Item = &'a N>> {
        match self.nodes.get(node) {
            None => None,
            Some(neighbors) => Some(neighbors.approvees.collect().into_iter()),
        }
    }

    pub fn get_approvers(&self, node: &'a N) -> Option<impl Iterator<Item = &&'a N>> {
        self.nodes.get(node).map(|neighbors| neighbors.approvers.iter())
    }

    // TEMPORARY: instead of iterating all the nodes, keep a synced set of tips
    pub fn tips(&self) -> impl Iterator<Item = &N> {
        self.nodes
            .iter()
            .filter_map(|(n, v)| if v.is_tip() { Some(n) } else { None })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
    pub struct Node {
        value: u8,
    }

    impl Node {
        pub fn new(value: u8) -> Self {
            Node { value }
        }
    }

    #[test]
    fn new_tangle() {
        let tangle: Tangle<Node> = Tangle::new();

        assert!(tangle.is_empty());
        assert_eq!(0, tangle.size());
        assert_eq!(None, tangle.tips().next());
    }

    #[test]
    fn add_and_contains() {
        let mut tangle = Tangle::new();

        let a = Node::new(0);
        let i = tangle.add_node(a);

        assert!(!tangle.is_empty());
        assert_eq!(1, tangle.size());
        assert_eq!(0, i);
        assert!(tangle.contains_index(i));

        let b = Node::new(1);
        let j = tangle.add_node(b);

        assert!(!tangle.is_empty());
        assert_eq!(2, tangle.size());
        assert_eq!(1, j);
        assert!(tangle.contains_index(i));
        assert!(tangle.contains_index(j));
    }

    #[test]
    fn add_and_get() {
        let mut tangle = Tangle::new();

        let a = Node::new(0);
        let i = tangle.add_node(a);

        let b = *tangle.get_node(i).expect("get_node");
        assert_eq!(a, b);
    }

    #[test]
    fn add_and_remove() {
        let mut tangle = Tangle::new();

        let a = Node::new(0);
        let i = tangle.add_node(a);
        assert_eq!(1, tangle.size());

        let b = tangle.remove_node(i).expect("remove_node");
        assert_eq!(0, tangle.size());
        assert_eq!(a, b);
    }

    #[test]
    fn add_trunk() {
        let mut tangle = Tangle::new();

        let a = Node::new(0);
        let b = Node::new(1);

        tangle.add_node(a);
        tangle.add_node(b);

        tangle.add_trunk(&a, &b);

        assert!(tangle.has_edge(&a, &b));
        assert!(tangle.has_edge(&b, &a));
    }

    #[test]
    fn add_branch() {
        let mut tangle = Tangle::new();

        let a = Node::new(0);
        let b = Node::new(1);

        tangle.add_node(a);
        tangle.add_node(b);

        tangle.add_branch(&a, &b);

        assert!(tangle.has_edge(&a, &b));
        assert!(tangle.has_edge(&b, &a));
    }

    #[test]
    fn iter_tips() {
        let mut tangle = Tangle::new();

        let a = Node::new(0);
        let b = Node::new(1);
        let c = Node::new(2);
        let d = Node::new(3);
        let e = Node::new(4);
        let f = Node::new(5);

        tangle.add_node(a);
        tangle.add_node(b);
        tangle.add_node(c);
        tangle.add_node(d);
        tangle.add_node(e);
        tangle.add_node(f);

        let mut num_tips = 0;
        for _ in tangle.tips() {
            num_tips += 1;
        }
        assert_eq!(6, num_tips);

        tangle.add_trunk(&c, &a);
        tangle.add_branch(&c, &b);

        let mut num_tips = 0;
        for _ in tangle.tips() {
            num_tips += 1;
        }
        assert_eq!(4, num_tips);
    }

    #[test]
    fn iter_approvers() {
        let mut tangle = Tangle::new();

        let a = Node::new(0);
        let b = Node::new(1);
        let c = Node::new(2);
        let d = Node::new(3);
        let e = Node::new(4);

        tangle.add_node(a);
        tangle.add_node(b);
        tangle.add_node(c);
        tangle.add_node(d);
        tangle.add_node(e);

        tangle.add_trunk(&c, &a);
        tangle.add_trunk(&d, &a);
        tangle.add_trunk(&e, &a);

        tangle.add_branch(&c, &b);
        tangle.add_branch(&d, &b);

        assert_eq!(3, tangle.get_approvers(&a).unwrap().count());
        assert_eq!(2, tangle.get_approvers(&b).unwrap().count());
    }
}
