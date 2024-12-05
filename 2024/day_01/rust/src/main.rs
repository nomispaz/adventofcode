use std::fs::read_to_string;

fn abs(x: u64, y: u64) -> u64 {
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn main() {
    let contents: Vec<String> = read_to_string("./input.txt")
        .unwrap()
        .split("\n")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    let mut col1: Vec<u64> = vec![];
    let mut col2: Vec<u64> = vec![];

    for line in contents {
        let line_split: Vec<u64> = line.split("   ").map(|s| s.parse().unwrap()).collect();
        col1.push(line_split[0].clone());
        col2.push(line_split[1].clone());
    }
    col1.sort();
    col2.sort();

    let mut sum = 0;
    for i in 0..col1.len() {
        sum += abs(col1[i], col2[i]);
    }

    println!("Result for part 1: {}", sum);

    sum = 0;

    // find number of entrys in the second column that are the same as in the first column.
    for entry in col1 {
        let mut num_found = 0;
        for entry_2 in col2.clone() {
            match entry {
                _ if entry < entry_2 => (),
                _ if entry == entry_2 => {
                    num_found += 1;
                }
                _ if entry > entry_2 && num_found == 0 => (),
                // break if the entry from first column cannot be found anymore
                _ => break,
            }
        }
        sum += num_found * entry
    }

    println!("Result for part 2: {}", sum);
}
