#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OverlayPosition {
    NoChange,
    Below,
    Above,
}
impl std::str::FromStr for OverlayPosition {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NoChange" => Ok(Self::NoChange),
            "Below" => Ok(Self::Below),
            "Above" => Ok(Self::Above),
            _ => Err("Invalid OverlayPosition".into()),
        }
    }
}
