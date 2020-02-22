use crate::data::Data;

use std::cell::RefCell;
use std::rc::Rc;

type Link = Rc<RefCell<Vertex>>;

pub struct Vertex {

    /// Trunk reference (past cone)
    trunk: Option<Link>,

    /// Branch reference (past cone)
    branch: Option<Link>,

    /// Referrers (future cone)
    referrers: Vec<Link>,

    /// The data referenced by this vertex.
    data: Data,
}

impl Vertex {
    pub fn new() -> Self {
        Self {
            trunk: None,
            branch: None,
            referrers: vec![],
            data: Data::zeros(),
        }
    }

    pub fn genesis() -> Self {
        Vertex::new()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vertex() {
        let vertex = Vertex::new();
    }
}