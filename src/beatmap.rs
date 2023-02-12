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
use customization::countdown::Countdown;
use customization::overlay_position::OverlayPosition;
use customization::*;
use difficulty::Difficulty;
use editor::Editor;
use filedata::Filedata;
use metadata::Metadata;
use mode::Mode;
use rayon::prelude::*;
use shared::sample_set::SampleSet;

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
        // Find which sections exist in the file
        fn is_in(sub: &str, content: &String) -> bool {
            content.contains(sub)
        }
        let all_sections = [
            "[General]",
            "[Editor]",
            "[Metadata]",
            "[Difficulty]",
            "[Events]",
            "[TimingPoints]",
            "[Colours]",
            "[HitObjects]",
        ];
        let existing_sections: Vec<&str> = all_sections
            .into_par_iter()
            .filter(|sub| is_in(sub, &content))
            .collect();
        // Split the file into sections
        let mut split_sections = Vec::with_capacity(9);
        let mut remainder = content.as_str();
        for section in existing_sections {
            let (split, rest) = remainder.split_once(section).unwrap();
            split_sections.push(split);
            remainder = rest;
        }
        split_sections.push(remainder);
        let split_sections: Vec<Vec<&str>> = split_sections
            .into_par_iter()
            .map(|s| {
                s.par_lines()
                    .map(|l| l.trim())
                    .filter(|l| !l.is_empty())
                    .collect::<Vec<&str>>() // Remove empty lines and trim whitespace
            })
            .collect();
        existing_sections.insert(0, "Preamble");
        // Parse each section in parallel
        fn delegate(heading: &str, content: Vec<&str>) -> Result<ReturnKey, String> {
            match heading {
                "Preamble" => Ok(ReturnKey::Preamble(parse_preamble(content))),
                "General" => Ok(ReturnKey::General(parse_general(content))),
                "Editor" => Ok(ReturnKey::Editor(parse_editor(content))),
                "Metadata" => Ok(ReturnKey::Metadata(parse_metadata(content))),
                "Difficulty" => Ok(ReturnKey::Difficulty(parse_difficulty(content))),
                "Events" => Ok(ReturnKey::Events(parse_events(content))),
                "TimingPoints" => Ok(ReturnKey::TimingPoints(parse_timing_points(content))),
                "Colours" => Ok(ReturnKey::Colours(parse_colours(content))),
                "HitObjects" => Ok(ReturnKey::HitObjects(parse_hit_objects(content))),
                _ => Err(format!("Unknown section: {}", heading)),
            }
        }
        let parsed_sections =
            existing_sections
                .into_par_iter()
                .zip(split_sections)
                .map(|(h, c)| {
                    delegate(h, c).unwrap_or_else(|e| {
                        panic!("Error parsing section '{}': {}", h, e.to_string())
                    })
                });

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

fn parse_preamble(content: Vec<&str>) -> Vec<PreambleKey> {
    vec![PreambleKey::FileFormat(14)]
}
fn parse_general(content: Vec<&str>) -> Vec<GeneralKey> {
    vec![GeneralKey::AudioFilename("".to_string())]
}
fn parse_editor(content: Vec<&str>) -> Vec<EditorKey> {
    vec![EditorKey::None]
}
fn parse_metadata(content: Vec<&str>) -> Vec<MetadataKey> {
    vec![MetadataKey::None]
}
fn parse_difficulty(content: Vec<&str>) -> Vec<DifficultyKey> {
    vec![DifficultyKey::None]
}
fn parse_events(content: Vec<&str>) -> Vec<EventsKey> {
    vec![EventsKey::None]
}
fn parse_timing_points(content: Vec<&str>) -> Vec<TimingPointsKey> {
    vec![TimingPointsKey::None]
}
fn parse_colours(content: Vec<&str>) -> Vec<ColoursKey> {
    vec![ColoursKey::None]
}
fn parse_hit_objects(content: Vec<&str>) -> Vec<HitObjectsKey> {
    vec![HitObjectsKey::None]
}

enum ReturnKey {
    Preamble(Vec<PreambleKey>),
    General(Vec<GeneralKey>),
    Editor(Vec<EditorKey>),
    Metadata(Vec<MetadataKey>),
    Difficulty(Vec<DifficultyKey>),
    Events(Vec<EventsKey>),
    TimingPoints(Vec<TimingPointsKey>),
    Colours(Vec<ColoursKey>),
    HitObjects(Vec<HitObjectsKey>),
}

enum PreambleKey {
    FileFormat(u8),
}

enum GeneralKey {
    AudioFilename(String),
    AudioLeadIn(i64),
    AudioHash(String), // Deprecated
    PreviewTime(i64),
    Countdown(Countdown),
    SampleSet(SampleSet),
    StackLeniency(u8),
    Mode(Mode),
    LetterboxInBreaks(bool),
    StoryFireInFront(bool), // Deprecated
    UseSkinSprites(bool),
    AlwaysShowPlayField(bool), // Deprecated
    OverlayPosition(OverlayPosition),
    SkinPreference(String),
    EpilepsyWarning(bool),
    CountdownOffset(i64),
    SpecialStyle(bool),
    WidescreenStoryboard(bool),
    SamplesMatchPlaybackRate(bool),
}

enum EditorKey {
    None,
}

enum MetadataKey {
    None,
}

enum DifficultyKey {
    None,
}

enum EventsKey {
    None,
}

enum TimingPointsKey {
    None,
}

enum ColoursKey {
    None,
}

enum HitObjectsKey {
    None,
}

// enum PreambleKey {
// FileFormat(u8),
// }

// enum GeneralKey {
// AudioFilename(&str),
// AudioLeadIn(i64),
// AudioHash(&str), // Deprecated
// PreviewTime(i64),
// Countdown(Countdown),
// SampleSet(SampleSet),
// StackLeniency(),
// mode: Mode::Osu,
// letterbox_in_breaks: false,
// story_fire_in_front: true, // Deprecated
// use_skin_sprites: false,
// always_show_play_field: false, // Deprecated
// overlay_position: OverlayPosition::NoChange,
// skin_preference: None,
// epilepsy_warning: false,
// countdown_offset: 0,
// special_style: false,
// widescreen_storyboard: false,
// samples_match_playback_rate: false,
// }
// enum EditorKey {
// bookmarks: None,
// distance_spacing: None,
// beat_divisor: None,
// grid_size: None,
// timeline_zoom: None,
// }
// enum MetadataKey {
// title: None,
// title_unicode: None,
// artist: None,
// artist_unicode: None,
// creator: None,
// version: None,
// source: None,
// tags: None,
// beatmap_id: None,
// beatmap_set_id: None,
// }
// enum DifficultyKey {
// HPDrainRate: u8,
// CircleSize: u8,
// OverallDifficulty: u8,
// ApproachRate: u8,
// SliderMultiplier: Ratio<i64>,
// SliderTickRate: Ratio<i64>,
// }
