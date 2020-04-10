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
    nodes: IndexMap<N, Neighbors<'a, N>>,
}

impl<'a, N> Tangle<'a, N>
where
    N: Eq + Hash,
{
    pub fn new() -> Self {
        Self { nodes: IndexMap::new() }
    }

    pub fn add_node(&mut self, node: N) {
        self.nodes.insert(node, Neighbors::new());
    }

    pub fn add_trunk(&mut self, node: &'a N, trunk: &'a N) {
        let nodes = &mut self.nodes;

        let mut node_neighbors = nodes.get_mut(node).unwrap();

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

        let trunk_neighbors = nodes.get_mut(trunk).unwrap();
        trunk_neighbors.approvers.push(node);
    }

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

    pub fn has_edge(&self, a: &N, b: &N) -> bool {
        let a_approvees = self.nodes.get(a).expect("error").approvees.collect();
        let b_approvees = self.nodes.get(b).expect("error").approvees.collect();

        if a_approvees.is_empty() && b_approvees.is_empty() {
            return false;
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

    pub fn get_trunk(&self, node: &N) -> Result<Option<&N>, ()> {
        match self.nodes.get(node) {
            None => Err(()),
            Some(neighbors) => match neighbors.approvees {
                Approvees::None | Approvees::Branch(_) => Ok(None),
                Approvees::Trunk(trunk) | Approvees::Both(trunk, _) => Ok(Some(trunk)),
            },
        }
    }

    pub fn get_branch(&self, node: &N) -> Result<Option<&N>, ()> {
        match self.nodes.get(node) {
            None => Err(()),
            Some(neighbors) => match neighbors.approvees {
                Approvees::None | Approvees::Trunk(_) => Ok(None),
                Approvees::Branch(branch) | Approvees::Both(_, branch) => Ok(Some(branch)),
            },
        }
    }

    pub fn approvers(&self, node: &N) -> Result<impl Iterator<Item = &&N>, ()> {
        match self.nodes.get(node) {
            None => Err(()),
            Some(neighbors) => Ok(neighbors.approvers.iter()),
        }
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
    fn insert_and_contains() {
        let mut tangle = Tangle::new();

        let a = Node::new(0);
        tangle.add_node(a);

        assert!(!tangle.is_empty());
        assert_eq!(1, tangle.size());
        assert!(tangle.contains(&a));

        let b = Node::new(1);
        tangle.add_node(b);

        assert!(!tangle.is_empty());
        assert_eq!(2, tangle.size());
        assert!(tangle.contains(&b));
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

        assert_eq!(3, tangle.approvers(&a).unwrap().count());
        assert_eq!(2, tangle.approvers(&b).unwrap().count());
    }
}
