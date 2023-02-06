#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HalfHitSample {
    normal_set: SampleSet,
    addition_set: SampleSet,
}
impl Default for HalfHitSample {
    fn default() -> Self {
        Self {
            normal_set: SampleSet::Default,
            addition_set: SampleSet::Default,
        }
    }
}
impl std::str::FromStr for HalfHitSample {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !(s.matches(':').count() == 1) {
            return Result::Err(format!("Invalid HalfHitSample: {}", s));
        }
        let mut values = s.split(':');
        let normal_set = values
            .next()
            .expect("at normal_set assignment in HalfHitSample parsing")
            .parse::<SampleSet>()
            .expect("at SampleSet parsing in normal_set assignment in HalfHitSample parsing");
        let addition_set = values
            .next()
            .expect("at addition_set assignment in HalfHitSample parsing")
            .parse::<SampleSet>()
            .expect("at SampleSet parsing in addition_set assignment in HalfHitSample parsing");
        Ok(Self {
            normal_set,
            addition_set,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct HitSample {
    normal_set: SampleSet,
    addition_set: SampleSet,
    index: i64,
    volume: i64, // From 0 to 100.
    filename: Option<String>,
}
impl Default for HitSample {
    fn default() -> Self {
        Self {
            normal_set: SampleSet::Default,
            addition_set: SampleSet::Default,
            index: 0,
            volume: 0,
            filename: None,
        }
    }
}
impl std::str::FromStr for HitSample {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut volume = 0;
        let mut filename = None;

        let mut values = match s.split_once(":") {
            Some(value) => value,
            None => {
                return Result::Err(format!(
                    "at values assignment before normal_set in HitSample parsing: {}",
                    s
                ))
            }
        };
        let normal_set = values
            .0
            .parse::<SampleSet>()
            .expect("at normal_set assignment with SampleSet parsing in HitSample parsing");
        values = match values.1.split_once(":") {
            Some(value) => value,
            None => {
                return Result::Err(format!(
                    "at values assignment before addition_set in HitSample parsing: {}",
                    s
                ))
            }
        };
        let addition_set = values
            .0
            .parse::<SampleSet>()
            .expect("at addition_set assignment with SampleSet parsing in HitSample parsing");
        values = match values.1.split_once(":") {
            Some(value) => value,
            None => {
                return Result::Err(format!(
                    "at values assignment before index in HitSample parsing: {}",
                    s
                ))
            }
        };
        let index = values
            .0
            .parse::<i64>()
            .expect("at index assignment with i64 parsing in HitSample parsing");
        if s.matches(':').count() > 3 {
            values = match values.1.split_once(":") {
                Some(value) => value,
                None => {
                    return Result::Err(format!(
                        "at values assignment before volume in HitSample parsing: {}",
                        s
                    ))
                }
            };
            volume = values
                .0
                .parse::<i64>()
                .expect("at volume assignment with i64 parsing in HitSample parsing");
        }
        if (s.matches(':').count() > 3) & !values.1.trim().is_empty() {
            filename = Some(values.1.to_string());
        }
        Ok(Self {
            normal_set,
            addition_set,
            index,
            volume,
            filename,
        })
    }
}
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
