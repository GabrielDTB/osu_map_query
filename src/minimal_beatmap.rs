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

/// Parse a .osu file and return a MapData object
pub fn parse_map(path: &std::path::Path) -> Result<Beatmap, String> {
    // Try to open file
    let mut file = match std::fs::File::open(path) {
        Err(why) => return Result::Err(format!("couldn't open: {}", why)),
        Ok(file) => file,
    };
    let mut content = String::new();
    // Assign contents of file to string.
    std::io::Read::read_to_string(&mut file, &mut content).expect("Couldn't read file.");
    assert!(
        content.starts_with("osu file format v"),
        "Invalid .osu file header"
    );
    assert!(
        content.contains("[Events]"),
        ".osu file does not contain [Events]"
    );
    assert!(
        content.contains("[TimingPoints]"),
        ".osu file does not contain [TimingPoints]"
    );
    assert!(
        content.contains("[HitObjects]"),
        ".osu file does not contain [HitObjects]"
    );
    let mut map = Beatmap {
        ..Default::default()
    };
    // TODO: Fix potential parsing error from arbitrary strings
    // within the file containing the content split markers.
    let mut content = content.split("[Events]");
    let mut tabular = content.next().expect("at tabular assignment").lines();
    map.file_format = tabular
        .next()
        .expect("at map.file_format assignment (1 deep)")
        .split("v")
        .last()
        .expect("at map.file_format assignment (2 deep)")
        .trim()
        .parse::<i64>()
        .expect("at map.file_format assignment (3 deep)");
    for line in tabular {
        if line.starts_with("[") || line.trim().is_empty() {
            // Filter out junk lines.
            continue;
        }
        let line = line.split_once(":").expect("at line splitting in tabular");
        let key = line.0.trim();
        let value = line.1.trim();
        match key {
            "StackLeniency" => {
                map.stack_leniency =
                    decimal_to_ratio(value).expect("at map.stack_leniency assignment in tabular")
            }
            "CircleSize" => {
                map.circle_size =
                    Some(decimal_to_ratio(value).expect("at map.circle_size assignment in tabular"))
            }
            "OverallDifficulty" => {
                map.overall_difficulty = Some(
                    decimal_to_ratio(value)
                        .expect("at map.overall_difficulty assignment in tabular"),
                )
            }
            "ApproachRate" => {
                map.approach_rate = Some(
                    decimal_to_ratio(value).expect("at map.approach_rate assignment in tabular"),
                )
            }
            "SliderMultiplier" => {
                map.slider_multiplier = Some(
                    decimal_to_ratio(value)
                        .expect("at map.slider_multiplier assignment in tabular"),
                )
            }
            "SliderTickRate" => {
                map.slider_tick_rate = Some(
                    decimal_to_ratio(value).expect("at map.slider_tick_rate assignment in tabular"),
                )
            }
            _ => {}
        }
    }
    // Old maps have their AR and OD tied together.
    if map.approach_rate.is_none() {
        map.approach_rate = map.circle_size;
    }
    let mut mixed = content
        .next()
        .expect("at mixed assignment after tabular")
        .split("[TimingPoints]");
    mixed.next();
    let mixed = mixed.next().expect("at mixed assignment after events");
    let colours_exists = mixed.contains("[Colours]");
    let mut mixed = mixed.split(if colours_exists {
        "[Colours]"
    } else {
        "[HitObjects]"
    });
    let timing_points = mixed.next().expect("at timing_points assignment").lines();
    for line in timing_points {
        if line.trim().is_empty() {
            // Filter out junk lines.
            continue;
        }
        if map.timing_points.is_none() {
            map.timing_points = Some(Vec::new());
        }
        map.timing_points
            .as_mut()
            .expect("at map.timing_points grabbing")
            .push(
                line.parse::<TimingPoint>()
                    .expect("at map.timing_points pushing"),
            );
    }
    if colours_exists {
        mixed = mixed
            .next()
            .expect("at mixed assignment in colours")
            .split("[HitObjects]");
        mixed.next();
    }
    let hit_objects = mixed.next().expect("at hit_objects assignment").lines();
    for line in hit_objects {
        if line.trim().is_empty() {
            // Filter out junk lines.
            continue;
        }
        if map.hit_objects.is_none() {
            map.hit_objects = Some(Vec::new());
        }
        match line
            .split(',')
            .nth(3)
            .expect("at type indexing for hit object classification")
            .parse::<Type>()
            .expect("at type parsing for hit object classification")
            .object_type
        {
            ObjectType::Circle => {
                map.hit_objects
                    .as_mut()
                    .expect("at map.circles grabbing")
                    .push(HitObject::Circle(
                        line.parse::<Circle>().expect("at map.circles pushing"),
                    ));
            }
            ObjectType::Slider => {
                map.hit_objects
                    .as_mut()
                    .expect("at map.sliders grabbing")
                    .push(HitObject::Slider(
                        line.parse::<Slider>().expect("at map.sliders pushing"),
                    ));
            }
            ObjectType::Spinner => {
                map.hit_objects
                    .as_mut()
                    .expect("at map.spinners grabbing")
                    .push(HitObject::Spinner(
                        line.parse::<Spinner>().expect("at map.spinners pushing"),
                    ));
            }
        }
    }
    Ok(map)
}

