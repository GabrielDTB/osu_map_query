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

/// Complete map data for a .osu file.
/// Arranged like map ver 14.
#[derive(Debug, Clone, PartialEq)]
pub struct Beatmap {
    pub file_format: i64,
    //[General]
    pub audio_filename: Option<String>,
    pub audio_lead_in: i64,
    pub audio_hash: Option<String>, // Deprecated
    pub preview_time: i64,
    pub countdown: Countdown,
    pub sample_set: SampleSet,
    pub stack_leniency: Ratio<i64>,
    pub mode: Mode,
    pub letterbox_in_breaks: bool,
    pub story_fire_in_front: bool, // Deprecated
    pub use_skin_sprites: bool,
    pub always_show_play_field: bool, // Deprecated
    pub overlay_position: OverlayPosition,
    pub skin_preference: Option<String>,
    pub epilepsy_warning: bool,
    pub countdown_offset: i64,
    pub special_style: bool,
    pub widescreen_storyboard: bool,
    pub samples_match_playback_rate: bool,

    //[Editor]
    pub bookmarks: Option<Vec<i64>>,
    pub distance_spacing: Option<Ratio<i64>>,
    pub beat_divisor: Option<i64>,
    pub grid_size: Option<i64>,
    pub timeline_zoom: Option<Ratio<i64>>,

    //[Metadata]
    pub title: Option<String>,
    pub title_unicode: Option<String>,
    pub artist: Option<String>,
    pub artist_unicode: Option<String>,
    pub creator: Option<String>,
    pub version: Option<String>,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    pub beatmap_id: Option<i64>,
    pub beatmap_set_id: Option<i64>,

    //[Difficulty]
    pub hpdrain_rate: Option<Ratio<i64>>,
    pub circle_size: Option<Ratio<i64>>,
    pub overall_difficulty: Option<Ratio<i64>>,
    pub approach_rate: Option<Ratio<i64>>,
    pub slider_multiplier: Option<Ratio<i64>>,
    pub slider_tick_rate: Option<Ratio<i64>>,

    //[Events]
    // I will be omitting the story board events because they are complicated.
    pub background: Option<Background>,
    pub breaks: Option<Vec<Break>>,

    //[TimingPoints]
    pub timing_points: Option<Vec<TimingPoint>>,

