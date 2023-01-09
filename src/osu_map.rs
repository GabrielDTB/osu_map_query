use crate::read_lines;

#[derive(Debug)]
pub struct Map {
    cs: f64,
    ar: f64,
    pub circles: Vec<Circle>,
}
impl Map {
    pub fn new(path: &str) -> Map {
        // .osu files are not very large, so we can afford to read the entire file into memory.
        if let Ok(mut lines) = read_lines(path) {
            // This is probably not best practice?
            let line = lines.next().unwrap().unwrap();
            if !line.starts_with("osu file format v") {
                panic!("Invalid .osu file format.");
            }
            let mut cs = std::f64::NAN;
            let mut ar = std::f64::NAN;
            let mut circles = Vec::new();
            enum Section {
                Preamble,
                General,
                Editor,
                Metadata,
                Difficulty,
                Events,
                TimingPoints,
                Colours,
                HitObjects,
            }
            let mut section = Section::Preamble;
            for line in lines {
                if let Ok(line) = line {
                    match line.as_str() {
                        "[General]" => {
                            section = Section::General;
                        }
                        "[Editor]" => {
                            section = Section::Editor;
                        }
                        "[Metadata]" => {
                            section = Section::Metadata;
                        }
                        "[Difficulty]" => {
                            section = Section::Difficulty;
                        }
                        "[Events]" => {
                            section = Section::Events;
                        }
                        "[TimingPoints]" => {
                            section = Section::TimingPoints;
                        }
                        "[Colours]" => {
                            section = Section::Colours;
                        }
                        "[HitObjects]" => {
                            section = Section::HitObjects;
                        }
                        "" => {
                            continue;
                        }
                        _ => {}
                    }
                    match section {
                        Section::Preamble => {
                            continue;
                        }
                        Section::General => {
                            continue;
                        }
                        Section::Editor => {
                            continue;
                        }
                        Section::Metadata => {
                            continue;
                        }
                        Section::Difficulty => {
                            if line.starts_with("CircleSize:") {
                                cs = line.split(':').last().unwrap().parse::<f64>().unwrap();
                            } else if line.starts_with("ApproachRate:") {
                                ar = line.split(':').last().unwrap().parse::<f64>().unwrap();
                            }
                        }
                        Section::Events => {
                            continue;
                        }
                        Section::TimingPoints => {
                            continue;
                        }
                        Section::Colours => {
                            continue;
                        }
                        Section::HitObjects => {
                            if circles.capacity() == circles.len() {
                                circles.reserve(100);
                            }
                            match line.matches(",").count() {
                                5 => {
                                    let mut split = line.split(',');
                                    let x = split.next().unwrap().parse::<i64>().unwrap();
                                    let y = split.next().unwrap().parse::<i64>().unwrap();
                                    let ms = split.next().unwrap().parse::<i64>().unwrap();
                                    circles.push(Circle::new(x, y, ms));
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }

            match (cs.is_nan(), ar.is_nan()) {
                (false, false) => Self { cs, ar, circles },
                (true, false) => {
                    panic!("CS was unable to be parsed during map parsing of {}.", path)
                }
                (false, true) => {
                    panic!("AR was unable to be parsed during map parsing of {}.", path)
                }
                (true, true) => panic!(
                    "CS and AR were unable to be parsed during map parsing of {}.",
                    path
                ),
            }
        } else {
            panic!("Incorrect file path, \"{}\"", path)
        }
    }
}

pub fn cs_to_osu_px(cs: f64) -> f64 {
    54.4 - 4.48 * cs
}

pub fn ar_to_ms(ar: f64) -> i64 {
    let scalar = if ar < 5.0 { 120.0 } else { 150.0 };
    1200 + (scalar * (5.0 - ar)) as i64
}

fn distance(p1: Point, p2: Point) -> f64 {
    let delta_x = p1.x - p2.x;
    let delta_y = p1.y - p2.y;
    (((delta_x.powi(2)) + (delta_y.powi(2))) as f64).sqrt()
}

fn velocity(p1: Point, p2: Point, delta_time: i64) -> f64 {
    distance(p1, p2) / delta_time as f64
}

fn angle(p1: Point, p2: Point, p3: Point) -> f64 {
    let a = distance(p1, p2);
    let b = distance(p2, p3);
    let c = distance(p1, p3);
    let cos = (a.powi(2) + b.powi(2) - c.powi(2)) / (2.0 * a * b);
    cos.acos()
}

/// Returns a vector of the velocities between circles in a map.
/// NAN is used where more time than objects are shown on screen
/// has passed since the last circle, since velocity is meaningless here.
pub fn velocities(map: &Map) -> Vec<f64> {
    let mut velocities = vec![std::f64::NAN];
    for i in 0..map.circles.len() - 1 {
        let c1 = map.circles[i];
        let c2 = map.circles[i + 1];
        let delta_time = c2.ms - c1.ms;
        if delta_time > ar_to_ms(map.ar) {
            velocities.push(std::f64::NAN);
        } else {
            velocities.push(velocity(c1.into(), c2.into(), delta_time));
        }
    }
    velocities
}

pub fn optimal_paths(map: &Map) -> Vec<Circle> {
    fn optimal_point(start: Point, end: Point, radius: f64) -> Point {
        let big_x = start.x - end.x;
        let big_y = start.y - end.y;
        let scale = radius / distance(start, end);
        let small_x = big_x as f64 * scale;
        let small_y = big_y as f64 * scale;
        let x = end.x + small_x;
        let y = end.y + small_y;
        Point { x, y }
    }
    let mut paths = Vec::new();
    let radius = cs_to_osu_px(map.cs);
    let start = map.circles[1];
    let end = map.circles[0];
    paths.push(Circle::from_point(
        optimal_point(start.into(), end.into(), radius),
        end.ms,
    ));
    for i in 1..map.circles.len() {
        let start = *paths.last().unwrap();
        let end = map.circles[i];
        paths.push(Circle::from_point(
            optimal_point(start.into(), end.into(), radius),
            end.ms,
        ));
    }
    paths
}

pub fn optimal_velocities(map: &Map) -> Vec<f64> {
    let mut velocities = vec![std::f64::NAN];
    let paths = optimal_paths(map);
    for i in 0..map.circles.len() - 1 {
        if let [p1, p2] = paths[i..=i + 1] {
            if let [c1, c2] = map.circles[i..=i + 1] {
                let delta_time = c2.ms - c1.ms;
                if delta_time > ar_to_ms(map.ar) {
                    velocities.push(std::f64::NAN);
                } else {
                    velocities.push(velocity(p1.into(), p2.into(), delta_time));
                }
            }
        }
    }
    velocities
}

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    x: i64,
    y: i64,
    ms: i64,
}
impl Circle {
    fn new(x: i64, y: i64, ms: i64) -> Self {
        Self { x, y, ms }
    }
    fn from_point(point: Point, ms: i64) -> Self {
        Self {
            x: point.x as i64,
            y: point.y as i64,
            ms,
        }
    }
}

impl std::ops::Add for Circle {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            ms: self.ms + other.ms,
        }
    }
}

impl std::ops::Sub for Circle {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            ms: self.ms - other.ms,
        }
    }
}

