use std::{error::Error, str::FromStr};

pub fn parse_map(path: &str) -> Result<MapData, Box<dyn Error>> {
    // Try to open file
    let mut file = match std::fs::File::open(path) {
        Err(why) => panic!("couldn't open: {}", why),
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
    let mut map = MapData {
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
        .parse::<i32>()
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
                    .parse::<i32>()
                    .expect("at map.audio_lead_in assignment in tabular")
            }
            "AudioHash" => map.audio_hash = Some(value.to_string()),
            "PreviewTime" => {
                map.preview_time = value
                    .parse::<i32>()
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
                map.stack_leniency = value
                    .parse::<f32>()
                    .expect("at map.stack_leniency assignment in tabular")
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
                    _ => panic!(
                        "at map.letterbox_in_breaks assignment in tabular: \"{}\"",
                        value
                    ),
                };
            }
            "StoryFireInFront" => {
                map.story_fire_in_front = match value {
                    "1" => true,
                    "0" => false,
                    _ => panic!(
                        "at map.story_fire_in_front assignment in tabular: \"{}\"",
                        value
                    ),
                };
            }
            "UseSkinSprites" => {
                map.use_skin_sprites = match value {
                    "1" => true,
                    "0" => false,
                    _ => panic!(
                        "at map.use_skin_sprites assignment in tabular: \"{}\"",
                        value
                    ),
                };
            }
            "AlwaysShowPlayField" => {
                map.always_show_play_field = match value {
                    "1" => true,
                    "0" => false,
                    _ => panic!(
                        "at map.always_show_playfield assignment in tabular: \"{}\"",
                        value
                    ),
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
                    _ => panic!(
                        "at map.epilepsy_warning assignment in tabular: \"{}\"",
                        value
                    ),
                };
            }
            "CountdownOffset" => {
                map.countdown_offset = value
                    .parse::<i32>()
                    .expect("at map.countdown_offset assignment in tabular")
            }
            "SpecialStyle" => {
                map.special_style = match value {
                    "1" => true,
                    "0" => false,
                    _ => panic!("at map.special_style assignment in tabular: \"{}\"", value),
                };
            }
            "WidescreenStoryboard" => {
                map.widescreen_storyboard = match value {
                    "1" => true,
                    "0" => false,
                    _ => panic!(
                        "at map.widescreen_storyboard assignment in tabular: \"{}\"",
                        value
                    ),
                };
            }
            "SamplesMatchPlaybackRate" => {
                map.samples_match_playback_rate = match value {
                    "1" => true,
                    "0" => false,
                    _ => panic!(
                        "at map.samples_match_playback_rate assignment in tabular: \"{}\"",
                        value
                    ),
                };
            }
            "Bookmarks" => {
                let mut bookmarks = Vec::new();
                for i in value.split(',') {
                    bookmarks.push(i.parse::<i32>().expect("at bookmarks pushing in tabular"));
                }
                map.bookmarks = Some(bookmarks);
            }
            "DistanceSpacing" => {
                map.distance_spacing = Some(
                    value
                        .parse::<f32>()
                        .expect("at map.distance_spacing assignment in tabular"),
                )
            }
            "BeatDivisor" => {
                map.beat_divisor = Some(
                    value
                        .parse::<i32>()
                        .expect("at map.beat_divisor assignment in tabular"),
                )
            }
            "GridSize" => {
                map.grid_size = Some(
                    value
                        .parse::<i32>()
                        .expect("at map.grid_size assignment in tabular"),
                )
            }
            "TimelineZoom" => {
                map.timeline_zoom = Some(
                    value
                        .parse::<f32>()
                        .expect("at map.timeline_zoom assignment in tabular"),
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
                        .parse::<i32>()
                        .expect("at map.beatmap_id assignment in tabular"),
                )
            }
            "BeatmapSetID" => {
                map.beatmap_set_id = Some(
                    value
                        .parse::<i32>()
                        .expect("at map.beatmap_set_id assignment in tabular"),
                )
            }
            "HPDrainRate" => {
                map.hpdrain_rate = Some(
                    value
                        .parse::<f32>()
                        .expect("at map.hpdrain_rate assignment in tabular"),
                )
            }
            "CircleSize" => {
                map.circle_size = Some(
                    value
                        .parse::<f32>()
                        .expect("at map.circle_size assignment in tabular"),
                )
            }
            "OverallDifficulty" => {
                map.overall_difficulty = Some(
                    value
                        .parse::<f32>()
                        .expect("at map.overall_difficulty assignment in tabular"),
                )
            }
            "ApproachRate" => {
                map.approach_rate = Some(
                    value
                        .parse::<f32>()
                        .expect("at map.approach_rate assignment in tabular"),
                )
            }
            "SliderMultiplier" => {
                map.slider_multiplier = Some(
                    value
                        .parse::<f32>()
                        .expect("at map.slider_multiplier assignment in tabular"),
                )
            }
            "SliderTickRate" => {
                map.slider_tick_rate = Some(
                    value
                        .parse::<f32>()
                        .expect("at map.slider_tick_rate assignment in tabular"),
                )
            }
            _ => panic!("Unknown key: {}", key),
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
        match line
            .split(',')
            .nth(3)
            .expect("at type indexing for hit object classification")
            .parse::<Type>()
            .expect("at type parsing for hit object classification")
            .object_type
        {
            ObjectType::Circle => {
                if map.circles.is_none() {
                    map.circles = Some(Vec::new());
                }
                map.circles
                    .as_mut()
                    .expect("at map.circles grabbing")
                    .push(line.parse::<Circle>().expect("at map.circles pushing"));
            }
            ObjectType::Slider => {
                if map.sliders.is_none() {
                    map.sliders = Some(Vec::new());
                }
                map.sliders
                    .as_mut()
                    .expect("at map.sliders grabbing")
                    .push(line.parse::<Slider>().expect("at map.sliders pushing"));
            }
            ObjectType::Spinner => {
                if map.spinners.is_none() {
                    map.spinners = Some(Vec::new());
                }
                map.spinners
                    .as_mut()
                    .expect("at map.spinners grabbing")
                    .push(line.parse::<Spinner>().expect("at map.spinners pushing"));
            }
        }
    }
    Ok(map)
}

