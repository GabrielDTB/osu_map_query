#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
impl std::str::FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s
            .split(" : ")
            .skip(1)
            .next()
            .expect("at line assignment in Color parsing")
            .split(',');
        let red = line
            .next()
            .expect("in red assignment in Color parsing")
            .parse::<u8>()
            .expect("at u8 parsing of red in Color parsing");
        let green = line
            .next()
            .expect("in green assignment in Color parsing")
            .parse::<u8>()
            .expect("at u8 parsing of green in Color parsing");
        let blue = line
            .next()
            .expect("in blue assignment in Color parsing")
            .parse::<u8>()
            .expect("at u8 parsing of blue in Color parsing");
        Ok(Self { red, green, blue })
    }
}
impl Color {
    pub fn tuple(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}
