use dirs::home_dir;
use regex::Regex;
use std::fs::read_to_string;

fn resolve_home(old_path: String) -> String {
//! used to reslve the home-path of the input-directory
    //if path of config-file contains ~ or $HOME, parse to the real home dir
    let home_dir: String = home_dir().unwrap().display().to_string();

    let mut new_path: String = old_path;

    if new_path.trim().starts_with("~") {
        new_path = new_path.replace("~", &home_dir);
    }

    if new_path.trim().starts_with("$HOME") {
        new_path = new_path.replace("$HOME", &home_dir);
    }

    new_path
}

fn collect_muls(contents: String) -> (String, String) {
//! helper function to split the string at don't and do and return the string-parts
    // split until do_not is found
    // match is needed to capture if the result of split is None. happens then don't() is not found
    // any more
    let (contents_split, remaining) = match contents.split_once("don't()") {
        Some((prev,remain)) => (prev.to_string(), remain.to_string()),
        None => (contents, "".to_string()),
    };

    // split remaining until do() but only use the then remaining part
    let (_, remaining) = match remaining.split_once("do()") {
        Some((prev, remain)) => (prev.to_string(), remain.to_string()),
        None => ("".to_string(), "".to_string()),
    };
        
    (contents_split.to_string(), remaining.to_string())
}

fn sum_muls(contents: &str) -> u64 {
//! sum the mul-entries
    // via regex, get all entries that start with mul(, contain a sequence of numbers, followed by
    // a comma, again a sequence of numbers and ends with a ).
    let re = Regex::new(r"mul\([0-9]*,[0-9]*\)").unwrap();
    let mut sum: u64 = 0;

    let entries: Vec<String> = re
        .find_iter(&contents)
        .map(|s| s.as_str().to_string())
        .collect();

    for entry in entries {
        // extract only the numbers from the mul-entries
        let entry: Vec<u64> = entry
            .replace("mul(", "")
            .replace(")", "")
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        sum += entry[0] * entry[1];
    }

    sum
}

fn main() {
    let contents: String = read_to_string(resolve_home(
        "~/Documents/aoc/2024/day_03/rust/src/input.txt".to_string(),
    ))
    .unwrap();

    let mut sum = sum_muls(&contents);
    println!("Result for part1: {}", sum);

    let mut collected_muls: String = "".to_string();
    let mut remaining = contents;
    while remaining != "" {
        let collected_tmp: String;
        (collected_tmp, remaining) = collect_muls(remaining);
        collected_muls.push_str(&collected_tmp);
    }

    sum = sum_muls(&collected_muls);
    println!("Result for part 2: {}", sum);

}
