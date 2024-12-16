use dirs::home_dir;
use core::panic;
use std::fs::read_to_string;

fn computing_step(starting_value: u64, next_value: u64, operator: &str) -> u64 {
    match operator {
        "+" => return starting_value + next_value,
        "*" => return starting_value * next_value,
        "||" => return format!("{}{}",starting_value.to_string(), next_value.to_string()).parse().unwrap(),
         _ => panic!("Error")
    }
}

fn computation_run(intermediate_results: Vec<u64>, operators: Vec<&str>, next_value: u64) -> Vec<u64> {
//! run for all operators and one set of intermediate results. Returns the next intermediate results
    let mut current_results: Vec<u64> = vec![];
    let mut current_result: u64;
    for value in intermediate_results {
        for operator in &operators {
            current_result = computing_step(value, next_value, operator);
            current_results.push(current_result);
        }
    }

    return current_results;
}

fn main() {
    let contents: Vec<String> = read_to_string(format!(
        "{}/git_repos/adventofcode/2024/day_07/rust/src/input.txt",
        home_dir().unwrap().display().to_string()
    ))
    .unwrap()
    .split("\n")
    .map(|s| s.to_string())
    //filter out empty rows
    .filter(|s| !s.trim().is_empty())
    .collect();

    // splits the file contents further into matrix where each line contains the numbers of the
    // line --> first entry is the result
    let contents_map: Vec<Vec<u64>> = contents
        .iter()
        .map(|s| {
            s.split([':', ' '])
                .filter(|p| !p.is_empty())
                .filter_map(|p| p.parse().ok())
                .collect()
        })
        .collect();

    let operators_part1 = vec!["+", "*"];

    // Algorithm:
    // build result of every two entries from the left for every operator.
    // save every partial result so that it can be computed together with the next entry for every
    // operator and again saved.
    // Do until no additional entry is read --> results in a list of final results that can be
    // compared to the equation result that is given
    // we need to know which equations can be solved and add the results
    // How to store:
    // during the computation: Vec<Vec<u64>> where the outer vector contains the computation step
    // and the inner vector all results of the step --> in the next computation step, all entries
    // in the inner vector need to be used
    
    let mut sum_results = 0;

    for mut equation in contents_map.clone() {

        // initialize the intermediate_results with the first value in the equation
        let mut intermediate_results: Vec<Vec<u64>> = vec![];
        // insert the first element of the equation after the expected result
        intermediate_results.push(vec![equation[1]]);
        // remove the expected result
        let expected_result = equation.get(0).unwrap().clone();
        equation.remove(0);
        // remove the first value that was already used to initialize the intermediate results
        equation.remove(0);

        for value in equation {
            intermediate_results.push(computation_run(intermediate_results[intermediate_results.len()-1].clone(), operators_part1.clone(), value));
        }

        for result in intermediate_results[intermediate_results.len()-1].clone() {
            if expected_result == result {
                sum_results += expected_result;
                // break is necessary so that an equation that could be solved differently is
                // counted twice.
                break;
            }
        }
    }

    println!("Result for part 1: {sum_results}");

    sum_results = 0;
    let operators_part2 = vec!["+", "*", "||"];
    
    for mut equation in contents_map.clone() {

        // initialize the intermediate_results with the first value in the equation
        let mut intermediate_results: Vec<Vec<u64>> = vec![];
        // insert the first element of the equation after the expected result
        intermediate_results.push(vec![equation[1]]);
        // remove the expected result
        let expected_result = equation.get(0).unwrap().clone();
        equation.remove(0);
        // remove the first value that was already used to initialize the intermediate results
        equation.remove(0);

        for value in equation {
            intermediate_results.push(computation_run(intermediate_results[intermediate_results.len()-1].clone(), operators_part2.clone(), value));
        }

        for result in intermediate_results[intermediate_results.len()-1].clone() {
            if expected_result == result {
                sum_results += expected_result;
                // break is necessary so that an equation that could be solved differently is
                // counted twice.
                break;
            }
        }
    }

    println!("Result for part 2: {sum_results}");

}
