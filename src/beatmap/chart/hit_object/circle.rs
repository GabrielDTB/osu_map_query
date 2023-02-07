use super::_type::Type;
use super::hit_sample::HitSample;
use super::hit_sound::HitSound;
use super::traits::Position;
use super::traits::Time;

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    x: i64,
    y: i64,
    time: i64,
    flags: Type,
    hit_sound: HitSound,
    hit_sample: HitSample,
}
impl std::str::FromStr for Circle {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        let hit_sample = HitSample {
            ..Default::default()
        };
        // TODO: Parse HitSamples
        let x = line
            .next()
            .expect("at x assignment in Circle parsing")
            .parse::<i64>()
            .expect("at i64 parsing in x assignment in Circle parsing");
        let y = line
            .next()
            .expect("at y assignment in Circle parsing")
            .parse::<i64>()
            .expect("at i64 parsing in y assignment in Circle parsing");
        let time = line
            .next()
            .expect("at time assignment in Circle parsing")
            .parse::<i64>()
            .expect("at i64 parsing in time assignment in Circle parsing");
        let flags = line
            .next()
            .expect("at flags assignment in Circle parsing")
            .parse::<Type>()
            .expect("at Type parsing in flags assignment in Circle parsing");
        let hit_sound = line
            .next()
            .expect("at hit_sound assignment in Circle parsing")
            .parse::<HitSound>()
            .expect("at HitSound parsing in hit_sound assignment in Circle parsing");
        Ok(Circle {
            x,
            y,
            time,
            flags,
            hit_sound,
            hit_sample,
        })
    }
}
impl Position for Circle {
    fn position(&self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }
}
impl Time for Circle {
    fn time(&self) -> i64 {
        self.time
    }
}
