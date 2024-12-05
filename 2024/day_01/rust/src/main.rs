use std::fs::read_to_string;

fn abs(x: u64, y: u64) -> u64 {
//! takes two unsigned integers and returns the absolute value
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn main() {
    // read input-file into vector of strings and filter out empty rows
    let contents: Vec<String> = read_to_string("./input.txt")
        .unwrap()
        .split("\n")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    let mut col1: Vec<u64> = vec![];
    let mut col2: Vec<u64> = vec![];

    // loop through each line and split the line into two columns of unsigned integers.
    for line in contents {
        let line_split: Vec<u64> = line.split("   ").map(|s| s.parse().unwrap()).collect();
        col1.push(line_split[0].clone());
        col2.push(line_split[1].clone());
    }

    // sort both columns in ascending order
    col1.sort();
    col2.sort();

    // sum the difference (using the abs function) of the two columns element by element
    let mut sum = 0;
    for i in 0..col1.len() {
        sum += abs(col1[i], col2[i]);
    }

    println!("Result for part 1: {}", sum);

    sum = 0;

    for entry in col1 {
        let mut num_found = 0;
        for entry_2 in col2.clone() {
        // find number of entrys in the second column that are the same as in the first column.
            match entry {
                // if col1 < col2, return nothing
                _ if entry < entry_2 => (),
                // if both columns are equal, increase the count
                _ if entry == entry_2 => {
                    num_found += 1;
                }
                // if a number was found and we are no passed it in col2, return nothing
                _ if entry > entry_2 && num_found == 0 => (),
                // break if the entry from first column cannot be found anymore
                _ => break,
            }
        }
        // sum the counts*entry-values
        sum += num_found * entry
    }

    println!("Result for part 2: {}", sum);
}
