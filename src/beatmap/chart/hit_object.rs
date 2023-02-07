pub mod _type;
pub mod circle;
pub mod hit_sample;
pub mod hit_sound;
pub mod slider;
pub mod spinner;
pub mod traits;

use circle::Circle;
use slider::Slider;
use spinner::Spinner;

#[derive(Debug, Clone, PartialEq)]
pub enum HitObject {
    Circle(Circle),
    Slider(Slider),
    Spinner(Spinner),
}
