mod colours;
mod difficulty;
mod editor;
mod events;
mod general;
mod hit_objects;
mod metadata;
mod preamble;
mod timing_points;

use crate::beatmap::chart::hit_object::hit_sample::SampleSet;
use crate::beatmap::countdown::Countdown;
use crate::beatmap::overlay_position::OverlayPosition;
use crate::beatmap::Beatmap;
use crate::beatmap::Mode;
use rayon::prelude::*;
trait Parse {
    fn parse(
        data: &str,
        customization: bool,
        difficulty: bool,
        editor: bool,
        filedata: bool,
        hit_objects: bool,
        metadata: bool,
    ) -> Result<Self, String>
    where
        Self: Sized;
}
/*
mode requires [General] // Always parse
customization requires [General], [Events], [Colours] // Events and colours only get parsed here
chart requires [HitObjects], [TimingPoints], [Difficulty] // TimingPoints and HitObjects only get parsed here
editor requires [Editor] // Editor only gets parsed here
filedata requires [General] // Always parses anyway
difficulty requires [Difficulty] // Difficulty gets parsed with difficulty or chart
metadata requires [Metadata] // Metadata only gets parsed here
*/
impl Parse for Beatmap {
    fn parse(
        data: &str,
        customization: bool,
        difficulty: bool,
        editor: bool,
        filedata: bool,
        hit_objects: bool,
        metadata: bool,
    ) -> Result<Self, String> {
        // Assemble active sections
        let mut active_sections = Vec::with_capacity(9);
        active_sections.push("Preamble");
        active_sections.push("[General]");
        if editor {
            active_sections.push("[Editor]");
        }
        if metadata {
            active_sections.push("[Metadata]");
        }
        if difficulty || hit_objects {
            active_sections.push("[Difficulty]");
        }
        if customization {
            active_sections.push("[Events]");
        }
        if hit_objects {
            active_sections.push("[TimingPoints]");
        }
        if customization {
            active_sections.push("[Colours]");
        }
        if hit_objects {
            active_sections.push("[HitObjects]");
        }

        // Find which sections exist in the data
        fn is_in(sub: &str, data: &str) -> bool {
            data.contains(sub)
        }
        let existing_sections: Vec<&str> = active_sections
            .into_par_iter()
            .filter(|sub| is_in(sub, &data))
            .collect();

        // Split the data into sections
        let mut split_sections = Vec::with_capacity(existing_sections.len());
        let mut remainder = data;
        for section in &existing_sections[1..] {
            let (split, rest) = remainder.split_once(section).unwrap();
            split_sections.push(split);
            remainder = rest;
        }
        split_sections.push(remainder);

        // Remove empty lines and trim whitespace
        let split_sections: Vec<Vec<&str>> = split_sections
            .into_par_iter()
            .map(|s| {
                s.par_lines()
                    .map(|l| l.trim())
                    .filter(|l| !l.is_empty())
                    .collect::<Vec<&str>>()
            })
            .collect();

        // Parse each section in parallel
        fn delegate(heading: &str, content: Vec<&str>) -> Result<AllReturns, String> {
            match heading {
                "[Colours]" => Ok(AllReturns::Colours(colours::parse(content)?)),
            }
        }
        let parsed_sections = existing_sections
            .into_par_iter()
            .zip(split_sections)
            .map(|(h, c)| {
                delegate(h, c)
                    .unwrap_or_else(|e| panic!("Error parsing section '{}': {}", h, e.to_string()))
            })
            .collect();
    }
}

enum AllReturns {
    Colours(Vec<colours::Returns>),
}

// enum PreambleKey {
//     FileFormat(u8),
// }

// enum GeneralKey {
//     AudioFilename(String),
//     AudioLeadIn(i64),
//     AudioHash(String), // Deprecated
//     PreviewTime(i64),
//     Countdown(Countdown),
//     SampleSet(SampleSet),
//     StackLeniency(u8),
//     Mode(Mode),
//     LetterboxInBreaks(bool),
//     StoryFireInFront(bool), // Deprecated
//     UseSkinSprites(bool),
//     AlwaysShowPlayField(bool), // Deprecated
//     OverlayPosition(OverlayPosition),
//     SkinPreference(String),
//     EpilepsyWarning(bool),
//     CountdownOffset(i64),
//     SpecialStyle(bool),
//     WidescreenStoryboard(bool),
//     SamplesMatchPlaybackRate(bool),
// }

// enum EditorKey {
//     None,
// }

// enum MetadataKey {
//     None,
// }

// enum DifficultyKey {
//     None,
// }

// enum EventsKey {
//     None,
// }

// enum TimingPointsKey {
//     None,
// }

// enum HitObjectsKey {
//     None,
// }
