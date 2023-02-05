#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    Osu,
    Taiko,
    Catch,
    Mania,
}
impl std::str::FromStr for Mode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Osu),
            "1" => Ok(Self::Taiko),
            "2" => Ok(Self::Catch),
            "3" => Ok(Self::Mania),
            _ => Err(format!("invalid mode {s}")),
        }
    }
}
