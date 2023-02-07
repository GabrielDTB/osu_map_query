#[derive(Debug, Clone, PartialEq)]
pub struct Background {
    filename: String,
    xoffset: i64,
    yoffset: i64,
}
impl std::str::FromStr for Background {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Background event
        let mut line = s.split(',').skip(2);
        let mut background = line
            .next()
            .expect("at background assignment in Background parsing")
            .chars();
        background.next();
        background.next_back();
        // Remove quotes
        let background = background.as_str().to_string();
        let x = line
            .next()
            .unwrap_or("0")
            .parse::<i64>()
            .expect("at x assignment in Background parsing");
        let y = line
            .next()
            .unwrap_or("0")
            .parse::<i64>()
            .expect("at y assignment in Background parsing");
        Ok(Self {
            filename: background,
            xoffset: x,
            yoffset: y,
        })
    }
}
