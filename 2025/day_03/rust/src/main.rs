use std::fs::read_to_string;

fn read_file(filepath: String) -> Vec<String> {
    let contents: Vec<String> = read_to_string(filepath.clone())
        .expect(&format!(
            "The file: {} couldn't be converted to string",
            filepath
        ))
        .split("\n")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect();
    return contents;
}

fn find_battery(sequence: String, start_idx: usize, end_index: usize) -> (i64, i32) {
    let battery_list = sequence.chars();
    let mut joltage: i64 = 0;
    let mut max_joltage: i64 = 0;
    let mut max_joltage_idx: usize = 0;
    println!("{}, {}", start_idx, end_index);
    for idx in start_idx..end_index {
        joltage = i64::from(battery_list.clone().nth(idx).unwrap().to_digit(10).unwrap());
        if joltage > max_joltage {
            max_joltage = joltage;
            max_joltage_idx = idx;
        }
    }
    return (max_joltage, (max_joltage_idx + start_idx) as i32);
}

fn main() {
    let contents = read_file("../testinput.input".to_string());

    // part 1
    // let number_batteries = 2
    // part 2
    let number_batteries = 12;
    let mut total_output: i64 = 0;

    for line in contents {
        let mut battery_array: Vec<i64> = vec![];
        let mut combined_joltage = "".to_string();
        let mut battery_value = 0;
        let mut battery_idx: i32 = -1;

        let length = line.len();
        for idx in 0..number_batteries - 1 {
            (battery_value, battery_idx) = find_battery(
                line.clone(),
                (battery_idx + 1) as usize,
                length - (number_batteries - 1 - idx),
            );
            battery_array.push(battery_value);
        }

        for battery in battery_array {
            combined_joltage = format!("{}{}", combined_joltage, battery.to_string());
        }
        // TODO from here
        total_output += combined_joltage.parse::<i64>().unwrap();
        println!("{}, {}, {}", combined_joltage, total_output, line)
    }
    println!("{}", total_output)
}
