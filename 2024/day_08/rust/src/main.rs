use dirs::home_dir;
use std::fs::read_to_string;

#[derive(Debug)]
struct Antenna {
    line: usize,
    entry: usize,
    antenna_type: char,
}

fn main() {
    let contents: Vec<String> = read_to_string(format!(
        "{}/git_repos/adventofcode/2024/day_08/rust/src/test_input.txt",
        home_dir().unwrap().display().to_string()
    ))
    .unwrap()
    .split("\n")
    .map(|s| s.to_string())
    //filter out empty rows
    .filter(|s| !s.trim().is_empty())
    .collect();

    let contents_map: Vec<Vec<char>> = contents.iter().map(|c| c.chars().collect()).collect();

    let mut antenna_locations: Vec<Antenna> = vec![];
    let mut result = contents_map.clone();

    // check every entry if it is an antenna, store type and position of antenna
    for line in 0..contents_map.len() - 1 {
        for entry in 0..contents_map[line].len() - 1 {
            if contents_map[line][entry] != '.' {
                antenna_locations.push(Antenna {
                    line: line,
                    entry: entry,
                    antenna_type: contents_map[line][entry],
                });
            }
        }
    }
    for antenna in antenna_locations {
        println!("{:?}", antenna);
    }
}
