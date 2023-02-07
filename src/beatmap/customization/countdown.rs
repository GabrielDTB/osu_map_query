#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Countdown {
    None,
    Normal,
    Half,
    Double,
}
impl std::str::FromStr for Countdown {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Countdown::None),
            "1" => Ok(Countdown::Normal),
            "2" => Ok(Countdown::Half),
            "3" => Ok(Countdown::Double),
            _ => Err("Invalid Countdown".into()),
        }
    }
}
