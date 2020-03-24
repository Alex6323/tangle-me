use crate::edge::Edge;

use crate::TransactionHash;

/// T5B1 encoded first 13 Trytes of a `TransactionHash`, that just fits into `u64`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct VertexId(u64);

impl VertexId {
    pub fn new(_hash: &TransactionHash) -> Self {
        // 5 trits * 8 = 40 trits ~ 13 Trytes
        // the first 13 Trytes would need to be equal for a hash collision (TANGLE_CAPACITY/3^40 or TANGLE_CAPACITY/10^19) which is **very** unlikely to happen for an in-memory Tangle
        let hash = 0;

        Self(hash)
    }
}

pub struct Vertex {
    pub id: VertexId,             //u64
    pub trunk: Edge,              //u64
    pub branch: Edge,             //u64
    pub referrers: Vec<VertexId>, //u64...
    pub solid: bool,              //bool
    pub last_access: u64,         //u64
}

impl Vertex {
    pub fn new(id: VertexId, solid: bool, last_access: u64) -> Self {
        Self {
            id,
            trunk: Edge::None,
            branch: Edge::None,
            referrers: vec![],
            solid,
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
    use super::*;

    #[test]
    fn new_vertex() {
        //let id = VertexId::new(_hash)
        //let _ = Vertex::new(id, solid, last_access)
    }
}
