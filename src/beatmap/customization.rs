pub mod _break;
pub mod background;
pub mod color;
pub mod countdown;
pub mod overlay_position;

use super::shared::sample_set::SampleSet;
use _break::Break;
use background::Background;
use countdown::Countdown;
use overlay_position::OverlayPosition;

pub struct Customization {
    pub sample_set: Option<SampleSet>,
    pub letterbox_in_breaks: Option<bool>,
    pub story_fire_in_front: Option<bool>, // Deprecated
    pub use_skin_sprites: Option<bool>,
    pub always_show_play_field: Option<bool>, // Deprecated
    pub overlay_position: Option<OverlayPosition>,
    pub skin_preference: Option<String>,
    pub epilepsy_warning: Option<bool>,
    pub countdown: Option<Countdown>,
    pub special_style: Option<bool>,
    pub widescreen_storyboard: Option<bool>,
    pub samples_match_playback_rate: Option<bool>,
    pub background: Option<Background>,
    pub breaks: Option<Vec<Break>>,
    pub colors: Option<Vec<color::Color>>,
}
