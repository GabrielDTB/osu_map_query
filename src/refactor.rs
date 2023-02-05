pub struct Beatmap {
    pub general: Option<Box<General>>,
    pub editor: Option<Box<Editor>>,
    pub metadata: Option<Box<Metadata>>,
    pub difficulty: Option<Box<Difficulty>>,
    pub events: Option<Box<Events>>,
    pub timing_points: Option<Box<TimingPoints>>,
    // Colours
    //pub colors: Option<Vec<Color>>,
    pub hit_objects: Option<Box<HitObjects>>,
}

pub struct General {
    // pub file_format: Option<i64>,
    // pub audio_filename: Option<String>,
    // pub audio_lead_in: Option<i64>,
    // pub audio_hash: Option<String>, // Deprecated
    // pub preview_time: Option<i64>,
    // pub countdown: Option<Countdown>,
    // pub sample_set: Option<SampleSet>,
    // pub stack_leniency: Option<Ratio<i64>>,
    // pub mode: Option<Mode>,
    // pub letterbox_in_breaks: Option<bool>,
    // pub story_fire_in_front: Option<bool>, // Deprecated
    // pub use_skin_sprites: Option<bool>,
    // pub always_show_play_field: Option<bool>, // Deprecated
    // pub overlay_position: Option<OverlayPosition>,
    // pub skin_preference: Option<String>,
    // pub epilepsy_warning: Option<bool>,
    // pub countdown_offset: Option<i64>,
    // pub special_style: Option<bool>,
    // pub widescreen_storyboard: Option<bool>,
    // pub samples_match_playback_rate: Option<bool>,
}

pub struct Editor {
    // pub bookmarks: Option<Vec<i64>>,
    // pub distance_spacing: Option<Ratio<i64>>,
    // pub beat_divisor: Option<i64>,
    // pub grid_size: Option<i64>,
    // pub timeline_zoom: Option<Ratio<i64>>,
}

pub struct Metadata {
    // pub title: Option<String>,
    // pub title_unicode: Option<String>,
    // pub artist: Option<String>,
    // pub artist_unicode: Option<String>,
    // pub creator: Option<String>,
    // pub version: Option<String>,
    // pub source: Option<String>,
    // pub tags: Option<Vec<String>>,
    // pub beatmap_id: Option<i64>,
    // pub beatmap_set_id: Option<i64>,
}

pub struct Difficulty {
    // pub hpdrain_rate: Option<Ratio<i64>>,
    // pub circle_size: Option<Ratio<i64>>,
    // pub overall_difficulty: Option<Ratio<i64>>,
    // pub approach_rate: Option<Ratio<i64>>,
    // pub slider_multiplier: Option<Ratio<i64>>,
    // pub slider_tick_rate: Option<Ratio<i64>>,
}

pub struct Events {
    // pub background: Option<Background>,
    // pub breaks: Option<Vec<Break>>,
}

pub struct TimingPoints {
    // pub timing_points: Option<Vec<TimingPoint>>,
}
pub struct HitObjects {
    // pub hit_objects: Option<Vec<HitObject>>,
}
