use crate::edge::Edge;
use crate::id::TransactionId;

pub struct Vertex {
    pub id: TransactionId,             //u64
    pub trunk: Edge,                   //u64
    pub branch: Edge,                  //u64
    pub referrers: Vec<TransactionId>, //u64...
    pub solid: bool,                   //bool
    pub last_access: u64,              //u64
}

impl Vertex {
    pub fn new(id: TransactionId, last_access: u64) -> Self {
        Self {
            id,
            trunk: Edge::None,
            branch: Edge::None,
            referrers: vec![],
            solid: false,
            last_access,
        }
    }

    fn has_trunk(&self) -> bool {
        self.trunk != Edge::None
    }

    fn has_branch(&self) -> bool {
        self.branch != Edge::None
    }

    fn has_referrers(&self) -> bool {
        self.referrers.len() > 0
    }
}

impl Eq for Vertex {}
impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn new_vertex() {
        //let id = VertexId::new(_hash)
        //let _ = Vertex::new(id, solid, last_access)
    }
}
