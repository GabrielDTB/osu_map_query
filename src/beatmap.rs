// Screen size: 640x480 osu px
// Play area: 510x385 osu px
// Center of playfield: 256x192 osu px
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
