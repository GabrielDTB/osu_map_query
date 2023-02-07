#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Effects {
    kiai: bool, // 1 on
    // 2 is unused
    ommit_barline: bool, // 4 on
}
impl std::str::FromStr for Effects {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kiai, ommit_barline) = match s {
            "0" => (false, false),
            "1" => (true, false),
            "4" => (false, true),
            "5" => (true, true),
            _ => return Result::Err(format!("Invalid effect")),
        };
        Ok(Self {
            kiai,
            ommit_barline,
        })
    }
}
