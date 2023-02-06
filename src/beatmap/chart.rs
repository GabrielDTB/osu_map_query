pub mod hit_object;
pub mod timing_point;

use hit_object::HitObject;
use num::rational::Ratio;
use timing_point::TimingPoint;

#[derive(Debug, Clone, PartialEq)]
pub struct Chart {
    pub stack_leniency: Ratio<i64>,
    pub slider_multiplier: Ratio<i64>,
    pub slider_tick_rate: Ratio<i64>,
    pub timing_points: Vec<TimingPoint>,
    pub hit_objects: Vec<HitObject>,
}
