#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Difficulty {
    circle_size: u8,
    hpdrain_rate: u8,
    overall_difficulty: u8,
    approach_rate: u8,
}
impl Difficulty {
    pub fn new(
        circle_size: u8,
        hpdrain_rate: u8,
        overall_difficulty: u8,
        approach_rate: u8,
    ) -> Self {
        Self {
            circle_size,
            hpdrain_rate,
            overall_difficulty,
            approach_rate,
        }
    }
}
