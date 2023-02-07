extern crate num;

use num::rational::Ratio;

macro_rules! ratio {
    ($numer:expr) => {
        num::rational::Ratio::from_integer($numer)
    };
    ($numer:expr, $denom:expr) => {
        num::rational::Ratio::new($numer, $denom)
    };
}

impl Default for Beatmap {
    fn default() -> Self {
        Self {
            file_format: 14,

            //[General]
            audio_filename: None,
            audio_lead_in: 0,
            audio_hash: None, // Deprecated
            preview_time: -1,
            countdown: Countdown::Normal,
            sample_set: SampleSet::Normal,
            stack_leniency: ratio!(7, 10),
            mode: Mode::Osu,
            letterbox_in_breaks: false,
            story_fire_in_front: true, // Deprecated
            use_skin_sprites: false,
            always_show_play_field: false, // Deprecated
            overlay_position: OverlayPosition::NoChange,
            skin_preference: None,
            epilepsy_warning: false,
            countdown_offset: 0,
            special_style: false,
            widescreen_storyboard: false,
            samples_match_playback_rate: false,

            //[Editor]
            bookmarks: None,
            distance_spacing: None,
            beat_divisor: None,
            grid_size: None,
            timeline_zoom: None,

            //[Metadata]
            title: None,
            title_unicode: None,
            artist: None,
            artist_unicode: None,
            creator: None,
            version: None,
            source: None,
            tags: None,
            beatmap_id: None,
            beatmap_set_id: None,

            //[Difficulty]
            hpdrain_rate: None,
            circle_size: None,
            overall_difficulty: None,
            approach_rate: None,
            slider_multiplier: None,
            slider_tick_rate: None,

            //[Events]
            // I will be omitting the story board events because they are complicated.
            background: None,
            breaks: None,

            //[TimingPoints]
            timing_points: None,

            //[Colo*rs]
            colors: None,
            //[HitObjects]
            hit_objects: None,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ObjectType {
    Circle,
    Slider,
    Spinner,
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
            "AudioFilename" => map.audio_filename = Some(value.to_string()),
            "AudioLeadIn" => {
                map.audio_lead_in = value
                    .parse::<i64>()
                    .expect("at map.audio_lead_in assignment in tabular")
            }
            "AudioHash" => map.audio_hash = Some(value.to_string()),
            "PreviewTime" => {
                map.preview_time = value
                    .parse::<i64>()
                    .expect("at map.preview_time assignment in tabular")
            }
            "Countdown" => {
                map.countdown = value
                    .parse::<Countdown>()
                    .expect("at map.countdown assignment in tabular")
            }
            "SampleSet" => {
                map.sample_set = value
                    .parse::<SampleSet>()
                    .expect("at map.sample_set assignment in tabular")
            }
            "StackLeniency" => {
                map.stack_leniency =
                    decimal_to_ratio(value).expect("at map.stack_leniency assignment in tabular")
            }
            "Mode" => {
                map.mode = value
                    .parse::<Mode>()
                    .expect("at map.mode assignment in tabular")
            }
            "LetterboxInBreaks" => {
                map.letterbox_in_breaks = match value {
                    "1" => true,
                    "0" => false,
                    _ => {
                        return Result::Err(format!(
                            "at map.letterbox_in_breaks assignment in tabular: \"{}\"",
                            value
                        ))
                    }
                };
            }
            "StoryFireInFront" => {
                map.story_fire_in_front = match value {
                    "1" => true,
                    "0" => false,
                    _ => {
                        return Result::Err(format!(
                            "at map.story_fire_in_front assignment in tabular: \"{}\"",
                            value
                        ))
                    }
                };
            }
            "UseSkinSprites" => {
                map.use_skin_sprites = match value {
                    "1" => true,
                    "0" => false,
                    _ => {
                        return Result::Err(format!(
                            "at map.use_skin_sprites assignment in tabular: \"{}\"",
                            value
                        ))
                    }
                };
            }
            "AlwaysShowPlayField" => {
                map.always_show_play_field = match value {
                    "1" => true,
                    "0" => false,
                    _ => {
                        return Result::Err(format!(
                            "at map.always_show_playfield assignment in tabular: \"{}\"",
                            value
                        ))
                    }
                };
            }
            "OverlayPosition" => {
                map.overlay_position = value
                    .parse::<OverlayPosition>()
                    .expect("at map.overlay_position assignment in tabular")
            }
            "SkinPreference" => map.skin_preference = Some(value.to_string()),
            "EpilepsyWarning" => {
                map.epilepsy_warning = match value {
                    "1" => true,
                    "0" => false,
                    _ => {
                        return Result::Err(format!(
                            "at map.epilepsy_warning assignment in tabular: \"{}\"",
                            value
                        ))
                    }
                };
            }
            "CountdownOffset" => {
                map.countdown_offset = value
                    .parse::<i64>()
                    .expect("at map.countdown_offset assignment in tabular")
            }
            "SpecialStyle" => {
                map.special_style = match value {
                    "1" => true,
                    "0" => false,
                    _ => {
                        return Result::Err(format!(
                            "at map.special_style assignment in tabular: \"{}\"",
                            value
                        ))
                    }
                };
            }
            "WidescreenStoryboard" => {
                map.widescreen_storyboard = match value {
                    "1" => true,
                    "0" => false,
                    _ => {
                        return Result::Err(format!(
                            "at map.widescreen_storyboard assignment in tabular: \"{}\"",
                            value
                        ))
                    }
                };
            }
            "SamplesMatchPlaybackRate" => {
                map.samples_match_playback_rate = match value {
                    "1" => true,
                    "0" => false,
                    _ => {
                        return Result::Err(format!(
                            "at map.samples_match_playback_rate assignment in tabular: \"{}\"",
                            value
                        ))
                    }
                };
            }
            "Bookmarks" => {
                let mut bookmarks = Vec::new();
                for i in value.split(',') {
                    bookmarks.push(i.parse::<i64>().expect("at bookmarks pushing in tabular"));
                }
                map.bookmarks = Some(bookmarks);
            }
            "DistanceSpacing" => {
                map.distance_spacing = Some(
                    decimal_to_ratio(value).expect("at map.distance_spacing assignment in tabular"),
                )
            }
            "BeatDivisor" => {
                map.beat_divisor = Some(
                    value
                        .parse::<i64>()
                        .expect("at map.beat_divisor assignment in tabular"),
                )
            }
            "GridSize" => {
                map.grid_size = Some(
                    value
                        .parse::<i64>()
                        .expect("at map.grid_size assignment in tabular"),
                )
            }
            "TimelineZoom" => {
                map.timeline_zoom = Some(
                    decimal_to_ratio(value).expect("at map.timeline_zoom assignment in tabular"),
                )
            }
            "Title" => map.title = Some(value.to_string()),
            "TitleUnicode" => map.title_unicode = Some(value.to_string()),
            "Artist" => map.artist = Some(value.to_string()),
            "ArtistUnicode" => map.artist_unicode = Some(value.to_string()),
            "Creator" => map.creator = Some(value.to_string()),
            "Version" => map.version = Some(value.to_string()),
            "Source" => map.source = Some(value.to_string()),
            "Tags" => {
                if value.trim().is_empty() {
                    continue;
                }
                let mut tags = Vec::new();
                for tag in value.split(' ') {
                    tags.push(tag.to_string());
                }
                map.tags = Some(tags);
            }
            "BeatmapID" => {
                map.beatmap_id = Some(
                    value
                        .parse::<i64>()
                        .expect("at map.beatmap_id assignment in tabular"),
                )
            }
            "BeatmapSetID" => {
                map.beatmap_set_id = Some(
                    value
                        .parse::<i64>()
                        .expect("at map.beatmap_set_id assignment in tabular"),
                )
            }
            "HPDrainRate" => {
                map.hpdrain_rate = Some(
                    decimal_to_ratio(value).expect("at map.hpdrain_rate assignment in tabular"),
                )
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
            "EditorBookmarks" => {}
            _ => return Result::Err(format!("Unknown key: {}", key)),
        }
    }
    let mut mixed = content
        .next()
        .expect("at mixed assignment after tabular")
        .split("[TimingPoints]");
    let events = mixed.next().expect("at events assignment").lines();
    for line in events {
        if line.starts_with("//") || line.trim().is_empty() {
            // Filter out junk lines.
            continue;
        }
        match line.chars().next().expect("at match first event line char") {
            '0' => {
                // Background event.
                map.background = Some(
                    line.parse::<Background>()
                        .expect("at map.background assignment"),
                );
            }
            '2' => {
                // Break event.
                if map.breaks.is_none() {
                    map.breaks = Some(Vec::new());
                }
                map.breaks
                    .as_mut()
                    .expect("at map.breaks grabbing")
                    .push(line.parse::<Break>().expect("at map.breaks pushing"));
            }
            _ => {}
        }
    }
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
        let colours = mixed.next().expect("at colours assignment").lines();
        for line in colours {
            if line.trim().is_empty() {
                // Filter out junk lines.
                continue;
            }
            if map.colors.is_none() {
                map.colors = Some(Vec::new());
            }
            map.colors
                .as_mut()
                .expect("at map.colors grabbing")
                .push(line.parse::<Color>().expect("at map.colors pushing"));
        }
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
    // Verify hit objects

    Ok(map)
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
