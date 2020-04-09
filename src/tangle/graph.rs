use indexmap::IndexMap;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct Node {
    value: u8,
}

impl Node {
    pub fn new(value: u8) -> Self {
        Node { value }
    }
}

pub enum Approvees<'a> {
    None,
    Trunk(&'a Node),
    Branch(&'a Node),
    Both(&'a Node, &'a Node),
}

impl<'a> Approvees<'a> {
    fn collect(&self) -> Vec<&'a Node> {
        use Approvees::*;
        match *self {
            None => vec![],
            Trunk(n) | Branch(n) => vec![n],
            Both(n1, n2) => vec![n1, n2],
        }
    }
}

pub struct Neighbors<'a> {
    approvees: Approvees<'a>,
    approvers: Vec<&'a Node>,
}

impl<'a> Neighbors<'_> {
    pub fn new() -> Self {
        Self {
            approvees: Approvees::None,
            approvers: vec![],
        }
    }
}

pub struct Tangle<'a> {
    nodes: IndexMap<Node, Neighbors<'a>>,
    edges: Vec<(&'a Node, &'a Node)>,
}

impl<'a> Tangle<'a> {
    pub fn new() -> Self {
        Self {
            nodes: IndexMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node, Neighbors::new());
    }

    pub fn add_trunk(&mut self, node: &'a Node, trunk: &'a Node) {
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

    pub fn add_branch(&mut self, node: &'a Node, branch: &'a Node) {
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

    pub fn has_edge(&self, a: &Node, b: &Node) -> bool {
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

    pub fn order(&self) -> usize {
        self.nodes.len()
    }

    pub fn size(&self) -> usize {
        self.edges.len()
    }

    /*
    pub fn nodes(&'a self) -> impl Iterator<Item = Node> {
        todo!()
    }

    fn has_node(&self, node: &N) -> bool;

    /// Returns the number of neighbors connected to node.
    fn degree_in(&self, node: &N) -> usize;

    /// Returns the number of neighbors connected to node.
    fn degree_out(&self, node: &N) -> usize;

    /// Returns true if an edge exists between source and target.
    fn has_edge(&self, source: &N, target: &N) -> bool;
    */
}
