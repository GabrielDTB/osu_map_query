use super::_type::Type;
use super::hit_sample::HitSample;
use super::hit_sound::HitSound;
use super::traits::Position;
use super::traits::Time;

#[derive(Debug, Clone, PartialEq)]
pub struct Spinner {
    x: i64,
    y: i64,
    time: i64,
    flags: Type,
    hit_sound: HitSound,
    end_time: i64,
    hit_sample: HitSample,
}
impl std::str::FromStr for Spinner {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        let hit_sample = HitSample {
            ..Default::default()
        };
        let x = line
            .next()
            .expect("in x assignment in Spinner parsing")
            .parse::<i64>()
            .expect("in i64 parsing in x assignment in Spinner parsing");
        let y = line
            .next()
            .expect("in y assignment in Spinner parsing")
            .parse::<i64>()
            .expect("in i64 parsing in y assignment in Spinner parsing");
        let time = line
            .next()
            .expect("in time assignment in Spinner parsing")
            .parse::<i64>()
            .expect("in i64 parsing in time assignment in Spinner parsing");
        let flags = line
            .next()
            .expect("in flags assignment in Spinner parsing")
            .parse::<Type>()
            .expect("in Type parsing in flags assignment in Spinner parsing");
        let hit_sound = line
            .next()
            .expect("in hit_sound assignment in Spinner parsing")
            .parse::<HitSound>()
            .expect("in HitSound parsing in hit_sound assignment in Spinner parsing");
        let end_time = line
            .next()
            .expect("in end_time assignment in Spinner parsing")
            .parse::<i64>()
            .expect("in i64 parsing in end_time assignment in Spinner parsing");
        Ok(Spinner {
            x,
            y,
            time,
            flags,
            hit_sound,
            end_time,
            hit_sample,
        })
    }
}
impl Position for Spinner {
    fn position(&self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }
}
impl Time for Spinner {
    fn time(&self) -> i64 {
        self.time
    }
}
