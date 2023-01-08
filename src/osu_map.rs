use crate::read_lines;

#[derive(Debug)]
pub struct Map {
    cs: f64,
    ar: f64,
    circles: Vec<Circle>,
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

pub fn ar_to_ms(ar: f64) -> f64 {
    1200.0 + {
        if ar < 5.0 {
            120.0 * (5.0 - ar)
        } else {
            150.0 * (5.0 - ar)
        }
    }
}

/// Returns a vector of the velocities between circles in a map.
/// NAN is used where more time than objects are shown on screen
/// has passed since the last circle, since velocity is meaningless here.
pub fn velocities(map: &Map) -> Vec<f64> {
    let mut velocities = Vec::new();
    for i in 0..map.circles.len() - 1 {
        if let [c1, c2] = map.circles[i..=i + 1] {
            let delta_time = (c2.ms - c1.ms) as f64;
            if delta_time > ar_to_ms(map.ar) {
                velocities.push(std::f64::NAN);
            } else {
                let delta_x = c2.x - c1.x;
                let delta_y = c2.y - c1.y;
                let distance = ((delta_x.pow(2) + delta_y.pow(2)) as f64).sqrt();
                velocities.push(distance / delta_time);
            }
        }
    }
    velocities
}

pub fn optimal_paths(map: &Map) -> Vec<(i64, i64)> {
    fn optimal_point(start: (i64, i64), end: (i64, i64), radius: f64) -> (i64, i64) {
        let big_x = start.0 - end.0;
        let big_y = start.1 - end.1;
        let distance = ((big_x.pow(2) + big_y.pow(2)) as f64).sqrt();
        let scale = radius / distance;
        let small_x = big_x as f64 * scale;
        let small_y = big_y as f64 * scale;
        let final_x = small_x.round() as i64 + end.0;
        let final_y = small_y.round() as i64 + end.1;
        (final_x, final_y)
    }
    let mut paths = Vec::new();
    let radius = cs_to_osu_px(map.cs);
    let p1 = (map.circles[1].x, map.circles[1].y);
    let p2 = (map.circles[0].x, map.circles[0].y);
    paths.push(optimal_point(p1, p2, radius));
    for i in 1..map.circles.len() {
        let p1 = *paths.last().unwrap();
        let p2 = (map.circles[i].x, map.circles[i].y);
        paths.push(optimal_point(p1, p2, radius));
    }
    paths
}

pub fn optimal_velocities(map: &Map) -> Vec<f64> {
    let mut velocities = Vec::new();
    let paths = optimal_paths(map);
    for i in 0..map.circles.len() - 1 {
        if let [p1, p2] = paths[i..=i + 1] {
            if let [c1, c2] = map.circles[i..=i + 1] {
                let delta_time = (c2.ms - c1.ms) as f64;
                if delta_time > ar_to_ms(map.ar) {
                    velocities.push(std::f64::NAN);
                } else {
                    let delta_x = p2.0 - p1.0;
                    let delta_y = p2.1 - p1.1;
                    let distance = ((delta_x.pow(2) + delta_y.pow(2)) as f64).sqrt();
                    velocities.push(distance / delta_time);
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
}
