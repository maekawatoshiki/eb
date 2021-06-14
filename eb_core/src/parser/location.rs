#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Location(pub u32);

impl Location {
    pub fn loc(self) -> u32 {
        self.0
    }
}
