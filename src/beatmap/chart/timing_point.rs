pub mod effects;

use super::super::shared::sample_set::SampleSet;
use effects::Effects;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TimingPoint {
    time: i64,
    beat_length: f64,
    meter: i64,
    sample_set: SampleSet,
    sample_index: i64,
    volume: i64,
    uninherited: bool,
    effects: Effects,
}
impl Default for TimingPoint {
    fn default() -> Self {
        Self {
            time: 0,
            beat_length: 0.0,
            meter: 4,
            sample_set: SampleSet::Default,
            sample_index: 0,
            volume: 100,
            uninherited: true,
            effects: Effects {
                kiai: false,
                ommit_barline: false,
            },
        }
    }
}
impl std::str::FromStr for TimingPoint {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.matches(',').count() {
            1 => {
                let mut line = s.split(',');
                let time = line
                    .next()
                    .expect("at time assignment in TimingPoint parsing, 1 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of time in TimingPoint parsing, 1 branch");
                let beat_length = line
                    .next()
                    .expect("at beat_length assignment in TimingPoint parsing, 1 branch")
                    .parse::<f64>()
                    .expect("at f64 parsing of beat_length in TimingPoint parsing, 1 branch");
                Ok(Self {
                    time,
                    beat_length,
                    ..Default::default()
                })
            }
            5 => {
                let mut line = s.split(',');
                let time = line
                    .next()
                    .expect("at time assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of time in TimingPoint parsing, 7 branch");
                let beat_length = line
                    .next()
                    .expect("at beat_length assignment in TimingPoint parsing, 7 branch")
                    .parse::<f64>()
                    .expect("at f64 parsing of beat_length in TimingPoint parsing, 7 branch");
                let meter = line
                    .next()
                    .expect("at meter assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of meter in TimingPoint parsing, 7 branch");
                let sample_set = line
                    .next()
                    .expect("at sample_set assignment in TimingPoint parsing, 7 branch")
                    .parse::<SampleSet>()
                    .expect("at SampleSet parsing of sample_set in TimingPoint parsing, 7 branch");
                let sample_index = line
                    .next()
                    .expect("at sample_index assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of sample_index in TimingPoint parsing, 7 branch");
                //sample index
                let volume = line
                    .next()
                    .expect("at volume assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of volume in TimingPoint parsing, 7 branch");
                Ok(Self {
                    time,
                    beat_length,
                    meter,
                    sample_set,
                    sample_index,
                    volume,
                    ..Default::default()
                })
            }
            6 => {
                let mut line = s.split(',');
                let time = line
                    .next()
                    .expect("at time assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of time in TimingPoint parsing, 7 branch");
                let beat_length = line
                    .next()
                    .expect("at beat_length assignment in TimingPoint parsing, 7 branch")
                    .parse::<f64>()
                    .expect("at f64 parsing of beat_length in TimingPoint parsing, 7 branch");
                let meter = line
                    .next()
                    .expect("at meter assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of meter in TimingPoint parsing, 7 branch");
                let sample_set = line
                    .next()
                    .expect("at sample_set assignment in TimingPoint parsing, 7 branch")
                    .parse::<SampleSet>()
                    .expect("at SampleSet parsing of sample_set in TimingPoint parsing, 7 branch");
                let sample_index = line
                    .next()
                    .expect("at sample_index assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of sample_index in TimingPoint parsing, 7 branch");
                //sample index
                let volume = line
                    .next()
                    .expect("at volume assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of volume in TimingPoint parsing, 7 branch");
                let uninherited = line
                    .next()
                    .expect("at uninherited assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of uninherited in TimingPoint parsing, 7 branch")
                    == 1;
                Ok(Self {
                    time,
                    beat_length,
                    meter,
                    sample_set,
                    sample_index,
                    volume,
                    uninherited,
                    ..Default::default()
                })
            }
            7 => {
                let mut line = s.split(',');
                let time = line
                    .next()
                    .expect("at time assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of time in TimingPoint parsing, 7 branch");
                let beat_length = line
                    .next()
                    .expect("at beat_length assignment in TimingPoint parsing, 7 branch")
                    .parse::<f64>()
                    .expect("at f64 parsing of beat_length in TimingPoint parsing, 7 branch");
                let meter = line
                    .next()
                    .expect("at meter assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of meter in TimingPoint parsing, 7 branch");
                let sample_set = line
                    .next()
                    .expect("at sample_set assignment in TimingPoint parsing, 7 branch")
                    .parse::<SampleSet>()
                    .expect("at SampleSet parsing of sample_set in TimingPoint parsing, 7 branch");
                let sample_index = line
                    .next()
                    .expect("at sample_index assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of sample_index in TimingPoint parsing, 7 branch");
                //sample index
                let volume = line
                    .next()
                    .expect("at volume assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of volume in TimingPoint parsing, 7 branch");
                let uninherited = line
                    .next()
                    .expect("at uninherited assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of uninherited in TimingPoint parsing, 7 branch")
                    == 1;
                let effects = line
                    .next()
                    .expect("at effects assignment in TimingPoint parsing, 7 branch")
                    .parse::<Effects>()
                    .expect("at Effects parsing of effects in TimingPoint parsing, 7 branch");
                Ok(Self {
                    time,
                    beat_length,
                    meter,
                    sample_set,
                    sample_index,
                    volume,
                    uninherited,
                    effects,
                })
            }
            _ => return Result::Err(format!("Invalid timing point: {}", s)),
        }
    }
}
