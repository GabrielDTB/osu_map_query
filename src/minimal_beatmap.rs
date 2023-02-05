// Screen size: 640x480 osu px
// Play area: 510x385 osu px
// Center of playfield: 256x192 osu px

extern crate num;

use num::rational::Ratio;
use std::str::FromStr;

macro_rules! ratio {
    ($numer:expr) => {
        num::rational::Ratio::from_integer($numer)
    };
    ($numer:expr, $denom:expr) => {
        num::rational::Ratio::new($numer, $denom)
    };
}

/// Complete map data for a .osu file.
/// Arranged like map ver 14.
#[derive(Debug, Clone, PartialEq)]
pub struct Beatmap {
    file_format: i64,

    //[General]
    stack_leniency: Ratio<i64>,

    //[Difficulty]
    circle_size: Ratio<i64>,
    overall_difficulty: Ratio<i64>,
    approach_rate: Ratio<i64>,
    slider_multiplier: Ratio<i64>,
    slider_tick_rate: Ratio<i64>,

    //[TimingPoints]
    timing_points: Vec<TimingPoint>,

    //[HitObjects]
    hit_objects: Vec<HitObject>,
}
impl Beatmap {
    pub fn details(self) -> Result<String, String> {
        Ok(format!(
            "CS: {}\nOD: {}\nAR: {}\nSlider Multiplier: {}\nSlider Tick Rate: {}",
            self.circle_size,
            self.overall_difficulty,
            self.approach_rate,
            self.slider_multiplier,
            self.slider_tick_rate,
        ))
    }
}
pub trait Parse: Sized {
    fn parse(path: &std::path::Path) -> Result<Self, String>;
}
impl Parse for Beatmap {
    fn parse(path: &std::path::Path) -> Result<Self, String> {
        // Setup
        let mut file = match std::fs::File::open(path) {
            Err(error) => return Result::Err(format!("couldn't open: '{}'", error)),
            Ok(file) => file,
        };
        let mut content = String::new();
        match std::io::Read::read_to_string(&mut file, &mut content) {
            Ok(_) => {}
            Err(error) => panic!(
                "Couldn't open file: '{}' with error: '{}'",
                path.to_string_lossy(),
                error
            ),
        };

        let mut header = "";
        for line in content.lines() {
            let line = line.trim();
            if line.contains("file format v") {
                header = line;
                break;
            }
        }
        assert!(
            !header.is_empty(),
            "No valid header in map: \"{}\"",
            path.to_string_lossy(),
        );
        let file_format = header.split_once("v").unwrap().1.parse::<i64>().unwrap();

        let mut stack_leniency = ratio!(7, 10);
        let mut circle_size = None;
        let mut overall_difficulty = None;
        let mut approach_rate = None;
        let mut slider_multiplier = None;
        let mut slider_tick_rate = None;
        let (tabular, content) = content
            .split_once("[General]")
            .unwrap()
            .1 // Discard everything before [General].
            .split_once("[Events]")
            .unwrap();
        for line in tabular.lines().map(|l| l.trim()) {
            if line.starts_with("[") || line.is_empty() {
                continue;
            }
            let (key, value) = match line.split_once(":") {
                Some(pair) => (pair.0.trim(), pair.1.trim()),
                None => return Err(format!("Can't split line '{}'", line)),
            };
            match key {
                "StackLeniency" => {
                    stack_leniency = decimal_to_ratio(value)?;
                }
                "CircleSize" => {
                    circle_size = Some(decimal_to_ratio(value)?);
                }
                "OverallDifficulty" => {
                    overall_difficulty = Some(decimal_to_ratio(value)?);
                }
                "ApproachRate" => {
                    approach_rate = Some(decimal_to_ratio(value)?);
                }
                "SliderMultiplier" => {
                    slider_multiplier = Some(decimal_to_ratio(value)?);
                }
                "SliderTickRate" => {
                    slider_tick_rate = Some(decimal_to_ratio(value)?);
                }
                _ => {} // Ignore everything else.
            }
        }
        // Old maps have their AR and OD tied together.
        if approach_rate.is_none() {
            approach_rate = overall_difficulty;
        }

        let (content, objects) = content
            .split_once("[TimingPoints]")
            .unwrap()
            .1
            .split_once("[HitObjects]")
            .unwrap();
        let mut hit_objects = Vec::new();
        for line in objects.lines().map(|l| l.trim()) {
            if line.is_empty() {
                continue;
            }
            let object_type = match line
                .split(',')
                .nth(3)
                .expect("at type indexing for hit object classification")
                .parse::<Type>()
            {
                Ok(value) => value.object_type,
                Err(error) => {
                    return Err(format!(
                        "couldn't determine object type: '{}' error: '{}'",
                        line, error
                    ))
                }
            };
            match object_type {
                ObjectType::Circle => {
                    hit_objects.push(HitObject::Circle(line.parse::<Circle>()?));
                }
                ObjectType::Slider => {
                    hit_objects.push(HitObject::Slider(line.parse::<Slider>()?));
                }
                ObjectType::Spinner => {
                    hit_objects.push(HitObject::Spinner(line.parse::<Spinner>()?));
                }
            }
        }

        let timing = if content.contains("[Colours]") {
            content.split_once("[Colours]").unwrap().0
        } else {
            content
        };
        let mut timing_points = Vec::new();
        for line in timing.lines().map(|l| l.trim()) {
            if line.is_empty() {
                continue;
            }
            timing_points.push(line.parse::<TimingPoint>()?);
        }
        Ok(Beatmap {
            file_format,
            stack_leniency,
            circle_size: circle_size.unwrap(),
            overall_difficulty: overall_difficulty.unwrap(),
            approach_rate: approach_rate.unwrap(),
            slider_multiplier: slider_multiplier.unwrap(),
            slider_tick_rate: slider_tick_rate.unwrap(),
            timing_points,
            hit_objects,
        })
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TimingPoint {
    time: Ratio<i64>,
    beat_length: f64,
    uninherited: bool,
}
impl Default for TimingPoint {
    fn default() -> Self {
        Self {
            time: ratio!(0),
            beat_length: 0.0,
            uninherited: true,
        }
    }
}
impl FromStr for TimingPoint {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.matches(',').count() {
            1 => {
                let mut line = s.split(',');
                let time = match decimal_to_ratio(line
                    .next()
                    .expect("at time assignment in TimingPoint parsing, 1 branch")
                 ) {
                        Ok(ok) => ok,
                        Err(error) => panic!("at i64 parsing of time in TimingPoint parsing, 1 branch. Input: \"{}\", Error: {}", s, error),
                };
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
            4 => {
                let mut line = s.split(',');
                let time = decimal_to_ratio(
                    line.next()
                        .expect("at time assignment in TimingPoint parsing, 7 branch"),
                )
                .expect("at i64 parsing of time in TimingPoint parsing, 7 branch");
                let beat_length = line
                    .next()
                    .expect("at beat_length assignment in TimingPoint parsing, 7 branch")
                    .parse::<f64>()
                    .expect("at f64 parsing of beat_length in TimingPoint parsing, 7 branch");
                Ok(Self {
                    time,
                    beat_length,
                    ..Default::default()
                })
            }
            5 => {
                let mut line = s.split(',');
                let time = decimal_to_ratio(
                    line.next()
                        .expect("at time assignment in TimingPoint parsing, 7 branch"),
                )
                .expect("at i64 parsing of time in TimingPoint parsing, 7 branch");
                let beat_length = line
                    .next()
                    .expect("at beat_length assignment in TimingPoint parsing, 7 branch")
                    .parse::<f64>()
                    .expect("at f64 parsing of beat_length in TimingPoint parsing, 7 branch");
                Ok(Self {
                    time,
                    beat_length,
                    ..Default::default()
                })
            }
            6 => {
                let mut line = s.split(',');
                let time = decimal_to_ratio(
                    line.next()
                        .expect("at time assignment in TimingPoint parsing, 7 branch"),
                )
                .expect("at i64 parsing of time in TimingPoint parsing, 7 branch");
                let beat_length = line
                    .next()
                    .expect("at beat_length assignment in TimingPoint parsing, 7 branch")
                    .parse::<f64>()
                    .expect("at f64 parsing of beat_length in TimingPoint parsing, 7 branch");
                line.next();
                line.next();
                line.next();
                line.next();
                let uninherited = line
                    .next()
                    .expect("at uninherited assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of uninherited in TimingPoint parsing, 7 branch")
                    == 1;
                Ok(Self {
                    time,
                    beat_length,
                    uninherited,
                    ..Default::default()
                })
            }
            7 => {
                let mut line = s.split(',');
                let time = decimal_to_ratio(
                    line.next()
                        .expect("at time assignment in TimingPoint parsing, 7 branch"),
                )
                .expect("at i64 parsing of time in TimingPoint parsing, 7 branch");
                let beat_length = line
                    .next()
                    .expect("at beat_length assignment in TimingPoint parsing, 7 branch")
                    .parse::<f64>()
                    .expect("at f64 parsing of beat_length in TimingPoint parsing, 7 branch");
                line.next();
                line.next();
                line.next();
                line.next();
                let uninherited = line
                    .next()
                    .expect("at uninherited assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of uninherited in TimingPoint parsing, 7 branch")
                    == 1;
                Ok(Self {
                    time,
                    beat_length,
                    uninherited,
                })
            }
            _ => return Result::Err(format!("Invalid timing point: {}", s)),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum HitObject {
    Circle(Circle),
    Slider(Slider),
    Spinner(Spinner),
}
pub trait HasPosition: Sized {
    fn position(&self) -> (f64, f64);
    fn distance(&self, other: &impl HasPosition) -> f64 {
        let (sx, sy) = self.position();
        let (ox, oy) = other.position();
        let delta_x = sx - ox;
        let delta_y = sy - oy;
        ((delta_x.powf(2.0) + delta_y.powf(2.0)) as f64).sqrt()
    }
    fn angle(&self, aa: &Self, ab: &Self) -> f64 {
        let sa = self.distance(aa);
        let sb = self.distance(ab);
        let sc = aa.distance(ab);
        let cos = (sa.powf(2.0) + sb.powf(2.0) - sc.powf(2.0)) / (2.0 * sa * sb);
        cos.acos()
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
}
impl FromStr for Circle {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
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
        Ok(Circle { x, y, time })
    }
}
impl HasPosition for Circle {
    fn position(&self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }
}
impl Time for Circle {
    fn time(&self) -> i64 {
        self.time
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Slider {
    x: Ratio<i64>,
    y: Ratio<i64>,
    time: i64,
    curve: Curve,
    slides: i64,
    length: f64,
}
impl FromStr for Slider {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        let x = match decimal_to_ratio(line.next().expect("in x assignment in Slider parsing")) {
            Ok(okay) => okay,
            Err(error) => {
                return Err(format!(
                    "in i64 parsing in x assignment in Slider parsing: {}, {}",
                    s, error
                ))
            }
        };
        let y = match decimal_to_ratio(line.next().expect("in y assignment in Slider parsing")) {
            Ok(okay) => okay,
            Err(error) => {
                return Err(format!(
                    "in i64 parsing in y assignment in Slider parsing: {}, {}",
                    s, error
                ))
            }
        };
        let time = line
            .next()
            .expect("in time assignment in Slider parsing")
            .parse::<i64>()
            .expect("in i64 parsing in time assignment in Slider parsing");
        line.next();
        line.next();
        let collected = line.collect::<Vec<&str>>();
        let commas = if collected.last().unwrap().is_empty() {
            collected.len() - 1
        } else {
            collected.len()
        };
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
        Ok(Slider {
            x,
            y,
            time,
            curve,
            slides,
            length,
        })
    }
}
impl HasPosition for Slider {
    fn position(&self) -> (f64, f64) {
        let x = (*self.x.numer() as f64) / (*self.x.denom() as f64);
        let y = (*self.y.numer() as f64) / (*self.y.denom() as f64);
        (x, y)
    }
}
impl Time for Slider {
    fn time(&self) -> i64 {
        self.time
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Spinner {
    x: i64,
    y: i64,
    time: i64,
    end_time: i64,
}
impl FromStr for Spinner {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
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
        line.next();
        line.next();
        let end_time = line
            .next()
            .expect("in end_time assignment in Spinner parsing")
            .parse::<i64>()
            .expect("in i64 parsing in end_time assignment in Spinner parsing");
        Ok(Spinner {
            x,
            y,
            time,
            end_time,
        })
    }
}
impl HasPosition for Spinner {
    fn position(&self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }
}
impl Time for Spinner {
    fn time(&self) -> i64 {
        self.time
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Type {
    pub object_type: ObjectType, // 0 circle, 1 slider, 3 spinner
}
impl FromStr for Type {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num: i64 = s
            .parse()
            .expect("at num assignment and i64 parsing in Type parsing");
        let mut bits = [false; 8];
        if num > 2_i64.pow(8) - 1 {
            return Result::Err(format!("Invalid Type"));
        }
        if num > 2_i64.pow(7) - 1 {
            bits[7] = true;
            num -= 2_i64.pow(7);
        }
        if num > 2_i64.pow(6) - 1 {
            bits[6] = true;
            num -= 2_i64.pow(6);
        }
        if num > 2_i64.pow(5) - 1 {
            bits[5] = true;
            num -= 2_i64.pow(5);
        }
        if num > 2_i64.pow(4) - 1 {
            bits[4] = true;
            num -= 2_i64.pow(4);
        }
        if num > 2_i64.pow(3) - 1 {
            bits[3] = true;
            num -= 2_i64.pow(3);
        }
        if num > 2_i64.pow(2) - 1 {
            bits[2] = true;
            num -= 2_i64.pow(2);
        }
        if num > 2_i64.pow(1) - 1 {
            bits[1] = true;
            num -= 2_i64.pow(1);
        }
        if num > 2_i64.pow(0) - 1 {
            bits[0] = true;
            num -= 2_i64.pow(0);
        }
        if num > 0 {
            return Result::Err(format!("Logic error in Type creation"));
        }
        if bits[7] {
            return Result::Err(format!("osu!mania hold object from '{}'", s));
        }
        let object_type = match (bits[0], bits[1], bits[3]) {
            (true, false, false) => ObjectType::Circle,
            (false, true, false) => ObjectType::Slider,
            (false, false, true) => ObjectType::Spinner,
            _ => {
                return Result::Err(format!(
                    "Invalid object type: {:?} in: '{}'",
                    (bits[0], bits[1], bits[3]),
                    s
                ))
            }
        };
        Ok(Self { object_type })
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ObjectType {
    Circle,
    Slider,
    Spinner,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Curve {
    _type: CurveType,
    points: Vec<Point>,
}
impl FromStr for Curve {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split('|');
        let _type = line
            .next()
            .expect("at _type assignment in Curve parsing")
            .parse::<CurveType>()
            .expect("at CurveType parsing in _type assignment in Curve parsing");
        let mut points = Vec::new();
        for pair in line {
            let mut pair = pair.split(':');
            points.push(Point {
                x: match pair
                    .next()
                    .expect("at x assignment in points pushing in Curve parsing")
                    .parse::<i64>() {
                        Ok(x) => x,
                        Err(error) => return Result::Err(format!(
                                "at i64 parsing in x assignment in points pushing in Curve parsing: error: {} with input: {}",
                                error, s
                        )),
                },
                y: match pair
                    .next()
                    .expect("at y assignment in points pushing in Curve parsing")
                    .parse::<i64>() {
                        Ok(y) => y,
                        Err(error) => return Result::Err(format!(
                            "at i64 parsing in y assignment in points pushing in Curve parsing: error: {} with input: {}",
                            error, s
                        )),
                },
            });
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
impl FromStr for CurveType {
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
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: i64,
    y: i64,
}
impl HasPosition for Point {
    fn position(&self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }
}
pub struct Angle {
    radians: f64,
}

pub fn decimal_to_ratio(decimal: &str) -> Result<Ratio<i64>, String> {
    // TODO: Switch from panics to errors
    if decimal.is_empty() {
        return Result::Err(format!("Empty str attempted to parse into ratio"));
    }
    match decimal.matches('-').count() {
        0 => {}
        1 => {
            if !decimal.starts_with('-') {
                return Result::Err(format!(
                    "Non-decimal str attempted to parse into ratio: Contains - sign in a location other than the start: {}", 
                    decimal
                ));
            }
        }
        _ => {
            return Result::Err(format!(
                "Non-decimal str attempted to parse into ratio: Contains more than one - sign: {}",
                decimal
            ))
        }
    }
    if !decimal
        .chars()
        .all(|c| c.is_ascii_digit() | (c == '.') | (c == '-'))
    {
        return Result::Err(format!(
            "Non-decimal str attempted to parse into ratio: Non-numeric characters present: {}",
            decimal
        ));
    }
    match decimal.matches('.').count() {
        0 => Ok(ratio!(decimal.parse::<i64>().unwrap())),
        1 => {
            let mut decimal = decimal.split('.');
            let top = decimal.next().unwrap();
            let bottom = decimal.next().unwrap();
            let denom = 10_i64.pow(bottom.len() as u32);
            let top = top.parse::<i64>().unwrap() * denom;
            let bottom = bottom.parse::<i64>().unwrap();
            let numer = top + bottom;
            Ok(ratio!(numer, denom))
        }
        _ => {
            return Result::Err(format!(
                "Non-decimal str attempted to parse into ratio: Too many periods: {}",
                decimal
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_all_maps() {
        let test_maps = std::fs::read_dir("/mnt/ramdisk").unwrap();
        for file in test_maps {
            match file {
                Ok(file) => {
                    let path = file.path();
                    if let Err(error) = Beatmap::parse(&path) {
                        panic!(
                            "Error parsing test map \"{}\" with error: {}",
                            path.to_str().unwrap(),
                            error
                        )
                    }
                }
                Err(error) => panic!("{}", error),
            }
        }
    }
}
