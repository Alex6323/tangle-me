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

    #[allow(dead_code)]
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
}

pub struct Tangle<'a> {
    nodes: IndexMap<Node, Neighbors<'a>>,
}

impl<'a> Tangle<'a> {
    pub fn new() -> Self {
        Self { nodes: IndexMap::new() }
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

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn contains(&self, node: &Node) -> bool {
        self.nodes.contains_key(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tangle() {
        let tangle = Tangle::new();

        assert!(tangle.is_empty());
        assert_eq!(0, tangle.size());
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
}
