#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Break {
    start_time: i64,
    end_time: i64,
}
impl std::str::FromStr for Break {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',').skip(1);
        let start_time = line
            .next()
            .expect("at start_time assignment in Break parsing")
            .parse::<i64>()
            .expect("at i64 parsing of start_time in Break parsing");
        let end_time = line
            .next()
            .expect("at end_time assignment in Break parsing")
            .parse::<i64>()
            .expect("at i64 parsing of end_time in Break parsing");
        Ok(Self {
            start_time,
            end_time,
        })
    }
}
