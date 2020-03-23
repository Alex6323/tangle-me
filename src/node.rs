use crate::edge::Edge;
use crate::id::NodeId;

pub struct Node {
    pub id: NodeId,             //u64
    pub trunk: Edge,            //u64
    pub branch: Edge,           //u64
    pub referrers: Vec<NodeId>, //u64...
    pub solid: bool,            //bool
    pub last_access: u64,       //u64
}

impl Node {
    pub fn new(id: NodeId, solid: bool, last_access: u64) -> Self {
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

impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn new_node() {}
}
