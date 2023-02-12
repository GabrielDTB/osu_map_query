pub mod chart;
pub mod customization;
pub mod difficulty;
pub mod editor;
pub mod filedata;
pub mod metadata;
pub mod mode;
pub mod parse;
pub mod shared;

use chart::Chart;
use customization::*;
use difficulty::Difficulty;
use editor::Editor;
use filedata::Filedata;
use metadata::Metadata;
use mode::Mode;

pub struct Beatmap {
    mode: Mode,
    customization: Option<Box<Customization>>,
    difficulty: Option<Box<Difficulty>>,
    editor: Option<Box<Editor>>,
    filedata: Option<Box<Filedata>>,
    chart: Option<Box<Chart>>,
    metadata: Option<Box<Metadata>>,
}
impl Beatmap {
    pub fn new(
        mode: Mode,
        customization: Option<Customization>,
        difficulty: Option<Difficulty>,
        editor: Option<Editor>,
        filedata: Option<Filedata>,
        chart: Option<Chart>,
        metadata: Option<Metadata>,
    ) -> Self {
        Beatmap {
            mode,
            customization: customization.map(Box::new),
            difficulty: difficulty.map(Box::new),
            editor: editor.map(Box::new),
            filedata: filedata.map(Box::new),
            chart: chart.map(Box::new),
            metadata: metadata.map(Box::new),
        }
    }
}
