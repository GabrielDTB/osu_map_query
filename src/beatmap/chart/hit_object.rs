pub mod _type;
pub mod hit_sample;
pub mod hit_sound;

use _type::Type;
use hit_sample::HitSample;
use hit_sound::HitSound;

#[derive(Debug, Clone, PartialEq)]
pub enum HitObject {
    Circle(Circle),
    // Slider(Slider),
    // Spinner(Spinner),
}
pub trait Position {
    fn position(&self) -> (f64, f64);
    fn distance<T: Position>(&self, other: &T) -> f64 {
        let first = self.position();
        let second = other.position();
        let delta_x = first.0 - second.0;
        let delta_y = first.1 - second.1;
        (delta_x.pow(2) + delta_y.pow(2)).sqrt()
    }
}
pub trait Time {
    fn time(&self) -> i64;
}
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
// #[derive(Debug, Clone, PartialEq)]
// pub struct Slider {
//     x: i64,
//     y: i64,
//     time: i64,
//     flags: Type,
//     hit_sound: HitSound,
//     curve: Curve,
//     slides: i64,
//     length: f64,
//     edge_sounds: Vec<HitSound>,
//     edge_sets: Vec<HalfHitSample>,
//     hit_sample: HitSample,
// }
// impl std::str::FromStr for Slider {
//     type Err = String;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut line = s.split(',');
//         let x = line
//             .next()
//             .expect("in x assignment in Slider parsing")
//             .parse::<i64>()
//             .expect("in i64 parsing in x assignment in Slider parsing");
//         let y = line
//             .next()
//             .expect("in y assignment in Slider parsing")
//             .parse::<i64>()
//             .expect("in i64 parsing in y assignment in Slider parsing");
//         let time = line
//             .next()
//             .expect("in time assignment in Slider parsing")
//             .parse::<i64>()
//             .expect("in i64 parsing in time assignment in Slider parsing");
//         let flags = line
//             .next()
//             .expect("in flags assignment in Slider parsing")
//             .parse::<Type>()
//             .expect("in Type parsing in flags assignment in Slider parsing");
//         let hit_sound = line
//             .next()
//             .expect("in hit_sound assignment in Slider parsing")
//             .parse::<HitSound>()
//             .expect("in HitSound parsing in hit_sound assignment in Slider parsing");
//         let collected = line.collect::<Vec<&str>>();
//         let commas = collected.len();
//         let mut line = collected.into_iter();
//         let curve = match commas {
//             3 | 4 | 6 => line
//                 .next()
//                 .expect("in curve assignment in Slider parsing")
//                 .parse::<Curve>()
//                 .expect("in Curve parsing in curve assignment in Slider parsing"),
//             _ => {
//                 return Result::Err(format!(
//                     "Invalid slider: wrong remaining line size: {} in line: {} at curve assignment",
//                     commas, s
//                 ))
//             }
//         };
//         let slides = match commas {
//             3 | 4 | 6 => line
//                 .next()
//                 .expect("in slides assignment in Slider parsing")
//                 .parse::<i64>()
//                 .expect("in i64 parsing in slides assignment in Slider parsing"),
//             _ => {
//                 return Result::Err(format!(
//                 "Invalid slider: wrong remaining line size: {} in line: {} at slides assignment",
//                 commas, s
//             ))
//             }
//         };
//         let length = match commas {
//             3 | 4 | 6 => line
//                 .next()
//                 .expect("in length assignment in Slider parsing")
//                 .parse::<f64>()
//                 .expect("in f64 parsing in length assignment in Slider parsing"),
//             _ => {
//                 return Result::Err(format!(
//                 "Invalid slider: wrong remaining line size: {} in line: {} at length assignment",
//                 commas, s
//             ))
//             }
//         };
//         let edge_sounds = match commas {
//             4 | 6 => {
//                 let mut sounds = Vec::new();
//                 for sound in line
//                     .next()
//                     .expect("in sound assignment in edge_sounds assignment in Slider parsing")
//                     .split('|')
//                 {
//                     sounds.push(
//                         sound
//                             .parse::<HitSound>()
//                             .expect("in sounds pushing with HitSound parsing in edge_sounds assignment in Slider parsing"),
//                     );
//                 }
//                 sounds
//             }
//             _ => vec![
//                 "0".parse::<HitSound>().expect(
//                     "at edge_sounds assignment with HitSound parsing of \"0\" in Slider parsing",
//                 ),
//                 "2".parse::<HitSound>().expect(
//                     "at edge_sounds assignment with HitSound parsing of \"2\" in Slider parsing",
//                 ),
//             ],
//         };
//         let edge_sets = match commas {
//             6 => {
//                 let mut sounds = Vec::new();
//                 for sound in line
//                     .next()
//                     .expect("in sound assignment in edge_sets assignment in Slider parsing")
//                     .split('|')
//                 {
//                     sounds.push(
//                         sound
//                             .parse::<HalfHitSample>()
//                             .expect("in sounds pushing with HalfHitSample parsing in edge_sets assignment in Slider parsing"),
//                     );
//                 }
//                 sounds
//             }
//             _ => vec![
//                 "0:0".parse::<HalfHitSample>().expect(
//                     "at edge_sets assignment with HalfHitSample parsing of \"0:0\" in Slider parsing",
//                 ),
//                 "0:0".parse::<HalfHitSample>().expect(
//                     "at edge_sets assignment with HalfHitSample parsing of \"0:0\" in Slider parsing",
//                 ),
//             ],
//         };
//         let hit_sample = match commas {
//             6 => line
//                 .next()
//                 .expect("in hit_sample assignment in Slider parsing")
//                 .parse::<HitSample>()
//                 .expect("in HitSample parsing in hit_sample assignment in Slider parsing"),
//             _ => HitSample {
//                 ..Default::default()
//             },
//         };
//         Ok(Slider {
//             x,
//             y,
//             time,
//             flags,
//             hit_sound,
//             curve,
//             slides,
//             length,
//             edge_sounds,
//             edge_sets,
//             hit_sample,
//         })
//     }
// }
// impl Position for Slider {
//     fn position(&self) -> Point {
//         Point {
//             x: self.x,
//             y: self.y,
//         }
//     }
// }
// impl Time for Slider {
//     fn time(&self) -> i64 {
//         self.time
//     }
// }
// #[derive(Debug, Clone, PartialEq)]
// pub struct Spinner {
//     x: i64,
//     y: i64,
//     time: i64,
//     flags: Type,
//     hit_sound: HitSound,
//     end_time: i64,
//     hit_sample: HitSample,
// }
// impl FromStr for Spinner {
//     type Err = String;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut line = s.split(',');
//         let hit_sample = HitSample {
//             ..Default::default()
//         };
//         let x = line
//             .next()
//             .expect("in x assignment in Spinner parsing")
//             .parse::<i64>()
//             .expect("in i64 parsing in x assignment in Spinner parsing");
//         let y = line
//             .next()
//             .expect("in y assignment in Spinner parsing")
//             .parse::<i64>()
//             .expect("in i64 parsing in y assignment in Spinner parsing");
//         let time = line
//             .next()
//             .expect("in time assignment in Spinner parsing")
//             .parse::<i64>()
//             .expect("in i64 parsing in time assignment in Spinner parsing");
//         let flags = line
//             .next()
//             .expect("in flags assignment in Spinner parsing")
//             .parse::<Type>()
//             .expect("in Type parsing in flags assignment in Spinner parsing");
//         let hit_sound = line
//             .next()
//             .expect("in hit_sound assignment in Spinner parsing")
//             .parse::<HitSound>()
//             .expect("in HitSound parsing in hit_sound assignment in Spinner parsing");
//         let end_time = line
//             .next()
//             .expect("in end_time assignment in Spinner parsing")
//             .parse::<i64>()
//             .expect("in i64 parsing in end_time assignment in Spinner parsing");
//         Ok(Spinner {
//             x,
//             y,
//             time,
//             flags,
//             hit_sound,
//             end_time,
//             hit_sample,
//         })
//     }
// }
// impl Position for Spinner {
//     fn position(&self) -> Point {
//         Point {
//             x: self.x,
//             y: self.y,
//         }
//     }
// }
// impl Time for Spinner {
//     fn time(&self) -> i64 {
//         self.time
//     }
// }
