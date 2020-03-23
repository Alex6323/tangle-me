use crate::id::NodeId;

#[derive(Debug, Eq, PartialEq)]
pub enum Edge {
    With { id: NodeId },
    None,
}