trait RoundEven {
    fn round_even(self) -> Self;
}

impl RoundEven for i64 {
    fn round_even(self) -> Self {
        let x = self;
        if x % 2 == 0 {
            x
        } else {
            x - 1
        }
    }
}

pub fn absolute_to_sequential(circles: &Vec<Circle>) -> Vec<Circle> {
    let mut sequential = Vec::new();
    let mut last = circles[0];
    sequential.push(last);
    for circle in circles {
        sequential.push(*circle - last);
        last = *circle;
    }
    sequential
}

pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}

#[derive(Debug, Clone)]
pub struct Moves {
    pub timescale: i64,
    pub moves: Vec<Move>,
}

impl Moves {
    pub fn new(sequential: &Vec<Circle>) -> Self {
        let timescale = find_timescale(sequential);
        let mut moves = Vec::new();
        for window in sequential.windows(3) {
            let c1 = window[0];
            let c2 = window[1];
            let c3 = window[2];
            moves.push(Move {
                angle: rad_to_deg(angle(c1.into(), c2.into(), c3.into())),
                distance: distance(c2.into(), c3.into()),
                time: c2.ms / timescale,
            });
        }
        Self { timescale, moves }
    }
}

pub fn find_timescale(sequential: &Vec<Circle>) -> i64 {
    let mut tally = std::collections::BTreeMap::new();
    for circle in sequential {
        *tally.entry(circle.ms.round_even()).or_insert(0) += 1;
    }
    tally.into_iter().max_by_key(|&(_, count)| count).unwrap().0
    // sequential
    //     .iter()
    //     .map(|x| x.ms)
    //     .collect::<std::collections::BTreeMap<i64>>()
    //     .unwrap()
}

#[derive(Debug, Copy, Clone)]
/// A move is a qualitative measure of what happens between circles.
/// They are not meant to describe exactly what happens, only provide
/// useful statistics in one place.
pub struct Move {
    angle: f64,
    distance: f64,
    time: i64,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl From<Circle> for Point {
    fn from(circle: Circle) -> Self {
        Self {
            x: circle.x as f64,
            y: circle.y as f64,
        }
    }
}
