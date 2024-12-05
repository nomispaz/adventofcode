use dirs::home_dir;
use std::fs::read_to_string;

fn resolve_home(old_path: String) -> String {
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

fn max(x: u64, y: u64) -> u64 {
    if x < y {
        y
    } else {
        x
    }
}

fn is_safe_without_dampener(val1: i64, val2: i64, increasing: bool) -> u64 {
    match val1 - val2 {
        x if x < 0 && increasing && x > -4 => 1,
        x if x > 0 && !increasing && x < 4 => 1,
        _ => 0,
    }
}

fn is_safe_with_dampener(values: Vec<i64>, values_removed: u8) -> u64 {
    let values = values;
    let increasing = values[0] < values[1];
    let mut prev_value = values[0];
    let mut current_report_safe = 0;
    for idx in 1..values.len() {
        match prev_value - values[idx] {
            x if x < 0 && increasing && x > -4 => current_report_safe = 1,
            x if x > 0 && !increasing && x < 4 => current_report_safe = 1,
            x => {
                current_report_safe = 0;
                if values_removed > 0 {
                    current_report_safe = 0;
                } else {
                    // special case when direction (increasing/decreasing changes after the first
                    // three entries, it could help to remove the first entry)
                    if (x < 0 && !increasing || x > 0 && increasing) && idx == 2 {
                        for i in 0..3 {
                            let mut values_tmp = values.clone();
                            values_tmp.remove(i);
                            current_report_safe = max(current_report_safe, is_safe_with_dampener(values_tmp, 1));
                        }
                    } else {
                        let mut values_tmp_1 = values.clone();
                        values_tmp_1.remove(idx - 1);
                        let mut values_tmp_2 = values.clone();
                        values_tmp_2.remove(idx);
                        current_report_safe = max(
                            is_safe_with_dampener(values_tmp_1, 1),
                            is_safe_with_dampener(values_tmp_2, 1),
                        );
                    }
                }
                break;
            }
        }

        if current_report_safe == 0 {
            break;
        } else {
            prev_value = values[idx];
        }
    }
    return current_report_safe;
}

fn main() {
    let contents: Vec<String> = read_to_string(resolve_home(
        "~/Documents/aoc/2024/day_02/rust/src/input.txt".to_string(),
    ))
    .unwrap()
    .split("\n")
    .filter(|s| !s.trim().is_empty())
    .map(|s| s.to_string())
    .collect();

    let mut number_safe_reports: u64 = 0;

    for line in contents.clone() {
        // increasing is true if the values are increasing, not decreasing.
        let increasing: bool;
        let mut current_report_safe: u64 = 0;

        let line_split: Vec<i64> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        let mut prev_entry = line_split[0];
        if line_split[0] < line_split[1] {
            increasing = true;
        } else {
            increasing = false;
        }
        for i in 1..line_split.len() {
            current_report_safe = is_safe_without_dampener(prev_entry, line_split[i], increasing);
            if current_report_safe == 0 {
                break;
            } else {
                prev_entry = line_split[i];
            }
        }
        number_safe_reports += current_report_safe;
    }
    println!("Answer to part 1: {number_safe_reports}");

    number_safe_reports = 0;

    for line in contents {
        let line_split: Vec<i64> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        number_safe_reports += is_safe_with_dampener(line_split.clone(), 0);
    }
    println!("Answer to part 2: {number_safe_reports}");
}