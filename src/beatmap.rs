pub mod customization;
pub mod difficulty;
pub mod editor;
pub mod filedata;
pub mod hit_objects;
pub mod metadata;
pub mod mode;

use customization::Customization;
use difficulty::Difficulty;
use editor::Editor;
use filedata::Filedata;
use hit_objects::HitObjects;
use metadata::Metadata;
use mode::Mode;

pub struct Beatmap {
    pub mode: Option<Mode>,
    pub customization: Option<Box<Customization>>,
    pub difficulty: Option<Box<Difficulty>>,
    pub editor: Option<Box<Editor>>,
    pub filedata: Option<Box<Filedata>>,
    pub hit_objects: Option<Box<HitObjects>>,
    pub metadata: Option<Box<Metadata>>,
}
