use crate::vertex::VertexId;

#[derive(Debug, Eq, PartialEq)]
pub enum Edge {
    With { id: VertexId },
    None,
}
