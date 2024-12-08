use dirs::home_dir;
use std::fs::read_to_string;

enum Direction {
    N,
    Ne,
    E,
    Se,
    S,
    Sw,
    W,
    Nw,
}

impl Direction {
    // Returns an iterator over all variants
    fn directions() -> &'static [Direction] {
        &[
            Direction::N,
            Direction::Ne,
            Direction::E,
            Direction::Se,
            Direction::S,
            Direction::Sw,
            Direction::W,
            Direction::Nw,
        ]
    }
}

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

fn search_xmas(
    contents: &Vec<String>,
    line_idx: i64,
    entry_idx: i64,
    direction: &Direction,
    char_idx: u8,
) -> bool {
    let next_line_idx: i64;
    let next_entry_idx: i64;
    match direction {
        Direction::N => {
            next_line_idx = line_idx - 1;
            next_entry_idx = entry_idx;
        }
        Direction::Ne => {
            next_line_idx = line_idx - 1;
            next_entry_idx = entry_idx + 1;
        }
        Direction::E => {
            next_line_idx = line_idx;
            next_entry_idx = entry_idx + 1;
        }
        Direction::Se => {
            next_line_idx = line_idx + 1;
            next_entry_idx = entry_idx + 1;
        }
        Direction::S => {
            next_line_idx = line_idx + 1;
            next_entry_idx = entry_idx;
        }
        Direction::Sw => {
            next_line_idx = line_idx + 1;
            next_entry_idx = entry_idx - 1;
        }
        Direction::W => {
            next_line_idx = line_idx;
            next_entry_idx = entry_idx - 1;
        }
        Direction::Nw => {
            next_line_idx = line_idx - 1;
            next_entry_idx = entry_idx - 1;
        }
    }
    // only continue if the next entry is still in the matrix and not out-of-bounds and the word
    // XMAS was completeley found
    if char_idx == 3 {
        return true;
    } else {
        let next_char_idx: usize = (char_idx + 1) as usize;
        // the word is not complete
        if next_entry_idx >= 0
            && next_line_idx >= 0
            && "XMAS".chars().nth(next_char_idx)
                == contents
                    .get(next_line_idx as usize)
                    .unwrap()
                    .chars()
                    .nth(next_entry_idx as usize)
        {
            // continue in the given direction
            return search_xmas(
                contents,
                next_line_idx,
                next_entry_idx,
                direction,
                char_idx + 1,
            );
        } else {
            // word is not complete and the end of the matrix is reached
            return false;
        }
    }
}

fn get_entry(
    contents: &Vec<String>,
    last_line_idx: i64,
    line_idx_mod: i64,
    last_entry_idx: i64,
    entry_idx_mod: i64,
) -> char {
    let next_line_idx = last_line_idx + line_idx_mod;
    let next_entry_idx = last_entry_idx + entry_idx_mod;
    // only return character,if not out-of-bounds
    if next_line_idx >= 0 && next_entry_idx >= 0 {
        match contents.get(next_line_idx as usize) {
            Some(line) => {
                match line.chars().nth(next_entry_idx as usize) {
                    Some(entry) => {
                        // return found character if it was M or S
                        if ['M', 'S'].contains(&entry) {
                            return entry;
                        } else {
                            return ' ';
                        }
                    }
                    None => return ' ',
                }
            }
            // Out of bounds on the right or bottom
            None => return ' ',
        }
    } else {
        return ' ';
    }
}

fn search_mas(
    contents: &Vec<String>,
    line_idx: i64,
    entry_idx: i64,
    direction: &Direction,
) -> bool {
    match direction {
        Direction::Ne => {
            match get_entry(contents, line_idx, 1, entry_idx, 1) {
                // if none of the searched chars M or S was found, return false, otherwise search for the
                // missing char in reversed direction
                'M' => match get_entry(contents, line_idx, -1, entry_idx, -1) {
                    'S' => return true,
                    _ => return false,
                },
                'S' => match get_entry(contents, line_idx, -1, entry_idx, -1) {
                    'M' => return true,
                    _ => return false,
                },
                _ => return false,
            };
        }
        Direction::Nw => {
            match get_entry(contents, line_idx, 1, entry_idx, -1) {
                // if none of the searched chars M or S was found, return false, otherwise search for the
                // missing char in reversed direction
                'M' => match get_entry(contents, line_idx, -1, entry_idx, 1) {
                    'S' => return true,
                    _ => return false,
                },
                'S' => match get_entry(contents, line_idx, -1, entry_idx, 1) {
                    'M' => return true,
                    _ => return false,
                },
                _ => return false,
            };
        }
        _ => return false,
    }
}

fn main() {
    let contents: Vec<String> = read_to_string(resolve_home(
        "~/git_repos/adventofcode/2024/day_04/rust/src/input.txt".to_string(),
    ))
    .unwrap()
    .split("\n")
    .map(|s| s.to_string())
    .collect();

    let mut line_idx: i64 = 0;
    let char_idx = 0;

    let mut sum: u64 = 0;

    for line in &contents {
        let mut entry_idx: i64 = 0;
        for entry in line.chars() {
            if entry == "XMAS".chars().nth(0).unwrap() {
                // potential start of XMAS found.
                // check the next entries in the inputmatrix in all directions
                for direction in Direction::directions() {
                    if search_xmas(&contents, line_idx, entry_idx, &direction, char_idx) {
                        sum += 1;
                    }
                }
            }
            entry_idx += 1;
        }
        line_idx += 1;
    }
    println!("Result for part 1: {sum}");

    sum = 0;
    line_idx = 0;

    for line in &contents {
        let mut entry_idx: i64 = 0;
        for entry in line.chars() {
            if entry == 'A' {
                // potential start of XMAS found.
                // check the next entries in the inputmatrix in directions Nw and Ne
                if search_mas(&contents, line_idx, entry_idx, &Direction::Ne)
                    && search_mas(&contents, line_idx, entry_idx, &Direction::Nw)
                {
                    sum += 1;
                }
            }
            entry_idx += 1;
        }
        line_idx += 1;
    }

    println!("Result for part 2: {sum}");
}
