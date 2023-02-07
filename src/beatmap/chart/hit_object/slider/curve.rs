#[derive(Debug, Clone, PartialEq)]
pub struct Curve {
    _type: CurveType,
    points: Vec<(i64, i64)>,
}
impl std::str::FromStr for Curve {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split('|');
        let _type = line
            .next()
            .expect("at _type assignment in Curve parsing")
            .parse::<CurveType>()
            .expect("at CurveType parsing in _type assignment in Curve parsing");
        let mut points = Vec::new();
        let mut count = 0;
        for pair in line {
            match (_type, count) {
                (CurveType::Perfect, 2) => {
                    return Result::Err(format!(
                        "Invalid Curve: Perfect curve {} has more than 2 points",
                        s
                    ))
                }
                _ => count += 1,
            }
            let mut pair = pair.split(':');
            points.push((
                match pair
                    .next()
                    .expect("at x assignment in points pushing in Curve parsing")
                    .parse::<i64>() {
                        Ok(x) => x,
                        Err(error) => return Result::Err(format!(
                                "at i64 parsing in x assignment in points pushing in Curve parsing: error: {} with input: {}",
                                error, s
                        )),
                },
                match pair
                    .next()
                    .expect("at y assignment in points pushing in Curve parsing")
                    .parse::<i64>() {
                        Ok(y) => y,
                        Err(error) => return Result::Err(format!(
                            "at i64 parsing in y assignment in points pushing in Curve parsing: error: {} with input: {}",
                            error, s
                        )),
                },
            ));
        }
        Ok(Self { _type, points })
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CurveType {
    Bezier,
    Centripetal,
    Linear,
    Perfect,
}
impl std::str::FromStr for CurveType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" => Ok(Self::Bezier),
            "C" => Ok(Self::Centripetal),
            "L" => Ok(Self::Linear),
            "P" => Ok(Self::Perfect),
            _ => return Result::Err(format!("Invalid CurveType: {}", s)),
        }
    }
}