    //[Colo*rs]
    pub colors: Option<Vec<Color>>,
    //[HitObjects]
    pub hit_objects: Option<Vec<HitObject>>,
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
pub enum Countdown {
    None,
    Normal,
    Half,
    Double,
}
impl FromStr for Countdown {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Countdown::None),
            "1" => Ok(Countdown::Normal),
            "2" => Ok(Countdown::Half),
            "3" => Ok(Countdown::Double),
            _ => Err("Invalid Countdown".into()),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    Osu,
    Taiko,
    Catch,
    Mania,
}
impl FromStr for Mode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Osu),
            "1" => Ok(Self::Taiko),
            "2" => Ok(Self::Catch),
            "3" => Ok(Self::Mania),
            _ => Err("Invalid Mode".into()),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OverlayPosition {
    NoChange,
    Below,
    Above,
}
impl FromStr for OverlayPosition {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NoChange" => Ok(Self::NoChange),
            "Below" => Ok(Self::Below),
            "Above" => Ok(Self::Above),
            _ => Err("Invalid OverlayPosition".into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Background {
    filename: String,
    xoffset: i64,
    yoffset: i64,
}
impl FromStr for Background {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Background event
        let mut line = s.split(',').skip(2);
        let mut background = line
            .next()
            .expect("at background assignment in Background parsing")
            .chars();
        background.next();
        background.next_back();
        // Remove quotes
        let background = background.as_str().to_string();
        let x = line
            .next()
            .unwrap_or("0")
            .parse::<i64>()
            .expect("at x assignment in Background parsing");
        let y = line
            .next()
            .unwrap_or("0")
            .parse::<i64>()
            .expect("at y assignment in Background parsing");
        Ok(Self {
            filename: background,
            xoffset: x,
            yoffset: y,
        })
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Break {
    start_time: i64,
    end_time: i64,
}
impl FromStr for Break {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',').skip(1);
        let start_time = line
            .next()
            .expect("at start_time assignment in Break parsing")
            .parse::<i64>()
            .expect("at i64 parsing of start_time in Break parsing");
        let end_time = line
            .next()
            .expect("at end_time assignment in Break parsing")
            .parse::<i64>()
            .expect("at i64 parsing of end_time in Break parsing");
        Ok(Self {
            start_time,
            end_time,
        })
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TimingPoint {
    time: i64,
    beat_length: f64,
    meter: i64,
    sample_set: SampleSet,
    sample_index: i64,
    volume: i64,
    uninherited: bool,
    effects: Effects,
}
impl Default for TimingPoint {
    fn default() -> Self {
        Self {
            time: 0,
            beat_length: 0.0,
            meter: 4,
            sample_set: SampleSet::Default,
            sample_index: 0,
            volume: 100,
            uninherited: true,
            effects: Effects {
                kiai: false,
                ommit_barline: false,
            },
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
                let meter = line
                    .next()
                    .expect("at meter assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of meter in TimingPoint parsing, 7 branch");
                let sample_set = line
                    .next()
                    .expect("at sample_set assignment in TimingPoint parsing, 7 branch")
                    .parse::<SampleSet>()
                    .expect("at SampleSet parsing of sample_set in TimingPoint parsing, 7 branch");
                let sample_index = line
                    .next()
                    .expect("at sample_index assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of sample_index in TimingPoint parsing, 7 branch");
                //sample index
                let volume = line
                    .next()
                    .expect("at volume assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of volume in TimingPoint parsing, 7 branch");
                Ok(Self {
                    time,
                    beat_length,
                    meter,
                    sample_set,
                    sample_index,
                    volume,
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
                let meter = line
                    .next()
                    .expect("at meter assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of meter in TimingPoint parsing, 7 branch");
                let sample_set = line
                    .next()
                    .expect("at sample_set assignment in TimingPoint parsing, 7 branch")
                    .parse::<SampleSet>()
                    .expect("at SampleSet parsing of sample_set in TimingPoint parsing, 7 branch");
                let sample_index = line
                    .next()
                    .expect("at sample_index assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of sample_index in TimingPoint parsing, 7 branch");
                //sample index
                let volume = line
                    .next()
                    .expect("at volume assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of volume in TimingPoint parsing, 7 branch");
                let uninherited = line
                    .next()
                    .expect("at uninherited assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of uninherited in TimingPoint parsing, 7 branch")
                    == 1;
                Ok(Self {
                    time,
                    beat_length,
                    meter,
                    sample_set,
                    sample_index,
                    volume,
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
                let meter = line
                    .next()
                    .expect("at meter assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of meter in TimingPoint parsing, 7 branch");
                let sample_set = line
                    .next()
                    .expect("at sample_set assignment in TimingPoint parsing, 7 branch")
                    .parse::<SampleSet>()
                    .expect("at SampleSet parsing of sample_set in TimingPoint parsing, 7 branch");
                let sample_index = line
                    .next()
                    .expect("at sample_index assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of sample_index in TimingPoint parsing, 7 branch");
                //sample index
                let volume = line
                    .next()
                    .expect("at volume assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of volume in TimingPoint parsing, 7 branch");
                let uninherited = line
                    .next()
                    .expect("at uninherited assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of uninherited in TimingPoint parsing, 7 branch")
                    == 1;
                let effects = line
                    .next()
                    .expect("at effects assignment in TimingPoint parsing, 7 branch")
                    .parse::<Effects>()
                    .expect("at Effects parsing of effects in TimingPoint parsing, 7 branch");
                Ok(Self {
                    time,
                    beat_length,
                    meter,
                    sample_set,
                    sample_index,
                    volume,
                    uninherited,
                    effects,
                })
            }
            _ => return Result::Err(format!("Invalid timing point: {}", s)),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Effects {
    kiai: bool, // 1 on
    // 2 is unused
    ommit_barline: bool, // 4 on
}
impl FromStr for Effects {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kiai, ommit_barline) = match s {
            "0" => (false, false),
            "1" => (true, false),
            "4" => (false, true),
            "5" => (true, true),
            _ => return Result::Err(format!("Invalid effect")),
        };
        Ok(Self {
            kiai,
            ommit_barline,
        })
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s
            .split(" : ")
            .skip(1)
            .next()
            .expect("at line assignment in Color parsing")
            .split(',');
        let red = line
            .next()
            .expect("in red assignment in Color parsing")
            .parse::<u8>()
            .expect("at u8 parsing of red in Color parsing");
        let green = line
            .next()
            .expect("in green assignment in Color parsing")
            .parse::<u8>()
            .expect("at u8 parsing of green in Color parsing");
        let blue = line
            .next()
            .expect("in blue assignment in Color parsing")
            .parse::<u8>()
            .expect("at u8 parsing of blue in Color parsing");
        Ok(Self { red, green, blue })
    }
}
impl Color {
    pub fn tuple(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
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
    flags: Type,
    hit_sound: HitSound,
    hit_sample: HitSample,
}
impl FromStr for Circle {
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
    flags: Type,
    hit_sound: HitSound,
    curve: Curve,
    slides: i64,
    length: f64,
    edge_sounds: Vec<HitSound>,
    edge_sets: Vec<HalfHitSample>,
    hit_sample: HitSample,
}
impl FromStr for Slider {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        let x = line
            .next()
            .expect("in x assignment in Slider parsing")
            .parse::<i64>()
            .expect("in i64 parsing in x assignment in Slider parsing");
        let y = line
            .next()
            .expect("in y assignment in Slider parsing")
            .parse::<i64>()
            .expect("in i64 parsing in y assignment in Slider parsing");
        let time = line
            .next()
            .expect("in time assignment in Slider parsing")
            .parse::<i64>()
            .expect("in i64 parsing in time assignment in Slider parsing");
        let flags = line
            .next()
            .expect("in flags assignment in Slider parsing")
            .parse::<Type>()
            .expect("in Type parsing in flags assignment in Slider parsing");
        let hit_sound = line
            .next()
            .expect("in hit_sound assignment in Slider parsing")
            .parse::<HitSound>()
            .expect("in HitSound parsing in hit_sound assignment in Slider parsing");
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
        let edge_sounds = match commas {
            4 | 6 => {
                let mut sounds = Vec::new();
                for sound in line
                    .next()
                    .expect("in sound assignment in edge_sounds assignment in Slider parsing")
                    .split('|')
                {
                    sounds.push(
                        sound
                            .parse::<HitSound>()
                            .expect("in sounds pushing with HitSound parsing in edge_sounds assignment in Slider parsing"),
                    );
                }
                sounds
            }
            _ => vec![
                "0".parse::<HitSound>().expect(
                    "at edge_sounds assignment with HitSound parsing of \"0\" in Slider parsing",
                ),
                "2".parse::<HitSound>().expect(
                    "at edge_sounds assignment with HitSound parsing of \"2\" in Slider parsing",
                ),
            ],
        };
        let edge_sets = match commas {
            6 => {
                let mut sounds = Vec::new();
                for sound in line
                    .next()
                    .expect("in sound assignment in edge_sets assignment in Slider parsing")
                    .split('|')
                {
                    sounds.push(
                        sound
                            .parse::<HalfHitSample>()
                            .expect("in sounds pushing with HalfHitSample parsing in edge_sets assignment in Slider parsing"),
                    );
                }
                sounds
            }
            _ => vec![
                "0:0".parse::<HalfHitSample>().expect(
                    "at edge_sets assignment with HalfHitSample parsing of \"0:0\" in Slider parsing",
                ),
                "0:0".parse::<HalfHitSample>().expect(
                    "at edge_sets assignment with HalfHitSample parsing of \"0:0\" in Slider parsing",
                ),
            ],
        };
        let hit_sample = match commas {
            6 => line
                .next()
                .expect("in hit_sample assignment in Slider parsing")
                .parse::<HitSample>()
                .expect("in HitSample parsing in hit_sample assignment in Slider parsing"),
            _ => HitSample {
                ..Default::default()
            },
        };
        Ok(Slider {
            x,
            y,
            time,
            flags,
            hit_sound,
            curve,
            slides,
            length,
            edge_sounds,
            edge_sets,
            hit_sample,
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
    flags: Type,
    hit_sound: HitSound,
    end_time: i64,
    hit_sample: HitSample,
}
impl FromStr for Spinner {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        let hit_sample = HitSample {
            ..Default::default()
        };
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
        let flags = line
            .next()
            .expect("in flags assignment in Spinner parsing")
            .parse::<Type>()
            .expect("in Type parsing in flags assignment in Spinner parsing");
        let hit_sound = line
            .next()
            .expect("in hit_sound assignment in Spinner parsing")
            .parse::<HitSound>()
            .expect("in HitSound parsing in hit_sound assignment in Spinner parsing");
        let end_time = line
            .next()
            .expect("in end_time assignment in Spinner parsing")
            .parse::<i64>()
            .expect("in i64 parsing in end_time assignment in Spinner parsing");
        Ok(Spinner {
            x,
            y,
            time,
            flags,
            hit_sound,
            end_time,
            hit_sample,
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
    new_combo: bool,         // 2
    color_skip: u8,          // 4-6 -- Actually a 3 bit uint
                             // 7 Mania hold
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
        let mut color_skip = 0;
        if bits[4] {
            color_skip += 2_u8.pow(2);
        }
        if bits[5] {
            color_skip += 2_u8.pow(1);
        }
        if bits[6] {
            color_skip += 2_u8.pow(0);
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
        Ok(Self {
            object_type,
            new_combo: bits[2],
            color_skip,
        })
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ObjectType {
    Circle,
    Slider,
    Spinner,
}
#[derive(Debug, Clone, PartialEq)]
pub struct HitSound {
    normal: bool,
    whistle: bool,
    finish: bool,
    clap: bool,
}
impl FromStr for HitSound {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num: i64 = s
            .parse()
            .expect("at num assignment and i64 parsing in HitSound parsing");
        let mut bits = [false; 8];
        if num > 2_i64.pow(4) - 1 {
            return Result::Err(format!("Invalid HitSound: {}", s));
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
            return Result::Err(format!("Logic error in HitSound creation"));
        }
        Ok(Self {
            normal: bits[0],
            whistle: bits[1],
            finish: bits[2],
            clap: bits[3],
        })
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HalfHitSample {
    normal_set: SampleSet,
    addition_set: SampleSet,
}
impl Default for HalfHitSample {
    fn default() -> Self {
        Self {
            normal_set: SampleSet::Default,
            addition_set: SampleSet::Default,
        }
    }
}
impl FromStr for HalfHitSample {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !(s.matches(':').count() == 1) {
            return Result::Err(format!("Invalid HalfHitSample: {}", s));
        }
        let mut values = s.split(':');
        let normal_set = values
            .next()
            .expect("at normal_set assignment in HalfHitSample parsing")
            .parse::<SampleSet>()
            .expect("at SampleSet parsing in normal_set assignment in HalfHitSample parsing");
        let addition_set = values
            .next()
            .expect("at addition_set assignment in HalfHitSample parsing")
            .parse::<SampleSet>()
            .expect("at SampleSet parsing in addition_set assignment in HalfHitSample parsing");
        Ok(Self {
            normal_set,
            addition_set,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct HitSample {
    normal_set: SampleSet,
    addition_set: SampleSet,
    index: i64,
    volume: i64, // From 0 to 100.
    filename: Option<String>,
}
impl Default for HitSample {
    fn default() -> Self {
        Self {
            normal_set: SampleSet::Default,
            addition_set: SampleSet::Default,
            index: 0,
            volume: 0,
            filename: None,
        }
    }
}
impl FromStr for HitSample {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut volume = 0;
        let mut filename = None;

        let mut values = match s.split_once(":") {
            Some(value) => value,
            None => {
                return Result::Err(format!(
                    "at values assignment before normal_set in HitSample parsing: {}",
                    s
                ))
            }
        };
        let normal_set = values
            .0
            .parse::<SampleSet>()
            .expect("at normal_set assignment with SampleSet parsing in HitSample parsing");
        values = match values.1.split_once(":") {
            Some(value) => value,
            None => {
                return Result::Err(format!(
                    "at values assignment before addition_set in HitSample parsing: {}",
                    s
                ))
            }
        };
        let addition_set = values
            .0
            .parse::<SampleSet>()
            .expect("at addition_set assignment with SampleSet parsing in HitSample parsing");
        values = match values.1.split_once(":") {
            Some(value) => value,
            None => {
                return Result::Err(format!(
                    "at values assignment before index in HitSample parsing: {}",
                    s
                ))
            }
        };
        let index = values
            .0
            .parse::<i64>()
            .expect("at index assignment with i64 parsing in HitSample parsing");
        if s.matches(':').count() > 3 {
            values = match values.1.split_once(":") {
                Some(value) => value,
                None => {
                    return Result::Err(format!(
                        "at values assignment before volume in HitSample parsing: {}",
                        s
                    ))
                }
            };
            volume = values
                .0
                .parse::<i64>()
                .expect("at volume assignment with i64 parsing in HitSample parsing");
        }
        if (s.matches(':').count() > 3) & !values.1.trim().is_empty() {
            filename = Some(values.1.to_string());
        }
        Ok(Self {
            normal_set,
            addition_set,
            index,
            volume,
            filename,
        })
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SampleSet {
    Default,
    Normal,
    Soft,
    Drum,
}
impl FromStr for SampleSet {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | "Default" => Ok(SampleSet::Default),
            "1" | "Normal" => Ok(SampleSet::Normal),
            "2" | "Soft" => Ok(SampleSet::Soft),
            "3" | "Drum" => Ok(SampleSet::Drum),
            _ => return Result::Err(format!("Invalid str during SampleSet parsing: {}", s)),
        }
    }
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
        let mut count = 0;
        for pair in line {
            match (_type, count) {
                (CurveType::Perfect, 2) => {
                    return Result::Err(format!(
                        "Invalid Curve: Perfect curve {} has more than 2 points",
                        s
                    ))
                }
                _ => count += 1,
            }
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
                flags: Type {
                    object_type: ObjectType::Slider,
                    new_combo: true,
                    color_skip: 0,
                },
                hit_sound: HitSound {
                    normal: false,
                    whistle: false,
                    finish: false,
                    clap: false
                },
                curve: Curve {
                    _type: CurveType::Linear,
                    points: vec![Point { x: 253, y: 60 }],
                },
                slides: 1,
                length: 105.493329791992,
                edge_sounds: vec![
                    HitSound {
                        normal: false,
                        whistle: true,
                        finish: false,
                        clap: false
                    },
                    HitSound {
                        normal: false,
                        whistle: false,
                        finish: false,
                        clap: false
                    },
                ],
                edge_sets: vec![
                    HalfHitSample {
                        normal_set: SampleSet::Default,
                        addition_set: SampleSet::Soft
                    },
                    HalfHitSample {
                        normal_set: SampleSet::Default,
                        addition_set: SampleSet::Soft
                    },
                ],
                hit_sample: HitSample {
                    ..Default::default()
                },
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
                flags: Type {
                    object_type: ObjectType::Slider,
                    new_combo: false,
                    color_skip: 0,
                },
                hit_sound: HitSound {
                    normal: false,
                    whistle: false,
                    finish: false,
                    clap: false
                },
                curve: Curve {
                    _type: CurveType::Perfect,
                    points: vec![Point { x: 282, y: 209 }, Point { x: 239, y: 210 }],
                },
                slides: 1,
                length: 105.493329791992,
                edge_sounds: vec![
                    HitSound {
                        normal: false,
                        whistle: true,
                        finish: false,
                        clap: false
                    },
                    HitSound {
                        normal: false,
                        whistle: false,
                        finish: false,
                        clap: false
                    },
                ],
                edge_sets: vec![
                    HalfHitSample {
                        normal_set: SampleSet::Default,
                        addition_set: SampleSet::Soft
                    },
                    HalfHitSample {
                        normal_set: SampleSet::Default,
                        addition_set: SampleSet::Soft
                    },
                ],
                hit_sample: HitSample {
                    ..Default::default()
                },
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
                flags: Type {
                    object_type: ObjectType::Slider,
                    new_combo: false,
                    color_skip: 0,
                },
                hit_sound: HitSound {
                    normal: false,
                    whistle: false,
                    finish: false,
                    clap: false
                },
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
                edge_sounds: vec![
                    HitSound {
                        normal: false,
                        whistle: true,
                        finish: false,
                        clap: false
                    },
                    HitSound {
                        normal: false,
                        whistle: false,
                        finish: false,
                        clap: false
                    },
                ],
                edge_sets: vec![
                    HalfHitSample {
                        normal_set: SampleSet::Default,
                        addition_set: SampleSet::Soft
                    },
                    HalfHitSample {
                        normal_set: SampleSet::Drum,
                        addition_set: SampleSet::Soft
                    },
                ],
                hit_sample: HitSample {
                    ..Default::default()
                },
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
                flags: Type {
                    object_type: ObjectType::Slider,
                    new_combo: false,
                    color_skip: 0,
                },
                hit_sound: HitSound {
                    normal: false,
                    whistle: false,
                    finish: true,
                    clap: false
                },
                curve: Curve {
                    _type: CurveType::Linear,
                    points: vec![Point { x: 160, y: 320 }, Point { x: 160, y: 240 }],
                },
                slides: 1,
                length: 70.0,
                edge_sounds: vec![
                    HitSound {
                        normal: false,
                        whistle: false,
                        finish: false,
                        clap: false
                    },
                    HitSound {
                        normal: false,
                        whistle: true,
                        finish: false,
                        clap: false
                    },
                ],
                edge_sets: vec![
                    HalfHitSample {
                        normal_set: SampleSet::Default,
                        addition_set: SampleSet::Default
                    },
                    HalfHitSample {
                        normal_set: SampleSet::Default,
                        addition_set: SampleSet::Default
                    },
                ],
                hit_sample: HitSample {
                    ..Default::default()
                },
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
                flags: Type {
                    object_type: ObjectType::Slider,
                    new_combo: false,
                    color_skip: 0,
                },
                hit_sound: HitSound {
                    normal: false,
                    whistle: false,
                    finish: true,
                    clap: false
                },
                curve: Curve {
                    _type: CurveType::Bezier,
                    points: vec![Point { x: 192, y: 256 }, Point { x: 288, y: 256 }],
                },
                slides: 3,
                length: 70.0,
                edge_sounds: vec![
                    HitSound {
                        normal: false,
                        whistle: false,
                        finish: false,
                        clap: false
                    },
                    HitSound {
                        normal: false,
                        whistle: true,
                        finish: false,
                        clap: false
                    },
                ],
                edge_sets: vec![
                    HalfHitSample {
                        normal_set: SampleSet::Default,
                        addition_set: SampleSet::Default
                    },
                    HalfHitSample {
                        normal_set: SampleSet::Default,
                        addition_set: SampleSet::Default
                    },
                ],
                hit_sample: HitSample {
                    ..Default::default()
                },
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
                flags: Type {
                    object_type: ObjectType::Slider,
                    new_combo: true,
                    color_skip: 0,
                },
                hit_sound: HitSound {
                    normal: false,
                    whistle: false,
                    finish: false,
                    clap: false
                },
                curve: Curve {
                    _type: CurveType::Centripetal,
                    points: vec![Point { x: 244, y: 69 }, Point { x: 352, y: 64 }],
                },
                slides: 2,
                length: 160.0,
                edge_sounds: vec![
                    HitSound {
                        normal: false,
                        whistle: false,
                        finish: true,
                        clap: false
                    },
                    HitSound {
                        normal: false,
                        whistle: false,
                        finish: false,
                        clap: false
                    },
                    HitSound {
                        normal: false,
                        whistle: false,
                        finish: false,
                        clap: false
                    },
                ],
                edge_sets: vec![
                    HalfHitSample {
                        normal_set: SampleSet::Default,
                        addition_set: SampleSet::Default
                    },
                    HalfHitSample {
                        normal_set: SampleSet::Default,
                        addition_set: SampleSet::Default
                    },
                ],
                hit_sample: HitSample {
                    ..Default::default()
                },
            },
        );
        // Circle from ver 14.
        assert!(catch_unwind_silent(|| "102,240,161,5,2,0:0:0:0:".parse::<Slider>()).is_err());
        // Circle from ver 3.
        assert!(catch_unwind_silent(|| "96,64,8118,5,4,".parse::<Slider>()).is_err());
        // Spinner from ver 14.
        assert!(
            catch_unwind_silent(|| "256,192,29544,12,0,32632,0:2:0:0:".parse::<Slider>()).is_err()
        );
        // Spinner from ver 3.
        assert!(catch_unwind_silent(|| "256,192,141619,12,0,143869".parse::<Slider>()).is_err());
        // Ensure that trim is not used during parsing.
        assert!(catch_unwind_silent(|| {
            " 137,72,2985,6,0,L|253:60,1,105.493329791992,2|0,0:2|0:2,0:0:0:0:".parse::<Slider>()
        })
        .is_err());
        // Trim actually needs to be used in the HitSample parsing so a space on the end must be OK.
        assert_eq!(
            "137,72,2985,6,0,L|253:60,1,105.493329791992,2|0,0:2|0:2,0:0:0:0: "
                .parse::<Slider>()
                .unwrap(),
            Slider {
                x: 137,
                y: 72,
                time: 2985,
                flags: Type {
                    object_type: ObjectType::Slider,
                    new_combo: true,
                    color_skip: 0,
                },
                hit_sound: HitSound {
                    normal: false,
                    whistle: false,
                    finish: false,
                    clap: false
                },
                curve: Curve {
                    _type: CurveType::Linear,
                    points: vec![Point { x: 253, y: 60 }],
                },
                slides: 1,
                length: 105.493329791992,
                edge_sounds: vec![
                    HitSound {
                        normal: false,
                        whistle: true,
                        finish: false,
                        clap: false
                    },
                    HitSound {
                        normal: false,
                        whistle: false,
                        finish: false,
                        clap: false
                    },
                ],
                edge_sets: vec![
                    HalfHitSample {
                        normal_set: SampleSet::Default,
                        addition_set: SampleSet::Soft
                    },
                    HalfHitSample {
                        normal_set: SampleSet::Default,
                        addition_set: SampleSet::Soft
                    },
                ],
                hit_sample: HitSample {
                    ..Default::default()
                },
            },
        );
        // Ensure panic on empty input.
        assert!(catch_unwind_silent(|| "".parse::<Slider>()).is_err());
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
                flags: Type {
                    object_type: ObjectType::Spinner,
                    new_combo: true,
                    color_skip: 0,
                },
                hit_sound: HitSound {
                    normal: false,
                    whistle: false,
                    finish: false,
                    clap: false,
                },
                end_time: 32632,
                hit_sample: HitSample {
                    ..Default::default()
                },
            },
        );
        // Spinner from ver 3.
        assert_eq!(
            "256,192,141619,12,0,143869".parse::<Spinner>().unwrap(),
            Spinner {
                x: 256,
                y: 192,
                time: 141619,
                flags: Type {
                    object_type: ObjectType::Spinner,
                    new_combo: true,
                    color_skip: 0,
                },
                hit_sound: HitSound {
                    normal: false,
                    whistle: false,
                    finish: false,
                    clap: false,
                },
                end_time: 143869,
                hit_sample: HitSample {
                    ..Default::default()
                },
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
                    new_combo: false,
                    color_skip: 0,
                },
            ),
            (
                "2",
                Type {
                    object_type: ObjectType::Slider,
                    new_combo: false,
                    color_skip: 0,
                },
            ),
            (
                "5",
                Type {
                    object_type: ObjectType::Circle,
                    new_combo: true,
                    color_skip: 0,
                },
            ),
            (
                "6",
                Type {
                    object_type: ObjectType::Slider,
                    new_combo: true,
                    color_skip: 0,
                },
            ),
            (
                "8",
                Type {
                    object_type: ObjectType::Spinner,
                    new_combo: false,
                    color_skip: 0,
                },
            ),
            (
                "12",
                Type {
                    object_type: ObjectType::Spinner,
                    new_combo: true,
                    color_skip: 0,
                },
            ),
            (
                "66",
                Type {
                    object_type: ObjectType::Slider,
                    new_combo: false,
                    color_skip: 1,
                },
            ),
            (
                "33",
                Type {
                    object_type: ObjectType::Circle,
                    new_combo: false,
                    color_skip: 2,
                },
            ),
            (
                "104",
                Type {
                    object_type: ObjectType::Spinner,
                    new_combo: false,
                    color_skip: 3,
                },
            ),
            (
                "17",
                Type {
                    object_type: ObjectType::Circle,
                    new_combo: false,
                    color_skip: 4,
                },
            ),
            (
                "86",
                Type {
                    object_type: ObjectType::Slider,
                    new_combo: true,
                    color_skip: 5,
                },
            ),
            (
                "53",
                Type {
                    object_type: ObjectType::Circle,
                    new_combo: true,
                    color_skip: 6,
                },
            ),
            (
                "124",
                Type {
                    object_type: ObjectType::Spinner,
                    new_combo: true,
                    color_skip: 7,
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
    fn hit_sound_parse() {
        let correct = [
            (
                "0",
                HitSound {
                    normal: false,
                    whistle: false,
                    finish: false,
                    clap: false,
                },
            ),
            (
                "1",
                HitSound {
                    normal: true,
                    whistle: false,
                    finish: false,
                    clap: false,
                },
            ),
            (
                "2",
                HitSound {
                    normal: false,
                    whistle: true,
                    finish: false,
                    clap: false,
                },
            ),
            (
                "3",
                HitSound {
                    normal: true,
                    whistle: true,
                    finish: false,
                    clap: false,
                },
            ),
            (
                "4",
                HitSound {
                    normal: false,
                    whistle: false,
                    finish: true,
                    clap: false,
                },
            ),
            (
                "5",
                HitSound {
                    normal: true,
                    whistle: false,
                    finish: true,
                    clap: false,
                },
            ),
            (
                "6",
                HitSound {
                    normal: false,
                    whistle: true,
                    finish: true,
                    clap: false,
                },
            ),
            (
                "7",
                HitSound {
                    normal: true,
                    whistle: true,
                    finish: true,
                    clap: false,
                },
            ),
            (
                "8",
                HitSound {
                    normal: false,
                    whistle: false,
                    finish: false,
                    clap: true,
                },
            ),
            (
                "9",
                HitSound {
                    normal: true,
                    whistle: false,
                    finish: false,
                    clap: true,
                },
            ),
            (
                "10",
                HitSound {
                    normal: false,
                    whistle: true,
                    finish: false,
                    clap: true,
                },
            ),
            (
                "11",
                HitSound {
                    normal: true,
                    whistle: true,
                    finish: false,
                    clap: true,
                },
            ),
            (
                "12",
                HitSound {
                    normal: false,
                    whistle: false,
                    finish: true,
                    clap: true,
                },
            ),
            (
                "13",
                HitSound {
                    normal: true,
                    whistle: false,
                    finish: true,
                    clap: true,
                },
            ),
            (
                "14",
                HitSound {
                    normal: false,
                    whistle: true,
                    finish: true,
                    clap: true,
                },
            ),
            (
                "15",
                HitSound {
                    normal: true,
                    whistle: true,
                    finish: true,
                    clap: true,
                },
            ),
        ];
        for pair in correct {
            assert_eq!(pair.0.parse::<HitSound>().unwrap(), pair.1);
        }
        // Ensure that trim is not used.
        let no_trim = [
            " 0", " 1", " 2", " 3", " 4", " 5", " 6", " 7", " 8", " 9", " 10", " 11", " 12", " 13",
            " 14", " 15", "0 ", "1 ", "2 ", "3 ", "4 ", "5 ", "6 ", "7 ", "8 ", "9 ", "10 ", "11 ",
            "12 ", "13 ", "14 ", "15 ",
        ];
        for line in no_trim {
            assert!(catch_unwind_silent(|| line.parse::<HitSound>()).is_err());
        }
        // Ensure panic on empty input.
        assert!(catch_unwind_silent(|| "".parse::<HitSound>()).is_err());
    }
    #[test]
    fn half_hit_sample_parse() {
        assert_eq!(
            "0:0".parse::<HalfHitSample>().unwrap(),
            HalfHitSample {
                ..Default::default()
            }
        );
        assert_eq!(
            "0:1".parse::<HalfHitSample>().unwrap(),
            HalfHitSample {
                normal_set: SampleSet::Default,
                addition_set: SampleSet::Normal,
            }
        );
        assert_eq!(
            "2:3".parse::<HalfHitSample>().unwrap(),
            HalfHitSample {
                normal_set: SampleSet::Soft,
                addition_set: SampleSet::Drum,
            }
        );
        // Ensure trim is not used.
        assert!(catch_unwind_silent(|| " 0:0".parse::<HalfHitSample>()).is_err());
        assert!(catch_unwind_silent(|| "0:0 ".parse::<HalfHitSample>()).is_err());
        // Ensure panic on empty input.
        assert!(catch_unwind_silent(|| "".parse::<HalfHitSample>()).is_err());
    }
    #[test]
    fn hit_sample_parse() {
        // Test default case is equal to 0:0:0:0:
        assert_eq!(
            "0:0:0:0:".parse::<HitSample>().unwrap(),
            HitSample {
                ..Default::default()
            },
        );
        // Test without filename.
        assert_eq!(
            "1:2:3:100:".parse::<HitSample>().unwrap(),
            HitSample {
                normal_set: "1".parse::<SampleSet>().unwrap(),
                addition_set: "2".parse::<SampleSet>().unwrap(),
                index: 3,
                volume: 100,
                ..Default::default()
            },
        );
        // Test with filename.
        assert_eq!(
            "0:3:2:100:file".parse::<HitSample>().unwrap(),
            HitSample {
                normal_set: "0".parse::<SampleSet>().unwrap(),
                addition_set: "3".parse::<SampleSet>().unwrap(),
                index: 2,
                volume: 100,
                filename: Some("file".into()),
            },
        );
        // Test with filename with three spaces.
        assert_eq!(
            "0:3:2:100: a file ".parse::<HitSample>().unwrap(),
            HitSample {
                normal_set: "0".parse::<SampleSet>().unwrap(),
                addition_set: "3".parse::<SampleSet>().unwrap(),
                index: 2,
                volume: 100,
                filename: Some(" a file ".into()),
            },
        );
        // Ensure that trim isn't used on the entire string.
        assert!(catch_unwind_silent(|| " 0:0:0:0:".parse::<SampleSet>()).is_err());
        // Ensure panic on empty input.
        assert!(catch_unwind_silent(|| "".parse::<SampleSet>()).is_err());
    }
    #[test]
    fn sample_set_parse() {
        // Test all correct inputs
        let correct = [
            ("0", SampleSet::Default),
            ("1", SampleSet::Normal),
            ("2", SampleSet::Soft),
            ("3", SampleSet::Drum),
        ];
        for pair in correct {
            assert_eq!(pair.0.parse::<SampleSet>().unwrap(), pair.1);
        }
        // Ensure that trim is not used
        let no_trim = [" 0", " 1", " 2", " 3", "0 ", "1 ", "2 ", "3 "];
        for line in no_trim {
            assert!(catch_unwind_silent(|| line.parse::<SampleSet>()).is_err());
        }
        // Ensure panic on empty input
        assert!(catch_unwind_silent(|| "".parse::<SampleSet>()).is_err());
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
        assert!(catch_unwind_silent(|| "L|0:0 ".parse::<Curve>()).is_err());
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
            if let Ok(_) = line.parse::<CurveType>() {
                panic!(
                    "curve_type_parse did not return error on bad input: {}",
                    line
                )
            }
        }
        // Ensure panic on empty input.
        // TODO: Convert all catch_unwind_silent to error catchers
        assert!(catch_unwind_silent(|| "".parse::<CurveType>()).is_err());
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
