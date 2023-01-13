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
    let mut tabular = content.next().unwrap().lines();
    map.file_format = tabular
        .next()
        .unwrap()
        .split("v")
        .last()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();
    for line in tabular {
        if line.starts_with("[") || line.trim().is_empty() {
            // Filter out junk lines.
            continue;
        }
        let line = line.split_once(":").unwrap();
        let key = line.0.trim();
        let value = line.1.trim();
        match key {
            "AudioFilename" => map.audio_filename = Some(value.to_string()),
            "AudioLeadIn" => map.audio_lead_in = value.parse::<i32>().unwrap(),
            "AudioHash" => map.audio_hash = Some(value.to_string()),
            "PreviewTime" => map.preview_time = value.parse::<i32>().unwrap(),
            "Countdown" => map.countdown = value.parse::<Countdown>().unwrap(),
            "SampleSet" => map.sample_set = value.parse::<SampleSet>().unwrap(),
            "StackLeniency" => map.stack_leniency = value.parse::<f32>().unwrap(),
            "Mode" => map.mode = value.parse::<Mode>().unwrap(),
            "LetterboxInBreaks" => map.letterbox_in_breaks = value.parse::<bool>().unwrap(),
            "StoryFireInFront" => map.story_fire_in_front = value.parse::<bool>().unwrap(),
            "UseSkinSprites" => map.use_skin_sprites = value.parse::<bool>().unwrap(),
            "AlwaysShowPlayField" => map.always_show_play_field = value.parse::<bool>().unwrap(),
            "OverlayPosition" => map.overlay_position = value.parse::<OverlayPosition>().unwrap(),
            "SkinPreference" => map.skin_preference = Some(value.to_string()),
            "EpilepsyWarning" => map.epilepsy_warning = value.parse::<bool>().unwrap(),
            "CountdownOffset" => map.countdown_offset = value.parse::<i32>().unwrap(),
            "SpecialStyle" => map.special_style = value.parse::<bool>().unwrap(),
            "WidescreenStoryboard" => map.widescreen_storyboard = value.parse::<bool>().unwrap(),
            "SamplesMatchPlaybackRate" => {
                map.samples_match_playback_rate = value.parse::<bool>().unwrap()
            }
            "Bookmarks" => {
                let mut bookmarks = Vec::new();
                for i in value.split(',') {
                    bookmarks.push(i.parse::<i32>().unwrap());
                }
                map.bookmarks = Some(bookmarks);
            }
            "DistanceSpacing" => map.distance_spacing = Some(value.parse::<f32>().unwrap()),
            "BeatDivisor" => map.beat_divisor = Some(value.parse::<i32>().unwrap()),
            "GridSize" => map.grid_size = Some(value.parse::<i32>().unwrap()),
            "TimelineZoom" => map.timeline_zoom = Some(value.parse::<f32>().unwrap()),
            "Title" => map.title = Some(value.to_string()),
            "TitleUnicode" => map.title_unicode = Some(value.to_string()),
            "Artist" => map.artist = Some(value.to_string()),
            "ArtistUnicode" => map.artist_unicode = Some(value.to_string()),
            "Creator" => map.creator = Some(value.to_string()),
            "PubVersion" => map.version = Some(value.to_string()),
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
            "BeatmapID" => map.beatmap_id = Some(value.parse::<i32>().unwrap()),
            "BeatmapSetID" => map.beatmap_set_id = Some(value.parse::<i32>().unwrap()),
            "HPDrainRate" => map.hpdrain_rate = Some(value.parse::<f32>().unwrap()),
            "CircleSize" => map.circle_size = Some(value.parse::<f32>().unwrap()),
            "OverallDifficulty" => map.overall_difficulty = Some(value.parse::<f32>().unwrap()),
            "ApproachRate" => map.approach_rate = Some(value.parse::<f32>().unwrap()),
            "SliderMultiplier" => map.slider_multiplier = Some(value.parse::<f32>().unwrap()),
            "SliderTickRate" => map.slider_tick_rate = Some(value.parse::<f32>().unwrap()),
            _ => panic!("Unknown key: {}", key),
        }
    }
    let mut mixed = content.next().unwrap().split("[TimingPoints]");
    let events = mixed.next().unwrap().lines();
    for line in events {
        if line.starts_with("//") || line.trim().is_empty() {
            // Filter out junk lines.
            continue;
        }
        match line.chars().next().unwrap() {
            '0' => {
                // Background event.
                map.background = Some(line.parse::<Background>().unwrap());
            }
            '2' => {
                // Break event.
                if map.breaks.is_none() {
                    map.breaks = Some(Vec::new());
                }
                map.breaks
                    .as_mut()
                    .unwrap()
                    .push(line.parse::<Break>().unwrap());
            }
            _ => {}
        }
    }
    let mixed = mixed.next().unwrap();
    let colours_exists = mixed.contains("[Colours]");
    let mut mixed = mixed.split(if colours_exists {
        "[Colours]"
    } else {
        "[HitObjects]"
    });
    let timing_points = mixed.next().unwrap().lines();
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
            .unwrap()
            .push(line.parse::<TimingPoint>().unwrap());
    }
    if colours_exists {
        let mut mixed = mixed.next().unwrap().split("[HitObjects]");
        let colours = mixed.next().unwrap().lines();
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
                .unwrap()
                .push(line.parse::<Color>().unwrap());
        }
    }
    let hit_objects = mixed.next().unwrap().lines();
    for line in hit_objects {
        match line
            .split(',')
            .nth(3)
            .unwrap()
            .parse::<Type>()
            .unwrap()
            .object_type
        {
            ObjectType::Circle => {
                if map.circles.is_none() {
                    map.circles = Some(Vec::new());
                }
                map.circles
                    .as_mut()
                    .unwrap()
                    .push(line.parse::<Circle>().unwrap());
            }
            ObjectType::Slider => {
                if map.sliders.is_none() {
                    map.sliders = Some(Vec::new());
                }
                map.sliders
                    .as_mut()
                    .unwrap()
                    .push(line.parse::<Slider>().unwrap());
            }
            ObjectType::Spinner => {
                if map.spinners.is_none() {
                    map.spinners = Some(Vec::new());
                }
                map.spinners
                    .as_mut()
                    .unwrap()
                    .push(line.parse::<Spinner>().unwrap());
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(SampleSet::Default),
            "1" => Ok(SampleSet::Normal),
            "2" => Ok(SampleSet::Soft),
            "3" => Ok(SampleSet::Drum),
            _ => Err(()),
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
        let mut background = line.next().unwrap().chars();
        background.next();
        background.next_back();
        // Remove quotes
        let background = background.as_str().to_string();
        let x = line.next().unwrap_or("0").parse::<i32>().unwrap();
        let y = line.next().unwrap_or("0").parse::<i32>().unwrap();
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
        let start_time = line.next().unwrap().parse::<i32>().unwrap();
        let end_time = line.next().unwrap().parse::<i32>().unwrap();
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
                let time = line.next().unwrap().parse::<i32>().unwrap();
                let beat_length = line.next().unwrap().parse::<f32>().unwrap();
                Ok(Self {
                    time,
                    beat_length,
                    ..Default::default()
                })
            }
            6 => {
                let mut line = s.split(',');
                let time = line.next().unwrap().parse::<i32>().unwrap();
                let beat_length = line.next().unwrap().parse::<f32>().unwrap();
                let meter = line.next().unwrap().parse::<i32>().unwrap();
                let sample_set = line.next().unwrap().parse::<SampleSet>().unwrap();
                let volume = line.next().unwrap().parse::<i32>().unwrap();
                let uninherited = line.next().unwrap().parse::<i32>().unwrap() == 1;
                let effects = line.next().unwrap().parse::<Effects>().unwrap();
                Ok(Self {
                    time,
                    beat_length,
                    meter,
                    sample_set,
                    volume,
                    uninherited,
                    effects,
                })
            }
            _ => Err("Invalid timing point".into()),
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
        let mut line = s.split(" : ").skip(1).next().unwrap().split(',');
        let red = line.next().unwrap().parse::<u8>().unwrap();
        let green = line.next().unwrap().parse::<u8>().unwrap();
        let blue = line.next().unwrap().parse::<u8>().unwrap();
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
        let mut num: i32 = s.parse().unwrap();
        let mut bits = [false; 8];
        if num > 2 ^ 8 - 1 {
            panic!("Invalid Type");
        }
        if num > 2 ^ 7 - 1 {
            bits[7] = true;
            num -= 2 ^ 7;
        }
        if num > 2 ^ 6 - 1 {
            bits[6] = true;
            num -= 2 ^ 6;
        }
        if num > 2 ^ 5 - 1 {
            bits[5] = true;
            num -= 2 ^ 5;
        }
        if num > 2 ^ 4 - 1 {
            bits[4] = true;
            num -= 2 ^ 4;
        }
        if num > 2 ^ 3 - 1 {
            bits[3] = true;
            num -= 2 ^ 3;
        }
        if num > 2 ^ 2 - 1 {
            bits[2] = true;
            num -= 2 ^ 2;
        }
        if num > 2 ^ 1 - 1 {
            bits[1] = true;
            num -= 2 ^ 1;
        }
        if num > 2 ^ 0 - 1 {
            bits[0] = true;
            num -= 2 ^ 0;
        }
        if num > 0 {
            panic!("Logic error in Type creation")
        }
        let mut color_skip = 0;
        if bits[4] {
            color_skip += 2 ^ 2;
        }
        if bits[5] {
            color_skip += 2 ^ 1;
        }
        if bits[6] {
            color_skip += 2 ^ 0;
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
        let mut num: i32 = s.parse().unwrap();
        let mut bits = [false; 8];
        if num > 2 ^ 4 - 1 {
            panic!("Invalid HitSound");
        }
        if num > 2 ^ 3 - 1 {
            bits[3] = true;
            num -= 2 ^ 3;
        }
        if num > 2 ^ 2 - 1 {
            bits[2] = true;
            num -= 2 ^ 2;
        }
        if num > 2 ^ 1 - 1 {
            bits[1] = true;
            num -= 2 ^ 1;
        }
        if num > 2 ^ 0 - 1 {
            bits[0] = true;
            num -= 2 ^ 0;
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
        let values = s.split_once(":").unwrap();
        let normal_set = values.0.parse::<SampleSet>().unwrap();
        let values = values.1.split_once(":").unwrap();
        let addition_set = values.0.parse::<SampleSet>().unwrap();
        let values = values.1.split_once(":").unwrap();
        let index = values.0.parse::<i32>().unwrap();
        let values = values.1.split_once(":").unwrap();
        let volume = values.0.parse::<i32>().unwrap();
        let values = values.1.split_once(":").unwrap();
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
        let hit_sample = // if s.contains(":") {
        //     line.last().unwrap().parse::<HitSample>().unwrap()
        // } else {
            HitSample {
                ..Default::default()
            };
        // };
        // TODO: Parse HitSamples
        let x = line.next().unwrap().parse::<i32>().unwrap();
        let y = line.next().unwrap().parse::<i32>().unwrap();
        let time = line.next().unwrap().parse::<i32>().unwrap();
        let flags = line.next().unwrap().parse::<Type>().unwrap();
        let hit_sound = line.next().unwrap().parse::<HitSound>().unwrap();
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
        let _type = line.next().unwrap().parse::<CurveType>().unwrap();
        let mut points = Vec::new();
        for pair in line {
            let mut pair = pair.split('|');
            points.push(Point {
                x: pair.next().unwrap().parse::<i32>().unwrap(),
                y: pair.next().unwrap().parse::<i32>().unwrap(),
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
    edge_sets: Vec<HitSample>,
    hit_sample: HitSample,
}
impl FromStr for Slider {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        let hit_sample = // if s.contains(":") {
        //     line.last().unwrap().parse::<HitSample>().unwrap()
        // } else {
            HitSample {
                ..Default::default()
            };
        // };
        // TODO: Parse HitSamples
        let x = line.next().unwrap().parse::<i32>().unwrap();
        let y = line.next().unwrap().parse::<i32>().unwrap();
        let time = line.next().unwrap().parse::<i32>().unwrap();
        let flags = line.next().unwrap().parse::<Type>().unwrap();
        let hit_sound = line.next().unwrap().parse::<HitSound>().unwrap();
        let collected = line.collect::<String>();
        let commas = collected.matches(',').count();
        let mut line = collected.split(',');
        let curve = match commas {
            2 | 5 => line.next().unwrap().parse::<Curve>().unwrap(),
            _ => panic!("Invalid slider: wrong remaining line size ({})", commas),
        };
        let slides = match commas {
            2 | 5 => line.next().unwrap().parse::<i32>().unwrap(),
            _ => panic!("Invalid slider: wrong remaining line size ({})", commas),
        };
        let length = match commas {
            2 | 5 => line.next().unwrap().parse::<f32>().unwrap(),
            _ => panic!("Invalid slider: wrong remaining line size ({})", commas),
        };
        let edge_sounds = match commas {
            5 => {
                let mut sounds = Vec::new();
                for sound in line.next().unwrap().split('|') {
                    sounds.push(sound.parse::<HitSound>().unwrap());
                }
                sounds
            }
            _ => vec![
                "0".parse::<HitSound>().unwrap(),
                "2".parse::<HitSound>().unwrap(),
            ],
        };
        let edge_sets = match commas {
            5 => {
                let mut sounds = Vec::new();
                for sound in line.next().unwrap().split('|') {
                    sounds.push(sound.parse::<HitSample>().unwrap());
                }
                sounds
            }
            _ => vec![
                "0:0".parse::<HitSample>().unwrap(),
                "0:0".parse::<HitSample>().unwrap(),
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
        let hit_sample = // if s.contains(":") {
        //     line.last().unwrap().parse::<HitSample>().unwrap()
        // } else {
            HitSample {
                ..Default::default()
            };
        // };
        // TODO: Parse HitSamples
        let x = line.next().unwrap().parse::<i32>().unwrap();
        let y = line.next().unwrap().parse::<i32>().unwrap();
        let time = line.next().unwrap().parse::<i32>().unwrap();
        let flags = line.next().unwrap().parse::<Type>().unwrap();
        let hit_sound = line.next().unwrap().parse::<HitSound>().unwrap();
        let end_time = line.next().unwrap().parse::<i32>().unwrap();
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
