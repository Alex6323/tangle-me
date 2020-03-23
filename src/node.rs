use crate::id::NodeId;

#[derive(Copy, Clone, Debug)]
pub struct Node {
    pub(crate) hash: NodeId,
    pub(crate) trunk: NodeId,
    pub(crate) branch: NodeId,
}

impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}

impl Node {
    pub fn from(hash: NodeId, trunk: NodeId, branch: NodeId) -> Self {
        Self { hash, trunk, branch }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn new_node() {}
}