/// Complete map data for a .osu file.
/// Arranged like map ver 14.
#[derive(Debug, Clone)]
pub struct MapData {
    pub file_format: i32,
    //[General]
    pub audio_filename: Option<String>,
    pub audio_lead_in: i32,
    pub audio_hash: Option<String>, // Deprecated
    pub preview_time: i32,
    pub countdown: Countdown,
    pub sample_set: SampleSet,
    pub stack_leniency: f32,
    pub mode: Mode,
    pub letterbox_in_breaks: bool,
    pub story_fire_in_front: bool, // Deprecated
    pub use_skin_sprites: bool,
    pub always_show_play_field: bool, // Deprecated
    pub overlay_position: OverlayPosition,
    pub skin_preference: Option<String>,
    pub epilepsy_warning: bool,
    pub countdown_offset: i32,
    pub special_style: bool,
    pub widescreen_storyboard: bool,
    pub samples_match_playback_rate: bool,

    //[Editor]
    pub bookmarks: Option<Vec<i32>>,
    pub distance_spacing: Option<f32>,
    pub beat_divisor: Option<i32>,
    pub grid_size: Option<i32>,
    pub timeline_zoom: Option<f32>,

    //[Metadata]
    pub title: Option<String>,
    pub title_unicode: Option<String>,
    pub artist: Option<String>,
    pub artist_unicode: Option<String>,
    pub creator: Option<String>,
    pub version: Option<String>,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    pub beatmap_id: Option<i32>,
    pub beatmap_set_id: Option<i32>,

