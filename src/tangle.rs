use crate::vertex::Vertex;

pub struct Tangle {
    genesis: Vertex,
    size: usize,
}

impl Tangle {
    pub fn new() -> Self {
        Self {
            genesis: Vertex::genesis(),
            size: 1,
        }
    }

    pub fn append(&self, v: Vertex) {

    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tangle() {
        let tangle = Tangle::new();

        assert_eq!(1, tangle.size());
    }
}