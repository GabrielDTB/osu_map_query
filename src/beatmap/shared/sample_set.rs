#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SampleSet {
    Default,
    Normal,
    Soft,
    Drum,
}
impl std::str::FromStr for SampleSet {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | "Default" => Ok(SampleSet::Default),
            "1" | "Normal" => Ok(SampleSet::Normal),
            "2" | "Soft" => Ok(SampleSet::Soft),
            "3" | "Drum" => Ok(SampleSet::Drum),
            _ => return Result::Err(format!("Invalid str during SampleSet parsing: {}", s)),
        }
    }
}
