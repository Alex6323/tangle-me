pub struct Data(Vec<u8>);

impl Data {
    pub fn zeros() -> Self {
        Self(vec![0u8; 1604])
    }
}