/// Complete map data for a .osu file.
/// Arranged like map ver 14.
#[derive(Debug, Clone, PartialEq)]
pub struct Beatmap {
    file_format: i64,

    //[General]
    stack_leniency: Ratio<i64>,

    //[Difficulty]
    circle_size: Option<Ratio<i64>>,
    overall_difficulty: Option<Ratio<i64>>,
    approach_rate: Option<Ratio<i64>>,
    slider_multiplier: Option<Ratio<i64>>,
    slider_tick_rate: Option<Ratio<i64>>,

    //[TimingPoints]
    timing_points: Option<Vec<TimingPoint>>,

    //[HitObjects]
    hit_objects: Option<Vec<HitObject>>,
}
impl Beatmap {
    pub fn new(path: &std::path::Path) -> Self {
        parse_map(path).unwrap()
    }
    pub fn details(self) -> Result<String, String> {
        let circle_size = match self.circle_size {
            Some(value) => value,
            None => return Result::Err(format!("Map does not have a circle size")),
        };
        let overall_difficulty = match self.overall_difficulty {
            Some(value) => value,
            None => return Result::Err(format!("Map does not have an overall difficulty")),
        };
        let approach_rate = match self.approach_rate {
            Some(value) => value,
            None => return Result::Err(format!("Map does not have an approach rate")),
        };
        let slider_multiplier = match self.slider_multiplier {
            Some(value) => value,
            None => return Result::Err(format!("Map does not have a slider multiplier")),
        };
        let slider_tick_rate = match self.slider_tick_rate {
            Some(value) => value,
            None => return Result::Err(format!("Map does not have a slider tick rate")),
        };
        Ok(format!(
            "CS: {}\nOD: {}\nAR: {}\nSlider Multiplier: {}\nSlider Tick Rate: {}",
            circle_size, overall_difficulty, approach_rate, slider_multiplier, slider_tick_rate,
        ))
    }
    pub fn hit_objects
}
impl Default for Beatmap {
    fn default() -> Self {
        Self {
            file_format: 14,

            stack_leniency: ratio!(7, 10),

            //[Difficulty]
            circle_size: None,
            overall_difficulty: None,
            approach_rate: None,
            slider_multiplier: None,
            slider_tick_rate: None,

            //[TimingPoints]
            timing_points: None,

            //[HitObjects]
            hit_objects: None,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TimingPoint {
    time: i64,
    beat_length: f64,
    uninherited: bool,
}
impl Default for TimingPoint {
    fn default() -> Self {
        Self {
            time: 0,
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
                Ok(Self {
                    time,
                    beat_length,
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
pub trait Position {
    fn position(&self) -> Point;
}
pub trait Time {
    fn time(&self) -> i64;
}
pub trait Distance {
    fn distance(&self, other: &Self) -> f64;
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
impl Position for Circle {
    fn position(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}
impl Time for Circle {
    fn time(&self) -> i64 {
        self.time
    }
}
impl Distance for Circle {
    fn distance(&self, other: &Self) -> f64 {
        let delta_x = self.x - other.x;
        let delta_y = self.y - other.y;
        ((delta_x.pow(2) + delta_y.pow(2)) as f64).sqrt()
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Slider {
    x: i64,
    y: i64,
    time: i64,
    curve: Curve,
    slides: i64,
    length: f64,
}
impl FromStr for Slider {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        let x = match line
            .next()
            .expect("in x assignment in Slider parsing")
            .parse::<i64>()
        {
            Ok(okay) => okay,
            Err(error) => {
                return Err(format!(
                    "in i64 parsing in x assignment in Slider parsing: {}, {}",
                    s, error
                ))
            }
        };
        let y = match line
            .next()
            .expect("in y assignment in Slider parsing")
            .parse::<i64>()
        {
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
impl Position for Slider {
    fn position(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
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
impl Position for Spinner {
    fn position(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}
impl Time for Spinner {
    fn time(&self) -> i64 {
        self.time
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Type {
    object_type: ObjectType, // 0 circle, 1 slider, 3 spinner
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
        let object_type = match (bits[0], bits[1], bits[3]) {
            (true, false, false) => ObjectType::Circle,
            (false, true, false) => ObjectType::Slider,
            (false, false, true) => ObjectType::Spinner,
            _ => {
                return Result::Err(format!(
                    "Invalid object type: {:?}",
                    (bits[0], bits[1], bits[3])
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
impl Distance for Point {
    fn distance(&self, other: &Self) -> f64 {
        let delta_x = self.x - other.x;
        let delta_y = self.y - other.y;
        ((delta_x.pow(2) + delta_y.pow(2)) as f64).sqrt()
    }
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
    fn test_parse_map() {
        let test_maps = std::fs::read_dir("test_maps").unwrap();
        for file in test_maps {
            let path = file.unwrap().path();
            if let Err(error) = parse_map(&path) {
                panic!(
                    "Error parsing test map \"{}\" with error: {}",
                    path.to_str().unwrap(),
                    error
                )
            }
        }
    }
    #[test]
    fn slider_parse() {
        // Linear slider from ver 14.
        assert_eq!(
            "137,72,2985,6,0,L|253:60,1,105.493329791992,2|0,0:2|0:2,0:0:0:0:"
                .parse::<Slider>()
                .unwrap(),
            Slider {
                x: 137,
                y: 72,
                time: 2985,
                curve: Curve {
                    _type: CurveType::Linear,
                    points: vec![Point { x: 253, y: 60 }],
                },
                slides: 1,
                length: 105.493329791992,
            },
        );
        // Perfect slider from ver 14.
        assert_eq!(
            "342,250,2279,2,0,P|282:209|239:210,1,105.493329791992,2|0,0:2|0:2,0:0:0:0:"
                .parse::<Slider>()
                .unwrap(),
            Slider {
                x: 342,
                y: 250,
                time: 2279,
                curve: Curve {
                    _type: CurveType::Perfect,
                    points: vec![Point { x: 282, y: 209 }, Point { x: 239, y: 210 }],
                },
                slides: 1,
                length: 105.493329791992,
            },
        );
        // Bezier slider from ver 14.
        assert_eq!(
            "183,255,4985,2,0,B|170:100|234:201|200:26,1,210.986659583985,2|0,0:2|3:2,0:0:0:0:"
                .parse::<Slider>()
                .unwrap(),
            Slider {
                x: 183,
                y: 255,
                time: 4985,
                curve: Curve {
                    _type: CurveType::Bezier,
                    points: vec![
                        Point { x: 170, y: 100 },
                        Point { x: 234, y: 201 },
                        Point { x: 200, y: 26 }
                    ],
                },
                slides: 1,
                length: 210.986659583985,
            },
        );
        // Linear slider from ver 3.
        assert_eq!(
            "160,320,79368,2,4,L|160:320|160:240,1,70"
                .parse::<Slider>()
                .unwrap(),
            Slider {
                x: 160,
                y: 320,
                time: 79368,
                curve: Curve {
                    _type: CurveType::Linear,
                    points: vec![Point { x: 160, y: 320 }, Point { x: 160, y: 240 }],
                },
                slides: 1,
                length: 70.0,
            },
        );
        // Perfect slider from ver 3. Doesn't exist??
        // Bezier slider from ver 3.
        assert_eq!(
            "192,256,64118,2,4,B|192:256|288:256,3,70"
                .parse::<Slider>()
                .unwrap(),
            Slider {
                x: 192,
                y: 256,
                time: 64118,
                curve: Curve {
                    _type: CurveType::Bezier,
                    points: vec![Point { x: 192, y: 256 }, Point { x: 288, y: 256 }],
                },
                slides: 3,
                length: 70.0,
            },
        );
        // Centripetal slider from ver 7.
        assert_eq!(
            "160,64,139347,6,0,C|244:69|352:64,2,160,4|0|0"
                .parse::<Slider>()
                .unwrap(),
            Slider {
                x: 160,
                y: 64,
                time: 139347,
                curve: Curve {
                    _type: CurveType::Centripetal,
                    points: vec![Point { x: 244, y: 69 }, Point { x: 352, y: 64 }],
                },
                slides: 2,
                length: 160.0,
            },
        );
        // Circle from ver 14.
        assert!("102,240,161,5,2,0:0:0:0:".parse::<Slider>().is_err());
        // Circle from ver 3.
        assert!("96,64,8118,5,4,".parse::<Slider>().is_err());
        // Spinner from ver 14.
        assert!("256,192,29544,12,0,32632,0:2:0:0:"
            .parse::<Slider>()
            .is_err());
        // Spinner from ver 3.
        assert!("256,192,141619,12,0,143869".parse::<Slider>().is_err());
        // Ensure that trim is not used during parsing.
        assert!(
            " 137,72,2985,6,0,L|253:60,1,105.493329791992,2|0,0:2|0:2,0:0:0:0:"
                .parse::<Slider>()
                .is_err()
        );
        // Trim actually needs to be used in the HitSample parsing so a space on the end must be OK.
        assert_eq!(
            "137,72,2985,6,0,L|253:60,1,105.493329791992,2|0,0:2|0:2,0:0:0:0: "
                .parse::<Slider>()
                .unwrap(),
            Slider {
                x: 137,
                y: 72,
                time: 2985,
                curve: Curve {
                    _type: CurveType::Linear,
                    points: vec![Point { x: 253, y: 60 }],
                },
                slides: 1,
                length: 105.493329791992,
            },
        );
        // Ensure panic on empty input.
        assert!("".parse::<Slider>().is_err());
    }
    #[test]
    fn spinner_parse() {
        // Spinner from ver 14.
        assert_eq!(
            "256,192,29544,12,0,32632,0:2:0:0:"
                .parse::<Spinner>()
                .unwrap(),
            Spinner {
                x: 256,
                y: 192,
                time: 29544,
                end_time: 32632,
            },
        );
        // Spinner from ver 3.
        assert_eq!(
            "256,192,141619,12,0,143869".parse::<Spinner>().unwrap(),
            Spinner {
                x: 256,
                y: 192,
                time: 141619,
                end_time: 143869,
            },
        );
        // Circle from ver 14.
        assert!(catch_unwind_silent(|| "102,240,161,5,2,0:0:0:0:".parse::<Spinner>()).is_err());
        // Circle from ver 3.
        assert!(catch_unwind_silent(|| "96,64,8118,5,4,".parse::<Spinner>()).is_err());
        // Slider from ver 14.
        assert!(catch_unwind_silent(|| {
            "137,72,2985,6,0,L|253:60,1,105.493329791992,2|0,0:2|0:2,0:0:0:0:".parse::<Spinner>()
        })
        .is_err());
        // Slider from ver 3.
        assert!(
            catch_unwind_silent(|| "336,96,81368,2,4,L|336:96|336:0,1,70".parse::<Spinner>())
                .is_err()
        );
        // Ensure that there is no trim
        assert!(catch_unwind_silent(|| " 256,192,141619,12,0,143869".parse::<Spinner>()).is_err());
        assert!(catch_unwind_silent(|| "256,192,141619,12,0,143869 ".parse::<Spinner>()).is_err());
        // Ensure panic on empty input.
        assert!(catch_unwind_silent(|| "".parse::<Spinner>()).is_err());
    }
    #[test]
    fn type_parse() {
        let correct = [
            (
                "1",
                Type {
                    object_type: ObjectType::Circle,
                },
            ),
            (
                "2",
                Type {
                    object_type: ObjectType::Slider,
                },
            ),
            (
                "5",
                Type {
                    object_type: ObjectType::Circle,
                },
            ),
            (
                "6",
                Type {
                    object_type: ObjectType::Slider,
                },
            ),
            (
                "8",
                Type {
                    object_type: ObjectType::Spinner,
                },
            ),
            (
                "12",
                Type {
                    object_type: ObjectType::Spinner,
                },
            ),
            (
                "66",
                Type {
                    object_type: ObjectType::Slider,
                },
            ),
            (
                "33",
                Type {
                    object_type: ObjectType::Circle,
                },
            ),
            (
                "104",
                Type {
                    object_type: ObjectType::Spinner,
                },
            ),
            (
                "17",
                Type {
                    object_type: ObjectType::Circle,
                },
            ),
            (
                "86",
                Type {
                    object_type: ObjectType::Slider,
                },
            ),
            (
                "53",
                Type {
                    object_type: ObjectType::Circle,
                },
            ),
            (
                "124",
                Type {
                    object_type: ObjectType::Spinner,
                },
            ),
        ];
        for pair in correct {
            assert_eq!(pair.0.parse::<Type>().unwrap(), pair.1);
        }
        // Ensure no trim.
        assert!(catch_unwind_silent(|| " 1".parse::<Type>()).is_err());
        assert!(catch_unwind_silent(|| "1 ".parse::<Type>()).is_err());
        // Ensure panic on empty input.
        assert!(catch_unwind_silent(|| "".parse::<Type>()).is_err());
    }
    #[test]
    fn curve_parse() {
        // Test bezier curve.
        assert_eq!(
            "B|170:100|234:201|200:26".parse::<Curve>().unwrap(),
            Curve {
                _type: CurveType::Bezier,
                points: vec![
                    Point { x: 170, y: 100 },
                    Point { x: 234, y: 201 },
                    Point { x: 200, y: 26 },
                ]
            },
        );
        // Test linear curve.
        // Linear has simple rules so I supply a fake curve point.
        assert_eq!(
            "L|0:0".parse::<Curve>().unwrap(),
            Curve {
                _type: CurveType::Linear,
                points: vec![Point { x: 0, y: 0 },]
            },
        );
        // Test centripetal curve.
        // TODO: Determine if there are rules for validating centripetal curves
        // (and whether they are followed in real maps).
        assert_eq!(
            "C|240:288|352:240|464:224".parse::<Curve>().unwrap(),
            Curve {
                _type: CurveType::Centripetal,
                points: vec![
                    Point { x: 240, y: 288 },
                    Point { x: 352, y: 240 },
                    Point { x: 464, y: 224 },
                ]
            },
        );
        // Test perfect curve.
        // TODO: Determine if there are rules for validating perfect curves
        // (and whether they are followed in real maps).
        assert_eq!(
            "P|282:209|239:210".parse::<Curve>().unwrap(),
            Curve {
                _type: CurveType::Perfect,
                points: vec![Point { x: 282, y: 209 }, Point { x: 239, y: 210 },]
            },
        );
        // Ensure that trim is not used.
        assert!(catch_unwind_silent(|| " L|0:0".parse::<Curve>()).is_err());
        assert!("L|0:0 ".parse::<Curve>().is_err());
        // Ensure panic on empty input.
        assert!(catch_unwind_silent(|| "".parse::<Curve>()).is_err());
    }
    #[test]
    fn curve_type_parse() {
        // Test correct inputs.
        let correct = [
            ("B", CurveType::Bezier),
            ("C", CurveType::Centripetal),
            ("L", CurveType::Linear),
            ("P", CurveType::Perfect),
        ];
        for pair in correct {
            assert_eq!(pair.0.parse::<CurveType>().unwrap(), pair.1);
        }
        // Ensure that there is no trim during parsing.
        let no_trim = [" B", " C", " L", " P", "B ", "C ", "L ", "P "];
        for line in no_trim {
            assert!(line.parse::<CurveType>().is_err());
        }
        // Ensure panic on empty input.
        assert!("".parse::<CurveType>().is_err());
    }
    #[test]
    fn test_decimal_to_ratio() {
        let correct = [
            ("2147483647", ratio!(2147483647)),
            ("-2147483648", ratio!(-2147483648)),
            ("0", ratio!(0)),
            ("-0.0", ratio!(0)),
            ("0.1", ratio!(1, 10)),
            ("1000.1", ratio!(10001, 10)),
            ("1.0000001", ratio!(10000001, 10000000)),
        ];
        let function = decimal_to_ratio;
        for pair in correct {
            assert_eq!(function(pair.0).unwrap(), pair.1);
        }
        // TODO: Implement panic test cases
    }

    fn catch_unwind_silent<F: FnOnce() -> R + std::panic::UnwindSafe, R>(
        f: F,
    ) -> std::thread::Result<R> {
        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let result = std::panic::catch_unwind(f);
        std::panic::set_hook(prev_hook);
        result
    }
}
