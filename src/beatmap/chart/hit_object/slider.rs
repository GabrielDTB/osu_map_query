pub mod curve;

use super::_type::Type;
use super::hit_sample::{HalfHitSample, HitSample};
use super::hit_sound::HitSound;
use super::traits::Position;
use super::traits::Time;
use curve::Curve;

#[derive(Debug, Clone, PartialEq)]
pub struct Slider {
    x: i64,
    y: i64,
    time: i64,
    flags: Type,
    hit_sound: HitSound,
    curve: Curve,
    slides: i64,
    length: f64,
    edge_sounds: Vec<HitSound>,
    edge_sets: Vec<HalfHitSample>,
    hit_sample: HitSample,
}
impl std::str::FromStr for Slider {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        let x = line
            .next()
            .expect("in x assignment in Slider parsing")
            .parse::<i64>()
            .expect("in i64 parsing in x assignment in Slider parsing");
        let y = line
            .next()
            .expect("in y assignment in Slider parsing")
            .parse::<i64>()
            .expect("in i64 parsing in y assignment in Slider parsing");
        let time = line
            .next()
            .expect("in time assignment in Slider parsing")
            .parse::<i64>()
            .expect("in i64 parsing in time assignment in Slider parsing");
        let flags = line
            .next()
            .expect("in flags assignment in Slider parsing")
            .parse::<Type>()
            .expect("in Type parsing in flags assignment in Slider parsing");
        let hit_sound = line
            .next()
            .expect("in hit_sound assignment in Slider parsing")
            .parse::<HitSound>()
            .expect("in HitSound parsing in hit_sound assignment in Slider parsing");
        let collected = line.collect::<Vec<&str>>();
        let commas = collected.len();
        let mut line = collected.into_iter();
        let curve = match commas {
            3 | 4 | 6 => line
                .next()
                .expect("in curve assignment in Slider parsing")
                .parse::<Curve>()
                .expect("in Curve parsing in curve assignment in Slider parsing"),
            _ => {
                return Result::Err(format!(
                    "Invalid slider: wrong remaining line size: {} in line: {} at curve assignment",
                    commas, s
                ))
            }
        };
        let slides = match commas {
            3 | 4 | 6 => line
                .next()
                .expect("in slides assignment in Slider parsing")
                .parse::<i64>()
                .expect("in i64 parsing in slides assignment in Slider parsing"),
            _ => {
                return Result::Err(format!(
                "Invalid slider: wrong remaining line size: {} in line: {} at slides assignment",
                commas, s
            ))
            }
        };
        let length = match commas {
            3 | 4 | 6 => line
                .next()
                .expect("in length assignment in Slider parsing")
                .parse::<f64>()
                .expect("in f64 parsing in length assignment in Slider parsing"),
            _ => {
                return Result::Err(format!(
                "Invalid slider: wrong remaining line size: {} in line: {} at length assignment",
                commas, s
            ))
            }
        };
        let edge_sounds = match commas {
            4 | 6 => {
                let mut sounds = Vec::new();
                for sound in line
                    .next()
                    .expect("in sound assignment in edge_sounds assignment in Slider parsing")
                    .split('|')
                {
                    sounds.push(
                        sound
                            .parse::<HitSound>()
                            .expect("in sounds pushing with HitSound parsing in edge_sounds assignment in Slider parsing"),
                    );
                }
                sounds
            }
            _ => vec![
                "0".parse::<HitSound>().expect(
                    "at edge_sounds assignment with HitSound parsing of \"0\" in Slider parsing",
                ),
                "2".parse::<HitSound>().expect(
                    "at edge_sounds assignment with HitSound parsing of \"2\" in Slider parsing",
                ),
            ],
        };
        let edge_sets = match commas {
            6 => {
                let mut sounds = Vec::new();
                for sound in line
                    .next()
                    .expect("in sound assignment in edge_sets assignment in Slider parsing")
                    .split('|')
                {
                    sounds.push(
                        sound
                            .parse::<HalfHitSample>()
                            .expect("in sounds pushing with HalfHitSample parsing in edge_sets assignment in Slider parsing"),
                    );
                }
                sounds
            }
            _ => vec![
                "0:0".parse::<HalfHitSample>().expect(
                    "at edge_sets assignment with HalfHitSample parsing of \"0:0\" in Slider parsing",
                ),
                "0:0".parse::<HalfHitSample>().expect(
                    "at edge_sets assignment with HalfHitSample parsing of \"0:0\" in Slider parsing",
                ),
            ],
        };
        let hit_sample = match commas {
            6 => line
                .next()
                .expect("in hit_sample assignment in Slider parsing")
                .parse::<HitSample>()
                .expect("in HitSample parsing in hit_sample assignment in Slider parsing"),
            _ => HitSample {
                ..Default::default()
            },
        };
        Ok(Slider {
            x,
            y,
            time,
            flags,
            hit_sound,
            curve,
            slides,
            length,
            edge_sounds,
            edge_sets,
            hit_sample,
        })
    }
}
impl Position for Slider {
    fn position(&self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }
}
impl Time for Slider {
    fn time(&self) -> i64 {
        self.time
    }
}
