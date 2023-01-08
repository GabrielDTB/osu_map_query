#![allow(dead_code)]
// pub mod hit_objects;
pub mod osu_map;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let map = osu_map::Map::new("/home/gabrieltb/Documents/Coding/osu!mapquery/osu_map_query/test_maps/Will Stetson - Harumachi Clover (Swing Arrangement) [Dictate Edit] (Sotarks) [Oh no!].osu");
    println!("{:?}", map);
    let velocities = osu_map::velocities(&map);
    println!("Velocities {:?}", velocities);
    let optimal_velocities = osu_map::optimal_velocities(&map);
    println!("OptimalVel {:?}", optimal_velocities);
    let velocity_differences: Vec<f64> = optimal_velocities
        .into_iter()
        .zip(velocities)
        .map(|(a, b)| a - b)
        .collect();
    println!("Difference {:?}", velocity_differences);
    //println!("Paths {:?}", osu_map::optimal_paths(&map));
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
