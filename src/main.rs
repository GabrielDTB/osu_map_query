#![allow(dead_code)]

pub mod parser;

fn main() {
    let _harumachi_clover = "/home/gabrieltb/Documents/Coding/osu!mapquery/osu_map_query/test_maps/Will Stetson - Harumachi Clover (Swing Arrangement) [Dictate Edit] (Sotarks) [Oh no!].osu";
    let _apparition = "/home/gabrieltb/Documents/Coding/osu!mapquery/osu_map_query/test_maps/Spawn Of Possession - Apparition (Mazzerin) [Blind Faith].osu";
    let _ver3 = "/home/gabrieltb/Documents/Coding/osu!mapquery/osu_map_query/test_maps/Kenji Ninuma - DISCOБЪPRINCE (peppy) [Normal].osu";
    let map = parser::parse_map(_ver3);
    println!("{:?}", map);
}
