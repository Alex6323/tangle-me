use tangle::Tangle;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct Node {
    value: u8,
}

impl Node {
    pub fn new(value: u8) -> Self {
        Node { value }
    }
}

fn main() {
    let mut tangle = Tangle::new();

    let a = Node::new(0);
    let b = Node::new(1);
    let c = Node::new(2);

    tangle.add_node(a);
    tangle.add_node(b);
    tangle.add_node(c);

    tangle.add_trunk(&c, &a);
    tangle.add_branch(&c, &b);

    assert!(tangle.has_edge(&c, &a));
    assert!(tangle.has_edge(&a, &c));
    assert!(tangle.has_edge(&c, &b));
    assert!(tangle.has_edge(&b, &c));
    assert!(!tangle.has_edge(&a, &b));
    assert!(!tangle.has_edge(&b, &a));
}
