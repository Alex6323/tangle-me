use std::hash::Hasher;

pub struct NoHasher {}

impl Hasher for NoHasher {
    //
    //fn write_u64(&mut self, i: u64) { ... }
}
