use dirs::home_dir;
use std::{collections::HashMap, fs::read_to_string};

fn check_queue(ordering_rules: &HashMap<String, Vec<String>>, current_data: &Vec<String>) -> i64 {
    // for every entry, echeck the next one until the start of the list is reached. Check
    // if the next entry needs to be ordered after the current entry.
    for entry_idx in 0..current_data.len() {
        let cur_value = current_data.get(entry_idx).unwrap();
        for value_idx in entry_idx + 1..current_data.len() {
            let next_value = current_data.get(value_idx).unwrap();
            // is there an ordering rule for the current entry?
            if ordering_rules.get(cur_value).is_some() {
                // yes --> check if the next value should be positioned after the current value. if
                // yes, return the current-value since this is the first entry that breaks a rule.
                // This works since the queue-data was inverted prior to the function.
                if ordering_rules.get(cur_value).unwrap().contains(next_value) {
                    return entry_idx as i64;
                }
            }
        }
    }
    return -1;
}

fn sort_queue(
    ordering_rules: &HashMap<String, Vec<String>>,
    current_data: &Vec<String>,
    first_entry_idx: usize,
) -> Vec<String> {
    // go through the dataset (it was reversed prior to the function)
    // start at the first entry (returned from function that checks if the queue is ok). this is
    // the first entry with an error.
    // check if the next entry has a rule that prevents it to be exchanged with the problematic
    // entry. If no problem exists, exchange the positions.
    // if an entry exists, add the problematic entry to the other entry and bubble both entries up
    // until the entries cannot be moved further.
    //
    let mut current_queue = current_data.clone();
    current_queue.reverse();
    // initiate the vector that will contain all elements to move with the first entry
    // new elements are added to the front
    let mut elements_to_move: Vec<String> =
        vec![format!("{}", current_data.get(first_entry_idx).unwrap())];
    // start check with first entry after the starting one
    for entry_idx in first_entry_idx + 1..current_data.len() {
        let next_value = current_data.get(entry_idx).unwrap();
        let mut insert_at_end = true;
        for move_entry_idx in 0..elements_to_move.len() {
            let entry = elements_to_move.get(move_entry_idx).unwrap();
            // first check if an ordering rule exist for the next value
            if ordering_rules.get(next_value).is_some() {
                if ordering_rules.get(next_value).unwrap().contains(entry) {
                    elements_to_move.insert(move_entry_idx, next_value.to_string());
                    insert_at_end = false;
                    break;
                }
            }
        }
        if insert_at_end {
            elements_to_move.insert(elements_to_move.len(), next_value.to_string());
        }
    }

    elements_to_move.reverse();

    for entry_idx in (0..first_entry_idx).rev() {
        let value = current_data.get(entry_idx).unwrap();
        elements_to_move.insert(0, value.to_string());
    }

    let resulting_queue = elements_to_move;
    return resulting_queue;
}

fn main() {
    let contents: Vec<String> = read_to_string(format!(
        "{}/git_repos/adventofcode/2024/day_05/rust/src/input.txt",
        home_dir().unwrap().display().to_string()
    ))
    .unwrap()
    .split("\n")
    .map(|s| s.to_string())
    //filter out empty rows
    .filter(|s| !s.trim().is_empty())
    .collect();

    //HashMap to collect to ordering rules (key-value is left column, vector is a collection of all
    //rules belonging to this key, i.e. a collection of the right column with the key value in the
    //left column)
    let mut ordering_rules: HashMap<String, Vec<String>> = HashMap::new();

    // get rows with ordering rules
    for line in contents.clone() {
        if line.contains("|") {
            let current_rule = line.split_once("|").unwrap();
            if !ordering_rules.get(&current_rule.0.to_string()).is_some() {
                // no rules for key-value (left column of input ordering rules file) exist
                ordering_rules.insert(current_rule.0.to_string(), vec![current_rule.1.to_string()]);
            } else {
                // one rule for the key exists --> add an additional rule to this key
                ordering_rules
                    .entry(current_rule.0.to_string())
                    .or_default()
                    .push(current_rule.1.to_string());
            }
        }
    }

    let mut sum: u64 = 0;

    // now loop through the data
    for line in &contents {
        if !line.contains("|") {
            let mut current_data: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
            //reverse the direction since we need to check if a preceding value is not allowed
            current_data.reverse();
            let printing_queue_err_idx = check_queue(&ordering_rules, &current_data);

            // if printing queue is ok, add middle value to sum
            if printing_queue_err_idx == -1 {
                let value: u64 = current_data
                    .get(current_data.len() / 2)
                    .unwrap()
                    .parse()
                    .unwrap();
                sum += value;
            }
        }
    }
    println!("Result for part 1: {sum}");
    sum = 0;
   
    for line in contents {
        if !line.contains("|") {
            let mut current_data: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
            //reverse the direction since we need to check if a preceding value is not allowed
            current_data.reverse();

            while true {
                let printing_queue_err_idx = check_queue(&ordering_rules, &current_data);
                if printing_queue_err_idx == -1 {
                    break;
                }

                current_data = sort_queue(
                    &ordering_rules,
                    &current_data,
                    printing_queue_err_idx as usize,
                );

                // only add the entries that were not correct before
                let value: u64 = current_data
                    .get(current_data.len() / 2)
                    .unwrap()
                    .parse()
                    .unwrap();
                sum += value;
                break;
            }
        }
    }
    println!("Result for part 2: {sum}");
}
