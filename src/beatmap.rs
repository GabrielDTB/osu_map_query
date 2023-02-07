pub mod chart;
pub mod customization;
pub mod difficulty;
pub mod editor;
pub mod filedata;
pub mod metadata;
pub mod mode;
pub mod parse;
pub mod shared;

use chart::Chart;
use customization::Customization;
use difficulty::Difficulty;
use editor::Editor;
use filedata::Filedata;
use metadata::Metadata;
use mode::Mode;

pub struct Beatmap {
    mode: Option<Mode>,
    customization: Option<Box<Customization>>,
    difficulty: Option<Box<Difficulty>>,
    editor: Option<Box<Editor>>,
    filedata: Option<Box<Filedata>>,
    hit_objects: Option<Box<Chart>>,
    metadata: Option<Box<Metadata>>,
}
impl Beatmap {
    pub fn parse(
        path: &std::path::Path,
        parse_customization: bool,
        parse_difficulty: bool,
        parse_editor: bool,
        parse_filedata: bool,
        parse_hit_objects: bool,
        parse_metadata: bool,
    ) -> Result<Beatmap, String> {
        // Try to open file
        let mut file = match std::fs::File::open(path) {
            Err(why) => return Result::Err(format!("couldn't open: {}", why)),
            Ok(file) => file,
        };
        let mut content = String::new();
        // Assign contents of file to string.
        match std::io::Read::read_to_string(&mut file, &mut content) {
            Ok(_) => {}
            Err(error) => panic!(
                "Couldn't open file: '{}' with error: '{}'",
                path.to_string_lossy(),
                error
            ),
        };
        let mut lines = content.lines().map(|s| s.trim()).filter(|s| !s.is_empty());

        let mut file_format = None;
        // Parse header
        loop {
            let line = match lines.next() {
                None => {
                    return Result::Err(format!(
                        "unexpected end of file while parsing header section of {}",
                        path.display()
                    ))
                }
                Some(line) => line,
            };
            if line == "[General]" {
                break;
            }
            if line.contains("format") {
                // rewrite to use errors instead of unwrap
                file_format = Some(line.split_once("v").unwrap().1.parse::<u8>().unwrap());
            }
        }

        // Parse general
        let mut next_section_is_editor = true;
        loop {
            let line = match lines.next() {
                None => {
                    return Result::Err(format!(
                        "unexpected end of file while parsing general section of {}",
                        path.display()
                    ))
                }
                Some(line) => line,
            };
            if line == "[Editor]" {
                break;
            } else if line == "[Metadata]" {
                next_section_is_editor = false;
                break;
            }
            // Do real parsing here
        }

        // Parse editor
        if next_section_is_editor {
            loop {
                let line = match lines.next() {
                    None => {
                        return Result::Err(format!(
                            "unexpected end of file while parsing editor section of {}",
                            path.display()
                        ))
                    }
                    Some(line) => line,
                };
                if line == "[Metadata]" {
                    break;
                }
                // Do real parsing here
            }
        }

        // Parse metadata
        loop {
            let line = match lines.next() {
                None => {
                    return Result::Err(format!(
                        "unexpected end of file while parsing metadata section of {}",
                        path.display()
                    ))
                }
                Some(line) => line,
            };
            if line == "[Difficulty]" {
                break;
            }
            // Do real parsing here
        }

        // Parse difficulty
        loop {
            let line = match lines.next() {
                None => {
                    return Result::Err(format!(
                        "unexpected end of file while parsing difficulty section of {}",
                        path.display()
                    ))
                }
                Some(line) => line,
            };
            if line == "[Events]" {
                break;
            }
            // Do real parsing here
        }

        // Parse events
        loop {
            let line = match lines.next() {
                None => {
                    return Result::Err(format!(
                        "unexpected end of file while parsing events section of {}",
                        path.display()
                    ))
                }
                Some(line) => line,
            };
            if line == "[TimingPoints]" {
                break;
            }
            // Do real parsing here
        }

        // Parse timing points
        let next_section_is_colours = true;
        loop {
            let line = match lines.next() {
                None => {
                    return Result::Err(format!(
                        "unexpected end of file while parsing timing points section of {}",
                        path.display()
                    ))
                }
                Some(line) => line,
            };
            if line == "[Colours]" {
                break;
            } else if line == "[HitObjects]" {
                next_section_is_colours = false;
                break;
            }
            // Do real parsing here
        }

        // Parse colours
        if next_section_is_colours {
            loop {
                let line = match lines.next() {
                    None => {
                        return Result::Err(format!(
                            "unexpected end of file while parsing colours section of {}",
                            path.display()
                        ))
                    }
                    Some(line) => line,
                };
                if line == "[HitObjects]" {
                    break;
                }
                // Do real parsing here
            }
        }

        // Parse hit objects
        loop {
            let line = match lines.next() {
                None => break,
                Some(line) => line,
            };
            // Do real parsing here
        }

        Ok(Beatmap {
            mode: None,
            customization: None,
            difficulty: None,
            editor: None,
            filedata: None,
            hit_objects: None,
            metadata: None,
        })
    }
}