    //[Difficulty]
    pub hpdrain_rate: Option<f32>,
    pub circle_size: Option<f32>,
    pub overall_difficulty: Option<f32>,
    pub approach_rate: Option<f32>,
    pub slider_multiplier: Option<f32>,
    pub slider_tick_rate: Option<f32>,

    //[Events]
    // I will be omitting the story board events because they are complicated.
    pub background: Option<Background>,
    pub breaks: Option<Vec<Break>>,

    //[TimingPoints]
    pub timing_points: Option<Vec<TimingPoint>>,

    //[Colo*rs]
    pub colors: Option<Vec<Color>>,
    //[HitObjects]
    pub circles: Option<Vec<Circle>>,
    pub sliders: Option<Vec<Slider>>,
    pub spinners: Option<Vec<Spinner>>,
}
impl Default for MapData {
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
            stack_leniency: 0.7,
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
            circles: None,
            sliders: None,
            spinners: None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SampleSet {
    Default,
    Normal,
    Soft,
    Drum,
}
impl FromStr for SampleSet {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | "Default" => Ok(SampleSet::Default),
            "1" | "Normal" => Ok(SampleSet::Normal),
            "2" | "Soft" => Ok(SampleSet::Soft),
            "3" | "Drum" => Ok(SampleSet::Drum),
            _ => panic!("Invalid str during SampleSet parsing: {}", s),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Countdown {
    None,
    Normal,
    Half,
    Double,
}
impl FromStr for Countdown {
    type Err = Box<dyn std::error::Error>;
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

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Osu,
    Taiko,
    Catch,
    Mania,
}
impl FromStr for Mode {
    type Err = Box<dyn std::error::Error>;
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

#[derive(Debug, Copy, Clone)]
pub enum OverlayPosition {
    NoChange,
    Below,
    Above,
}
impl FromStr for OverlayPosition {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NoChange" => Ok(Self::NoChange),
            "Below" => Ok(Self::Below),
            "Above" => Ok(Self::Above),
            _ => Err("Invalid OverlayPosition".into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Background {
    filename: String,
    xoffset: i32,
    yoffset: i32,
}
impl FromStr for Background {
    type Err = ();
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
            .parse::<i32>()
            .expect("at x assignment in Background parsing");
        let y = line
            .next()
            .unwrap_or("0")
            .parse::<i32>()
            .expect("at y assignment in Background parsing");
        Ok(Self {
            filename: background,
            xoffset: x,
            yoffset: y,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Video {
    start_time: i32,
    filename: String,
    xoffset: i32,
    yoffset: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Break {
    start_time: i32,
    end_time: i32,
}
impl FromStr for Break {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',').skip(1);
        let start_time = line
            .next()
            .expect("at start_time assignment in Break parsing")
            .parse::<i32>()
            .expect("at i32 parsing of start_time in Break parsing");
        let end_time = line
            .next()
            .expect("at end_time assignment in Break parsing")
            .parse::<i32>()
            .expect("at i32 parsing of end_time in Break parsing");
        Ok(Self {
            start_time,
            end_time,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Effects {
    kiai: bool, // 1 on
    // 2 is unused
    ommit_barline: bool, // 4 on
}
impl FromStr for Effects {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kiai, ommit_barline) = match s {
            "0" => (false, false),
            "1" => (true, false),
            "4" => (false, true),
            "5" => (true, true),
            _ => panic!("Invalid effect"),
        };
        Ok(Self {
            kiai,
            ommit_barline,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TimingPoint {
    time: i32,
    beat_length: f32,
    meter: i32,
    sample_set: SampleSet,
    sample_index: i32,
    volume: i32,
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
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.matches(',').count() {
            1 => {
                let mut line = s.split(',');
                let time = line
                    .next()
                    .expect("at time assignment in TimingPoint parsing, 1 branch")
                    .parse::<i32>()
                    .expect("at i32 parsing of time in TimingPoint parsing, 1 branch");
                let beat_length = line
                    .next()
                    .expect("at beat_length assignment in TimingPoint parsing, 1 branch")
                    .parse::<f32>()
                    .expect("at f32 parsing of beat_length in TimingPoint parsing, 1 branch");
                Ok(Self {
                    time,
                    beat_length,
                    ..Default::default()
                })
            }
            7 => {
                let mut line = s.split(',');
                let time = line
                    .next()
                    .expect("at time assignment in TimingPoint parsing, 7 branch")
                    .parse::<i32>()
                    .expect("at i32 parsing of time in TimingPoint parsing, 7 branch");
                let beat_length = line
                    .next()
                    .expect("at beat_length assignment in TimingPoint parsing, 7 branch")
                    .parse::<f32>()
                    .expect("at f32 parsing of beat_length in TimingPoint parsing, 7 branch");
                let meter = line
                    .next()
                    .expect("at meter assignment in TimingPoint parsing, 7 branch")
                    .parse::<i32>()
                    .expect("at i32 parsing of meter in TimingPoint parsing, 7 branch");
                let sample_set = line
                    .next()
                    .expect("at sample_set assignment in TimingPoint parsing, 7 branch")
                    .parse::<SampleSet>()
                    .expect("at SampleSet parsing of sample_set in TimingPoint parsing, 7 branch");
                let sample_index = line
                    .next()
                    .expect("at sample_index assignment in TimingPoint parsing, 7 branch")
                    .parse::<i32>()
                    .expect("at i32 parsing of sample_index in TimingPoint parsing, 7 branch");
                //sample index
                let volume = line
                    .next()
                    .expect("at volume assignment in TimingPoint parsing, 7 branch")
                    .parse::<i32>()
                    .expect("at i32 parsing of volume in TimingPoint parsing, 7 branch");
                let uninherited = line
                    .next()
                    .expect("at uninherited assignment in TimingPoint parsing, 7 branch")
                    .parse::<i32>()
                    .expect("at i32 parsing of uninherited in TimingPoint parsing, 7 branch")
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
            _ => panic!("Invalid timing point: {}", s),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
impl FromStr for Color {
    type Err = Box<dyn std::error::Error>;
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

#[derive(Debug, Copy, Clone)]
pub enum ObjectType {
    Circle,
    Slider,
    Spinner,
}

#[derive(Debug, Copy, Clone)]
pub struct Type {
    object_type: ObjectType, // 0 circle, 1 slider, 3 spinner
    new_combo: bool,         // 2
    color_skip: u8,          // 4-6 -- Actually a 3 bit uint
                             // 7 Mania hold
}
impl FromStr for Type {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num: i32 = s
            .parse()
            .expect("at num assignment and i32 parsing in Type parsing");
        let mut bits = [false; 8];
        if num > 2_i32.pow(8) - 1 {
            panic!("Invalid Type");
        }
        if num > 2_i32.pow(7) - 1 {
            bits[7] = true;
            num -= 2_i32.pow(7);
        }
        if num > 2_i32.pow(6) - 1 {
            bits[6] = true;
            num -= 2_i32.pow(6);
        }
        if num > 2_i32.pow(5) - 1 {
            bits[5] = true;
            num -= 2_i32.pow(5);
        }
        if num > 2_i32.pow(4) - 1 {
            bits[4] = true;
            num -= 2_i32.pow(4);
        }
        if num > 2_i32.pow(3) - 1 {
            bits[3] = true;
            num -= 2_i32.pow(3);
        }
        if num > 2_i32.pow(2) - 1 {
            bits[2] = true;
            num -= 2_i32.pow(2);
        }
        if num > 2_i32.pow(1) - 1 {
            bits[1] = true;
            num -= 2_i32.pow(1);
        }
        if num > 2_i32.pow(0) - 1 {
            bits[0] = true;
            num -= 2_i32.pow(0);
        }
        if num > 0 {
            panic!("Logic error in Type creation")
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
            _ => panic!("Invalid object type: {:?}", (bits[0], bits[1], bits[3])),
        };
        Ok(Self {
            object_type,
            new_combo: bits[2],
            color_skip,
        })
    }
}

#[derive(Debug, Clone)]
pub struct HitSound {
    normal: bool,
    whistle: bool,
    finish: bool,
    clap: bool,
}
impl FromStr for HitSound {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num: i32 = s
            .parse()
            .expect("at num assignment and i32 parsing in HitSound parsing");
        let mut bits = [false; 8];
        if num > 2_i32.pow(4) - 1 {
            panic!("Invalid HitSound: {}", s);
        }
        if num > 2_i32.pow(3) - 1 {
            bits[3] = true;
            num -= 2_i32.pow(3);
        }
        if num > 2_i32.pow(2) - 1 {
            bits[2] = true;
            num -= 2_i32.pow(2);
        }
        if num > 2_i32.pow(1) - 1 {
            bits[1] = true;
            num -= 2_i32.pow(1);
        }
        if num > 2_i32.pow(0) - 1 {
            bits[0] = true;
            num -= 2_i32.pow(0);
        }
        if num > 0 {
            panic!("Logic error in HitSound creation")
        }
        Ok(Self {
            normal: bits[0],
            whistle: bits[1],
            finish: bits[2],
            clap: bits[3],
        })
    }
}

#[derive(Debug, Copy, Clone)]
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
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !(s.matches(':').count() == 1) {
            panic!("Invalid HalfHitSample: {}", s)
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

#[derive(Debug, Clone)]
pub struct HitSample {
    normal_set: SampleSet,
    addition_set: SampleSet,
    index: i32,
    volume: i32, // From 0 to 100.
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
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_once(":")
            .expect("at values assignment before normal_set in HitSample parsing");
        let normal_set = values
            .0
            .parse::<SampleSet>()
            .expect("at normal_set assignment with SampleSet parsing in HitSample parsing");
        let values = values.1.split_once(":").unwrap_or_else(|| {
            panic!(
                "at values assignment before addition_set in HitSample parsing: err: {}",
                s
            )
        });
        let addition_set = values
            .0
            .parse::<SampleSet>()
            .expect("at addition_set assignment with SampleSet parsing in HitSample parsing");
        let values = values
            .1
            .split_once(":")
            .expect("at values assignment before index in HitSample parsing");
        let index = values
            .0
            .parse::<i32>()
            .expect("at index assignment with i32 parsing in HitSample parsing");
        let values = values
            .1
            .split_once(":")
            .expect("at values assignment before volume in HitSample parsing");
        let volume = values
            .0
            .parse::<i32>()
            .expect("at volume assignment with i32 parsing in HitSample parsing");
        let values = values
            .1
            .split_once(":")
            .expect("at values assignment before filename in HitSample parsing");
        let filename = if values.1.trim().is_empty() {
            None
        } else {
            Some(values.1.trim().to_string())
        };
        Ok(Self {
            normal_set,
            addition_set,
            index,
            volume,
            filename,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Circle {
    x: i32,
    y: i32,
    time: i32,
    flags: Type,
    hit_sound: HitSound,
    hit_sample: HitSample,
}
impl FromStr for Circle {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        let hit_sample = HitSample {
            ..Default::default()
        };
        // TODO: Parse HitSamples
        let x = line
            .next()
            .expect("at x assignment in Circle parsing")
            .parse::<i32>()
            .expect("at i32 parsing in x assignment in Circle parsing");
        let y = line
            .next()
            .expect("at y assignment in Circle parsing")
            .parse::<i32>()
            .expect("at i32 parsing in y assignment in Circle parsing");
        let time = line
            .next()
            .expect("at time assignment in Circle parsing")
            .parse::<i32>()
            .expect("at i32 parsing in time assignment in Circle parsing");
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

#[derive(Debug, Copy, Clone)]
pub enum CurveType {
    Bezier,
    Centripetal,
    Linear,
    Perfect,
}
impl FromStr for CurveType {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" => Ok(Self::Bezier),
            "C" => Ok(Self::Centripetal),
            "L" => Ok(Self::Linear),
            "P" => Ok(Self::Perfect),
            _ => panic!("Invalid CurveType: {}", s),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
pub struct Curve {
    _type: CurveType,
    points: Vec<Point>,
}
impl FromStr for Curve {
    type Err = Box<dyn std::error::Error>;
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
                x: pair
                    .next()
                    .expect("at x assignment in points pushing in Curve parsing")
                    .parse::<i32>()
                    .unwrap_or_else(|err| panic!(
                        "at i32 parsing in x assignment in points pushing in Curve parsing: error: {} with input: {}",
                        err, s
                    )),
                y: pair
                    .next()
                    .expect("at y assignment in points pushing in Curve parsing")
                    .parse::<i32>()
                    .unwrap_or_else(|err| panic!(
                        "at i32 parsing in y assignment in points pushing in Curve parsing: error: {} with input: {}",
                        err, s
                    )),
            });
        }
        Ok(Self { _type, points })
    }
}

#[derive(Debug, Clone)]
pub struct Slider {
    x: i32,
    y: i32,
    time: i32,
    flags: Type,
    hit_sound: HitSound,
    curve: Curve,
    slides: i32,
    length: f32,
    edge_sounds: Vec<HitSound>,
    edge_sets: Vec<HalfHitSample>,
    hit_sample: HitSample,
}
impl FromStr for Slider {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        let hit_sample = HitSample {
            ..Default::default()
        };
        let x = line
            .next()
            .expect("in x assignment in Slider parsing")
            .parse::<i32>()
            .expect("in i32 parsing in x assignment in Slider parsing");
        let y = line
            .next()
            .expect("in y assignment in Slider parsing")
            .parse::<i32>()
            .expect("in i32 parsing in y assignment in Slider parsing");
        let time = line
            .next()
            .expect("in time assignment in Slider parsing")
            .parse::<i32>()
            .expect("in i32 parsing in time assignment in Slider parsing");
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
            3 | 6 => line
                .next()
                .expect("in curve assignment in Slider parsing")
                .parse::<Curve>()
                .expect("in Curve parsing in curve assignment in Slider parsing"),
            _ => panic!(
                "Invalid slider: wrong remaining line size: {} in line: {} at curve assignment",
                commas, s
            ),
        };
        let slides = match commas {
            3 | 6 => line
                .next()
                .expect("in slides assignment in Slider parsing")
                .parse::<i32>()
                .expect("in i32 parsing in slides assignment in Slider parsing"),
            _ => panic!(
                "Invalid slider: wrong remaining line size: {} in line: {} at slides assignment",
                commas, s
            ),
        };
        let length = match commas {
            3 | 6 => line
                .next()
                .expect("in length assignment in Slider parsing")
                .parse::<f32>()
                .expect("in f32 parsing in length assignment in Slider parsing"),
            _ => panic!(
                "Invalid slider: wrong remaining line size: {} in line: {} at length assignment",
                commas, s
            ),
        };
        let edge_sounds = match commas {
            6 => {
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
            5 => {
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

#[derive(Debug, Clone)]
pub struct Spinner {
    x: i32,
    y: i32,
    time: i32,
    flags: Type,
    hit_sound: HitSound,
    end_time: i32,
    hit_sample: HitSample,
}
impl FromStr for Spinner {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        let hit_sample = HitSample {
            ..Default::default()
        };
        let x = line
            .next()
            .expect("in x assignment in Spinner parsing")
            .parse::<i32>()
            .expect("in i32 parsing in x assignment in Spinner parsing");
        let y = line
            .next()
            .expect("in y assignment in Spinner parsing")
            .parse::<i32>()
            .expect("in i32 parsing in y assignment in Spinner parsing");
        let time = line
            .next()
            .expect("in time assignment in Spinner parsing")
            .parse::<i32>()
            .expect("in i32 parsing in time assignment in Spinner parsing");
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
            .parse::<i32>()
            .expect("in i32 parsing in end_time assignment in Spinner parsing");
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
