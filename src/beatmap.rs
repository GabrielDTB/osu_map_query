pub mod chart;
pub mod customization;
pub mod difficulty;
pub mod editor;
pub mod filedata;
pub mod metadata;
pub mod mode;

use chart::Chart;
use customization::Customization;
use difficulty::Difficulty;
use editor::Editor;
use filedata::Filedata;
use metadata::Metadata;
use mode::Mode;

pub struct Beatmap {
    pub mode: Option<Mode>,
    pub customization: Option<Box<Customization>>,
    pub difficulty: Option<Box<Difficulty>>,
    pub editor: Option<Box<Editor>>,
    pub filedata: Option<Box<Filedata>>,
    pub hit_objects: Option<Box<Chart>>,
    pub metadata: Option<Box<Metadata>>,
}
