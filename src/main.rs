#![allow(dead_code)]

pub mod analysis;
pub mod beatmap;
pub mod minimal_beatmap;
pub mod old_beatmap;
pub mod refactor;
pub mod utility;

use std::collections::HashMap;
//use std::path::Path;

fn main() {
    println!("{}", std::mem::size_of::<old_beatmap::Beatmap>());
    println!("{}", std::mem::size_of::<minimal_beatmap::Beatmap>());
    println!("{}", std::mem::size_of::<HashMap<String, Option<i64>>>());
    println!("{}", std::mem::size_of::<Vec<i64>>());
    println!("{}", std::mem::size_of::<Box<Vec<String>>>());
    println!("{}", std::mem::size_of::<refactor::Beatmap>());
    println!("{}", std::mem::size_of::<beatmap::Beatmap>());
    println!("{}", std::mem::size_of::<bool>());
    println!("{}", std::mem::size_of::<i32>());
    // let _harumachi_clover = Path::new("/home/gabrieltb/Documents/Coding/osu!mapquery/osu_map_query/test_maps/Will Stetson - Harumachi Clover (Swing Arrangement) [Dictate Edit] (Sotarks) [Oh no!].osu");
    // let _apparition = Path::new("/home/gabrieltb/Documents/Coding/osu!mapquery/osu_map_query/test_maps/Spawn Of Possession - Apparition (Mazzerin) [Blind Faith].osu");
    // let _ver3 = Path::new("/home/gabrieltb/Documents/Coding/osu!mapquery/osu_map_query/test_maps/Kenji Ninuma - DISCOБЪPRINCE (peppy) [Normal].osu");
    // let map = minimal_beatmap::parse_map(_ver3).unwrap();
    // println!("{}", map.details().unwrap());
}
