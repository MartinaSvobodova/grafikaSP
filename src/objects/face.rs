#[derive(Copy, Clone, Debug)]
pub struct Face {
    pub indices: [u16; 3],
}

impl Face {
    pub fn new(a: u16, b: u16, c: u16) -> Self {
        Self { indices: [a, b, c] }
    }
}
