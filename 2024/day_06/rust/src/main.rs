use core::panic;
use dirs::home_dir;
use std::fs::read_to_string;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    N,
    E,
    S,
    W,
    None,
}

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct VisitedLocation {
    line_idx: i64,
    entry_idx: i64,
}

fn is_obstacle(entry: char, direction: Direction) -> (Direction, bool) {
    //! checks if the given line contains an obstacle "#" and returns a direction, the guard turns to

    if entry == '#' {
        match direction {
            Direction::N => return (Direction::E, true),
            Direction::E => return (Direction::S, true),
            Direction::S => return (Direction::W, true),
            Direction::W => return (Direction::N, true),
            _ => panic!("Error while trying to determine next direction the guard takes."),
        }
    } else {
        return (direction, false);
    }
}

fn guard_move(
    contents: &Vec<Vec<char>>,
    next_line_idx: i64,
    next_entry_idx: i64,
    direction: Direction,
) -> (Direction, bool, bool) {
    let mut direction = direction;
    let hit_obstacle: bool;
    // only continue,if not out-of-bounds
    if next_line_idx >= 0 && next_entry_idx >= 0 && contents.len() > next_line_idx as usize && contents[0].len() > next_entry_idx as usize {
        // we are still in the field --> check if next line is an obstacle
        (direction, hit_obstacle) = is_obstacle(contents[next_line_idx as usize][next_entry_idx as usize], direction);
        return (direction, false, hit_obstacle);
    } else {
        // Out of bounds on left or top --> the guard left
        return (direction, true, false);
    }
}

fn create_turn_point(
    line_idx: i64,
    entry_idx: i64,
    direction: Direction,
    visited_turn_points: Vec<(VisitedLocation, Direction)>,
) -> (Vec<(VisitedLocation, Direction)>, bool) {
    let mut visited_turn_points = visited_turn_points;
    let mut visited_twice: bool = false;
    let location = VisitedLocation {
        line_idx: line_idx,
        entry_idx: entry_idx,
    };

    for l in &visited_turn_points {
        if l.0 == location && l.1 == direction {
            visited_twice = true;
            break;
        }
    }

    visited_turn_points.push((location, direction));

    return (visited_turn_points, visited_twice);
}

fn simulate_guard(mut contents: Vec<Vec<char>>, mut current_line_idx: i64, mut current_entry_idx: i64, mut guard_facing: Direction, update_contents: bool) -> (Vec<Vec<char>>, Vec<(VisitedLocation, Direction)>, bool) {
    let mut guard_left = false;
    let mut next_line_idx: i64;
    let mut next_entry_idx: i64;
    let mut reached_obstacle: bool;
    let mut visited_locations: Vec<(VisitedLocation, Direction)> = vec![];
    let mut visited_twice: bool = false;
   
    // continue until the guard left
    while !guard_left {
     
        next_line_idx = current_line_idx as i64;
        next_entry_idx = current_entry_idx;

        match guard_facing {
            Direction::N => {
                next_line_idx += -1;
                next_entry_idx += 0;
            }
            Direction::E => {
                next_line_idx += 0;
                next_entry_idx += 1;
            }
            Direction::S => {
                next_line_idx += 1;
                next_entry_idx += 0;
            }
            Direction::W => {
                next_line_idx += 0;
                next_entry_idx += -1;
            }
            Direction::None => panic!("This should never happen."),
        }
        (guard_facing, guard_left, reached_obstacle) = guard_move(
            &contents,
            next_line_idx,
            next_entry_idx,
            guard_facing,
        );

        // make sure that we are not out-of-bounds
        if current_line_idx >= 0 && current_entry_idx >= 0 && contents.len() > current_line_idx as usize && contents[0].len() > current_entry_idx as usize {

            if update_contents {
                contents[current_line_idx as usize][current_entry_idx as usize] = 'X';
            }

            // create location for part 2 if the guard turns
            if reached_obstacle {
                (visited_locations, visited_twice) = create_turn_point(
                    current_line_idx as i64,
                    current_entry_idx,
                    guard_facing.clone(),
                    visited_locations,
                );
            }
            // visited_twice can only be true when a loop was found. This shouldn't happen in part
            // 1, so it is ok to break the loop when this happens for part 1 and necessary for part
            // 2.
            if visited_twice {
                break;
            }
        }


        if !reached_obstacle {
            current_entry_idx = next_entry_idx;
            current_line_idx = next_line_idx;
        }

        // println!("guard is facing: {:?}, entry {}, line {}", guard_facing, current_entry_idx, current_line_idx)
    }
    return (contents, visited_locations, visited_twice);
}

fn create_obstacle(mut contents: Vec<Vec<char>>, location: &VisitedLocation) -> Vec<Vec<char>> {
    // make sure that we are not out-of-bounds
    if location.line_idx >= 0 && location.entry_idx >= 0 && contents.len() > location.line_idx as usize && contents[0].len() > location.entry_idx as usize {
    contents[location.line_idx as usize][location.entry_idx as usize] = '#';
    }

    return contents;
}

fn main() {
    let contents: Vec<String> = read_to_string(format!(
        "{}/git_repos/adventofcode/2024/day_06/rust/src/input.txt",
        home_dir().unwrap().display().to_string()
    ))
    .unwrap()
    .split("\n")
    .map(|s| s.to_string())
    //filter out empty rows
    .filter(|s| !s.trim().is_empty())
    .collect();

    // create a matrix out of the input
    let contents_map: Vec<Vec<char>> = contents.clone().into_iter().map(|s|s.chars().collect()).collect();

    let mut guard_facing = Direction::None;
    let mut current_line_idx = 0;
    let mut current_entry_idx = 0;

    let mut number_positions = 0;

    let mut initial_location: VisitedLocation = VisitedLocation {
        line_idx: 0,
        entry_idx: 0,
    };

    // first search for position of the guard and the direction he is facing
    for facing_direction in vec!["^", "v", ">", "<"] {
        for line_number in 0..contents_map.len() {
            match contents[line_number].find(facing_direction) {
                Some(entry_number) => {
                    current_line_idx = line_number;
                    current_entry_idx = entry_number as i64;
                    initial_location = VisitedLocation {
                        line_idx: current_line_idx as i64,
                        entry_idx: current_entry_idx,
                    };
                    match facing_direction {
                        "^" => guard_facing = Direction::N,
                        "v" => guard_facing = Direction::S,
                        ">" => guard_facing = Direction::E,
                        "<" => guard_facing = Direction::W,
                        _ => println!("No direction found."),
                    }
                }
                None => continue,
            }
        }
    }

    // stop, if no initial direction was not found:
    match guard_facing {
        Direction::None => panic!("No initial direction was found."),
        _ => println!("Initial_location: {:?}, initial direction: {:?}", initial_location, guard_facing),
    }

    // at this point, the initial direction, entry and line was found and saved
    // visited_locations necessary for part 2
    let (modified_contents, _, _) = simulate_guard(contents_map.clone(), current_line_idx as i64, current_entry_idx, guard_facing.clone(), true);

    let mut visited_turn_points: Vec<VisitedLocation> = vec![];

    for line_number in 0..modified_contents.len() {
        for entry_number in 0..modified_contents[line_number].len() {
            if modified_contents[line_number][entry_number] == 'X' {
                number_positions += 1;
                visited_turn_points.push(VisitedLocation { line_idx: line_number as i64, entry_idx: entry_number as i64 });
              }
        }
    }

    println!("Result for part1: {number_positions}");

    // part 2 idea:
    // save all visited locations with direction into which the guard faces
    // go through all locations and put an obstacle to the next field in the direction, the guard
    // faces (but not the starting place)
    // after the obstacle was created, run the guard_move function again and additionally save all
    // visited locations with facing direction. If one location was visited twice with the same
    // direction, then a loop was created. If the guard eventually leaves the matrix, no loop was
    // created.

    let mut number_loop_possibilities = 0;

    // go through all locations visited in the standard run without additional obstacles
    for location in &visited_turn_points {
      
        match &location {
            // deference and compare to initial_location so that the variable is used
            // if only match initial_location ist used, rust interprets this as a new variable in
            // the match scope
            l if **l != initial_location => {
                // we are not at the starting position --> create obstacle at this point
                let modified_contents = create_obstacle(contents_map.clone(), &l);

                let (_, _, visited_twice) = simulate_guard(modified_contents, current_line_idx as i64, current_entry_idx, guard_facing.clone(), false);
                if visited_twice {
                    number_loop_possibilities += 1;
                }
            }
            _ => continue,
        }
    }

    println!("Result for part2: {number_loop_possibilities}");

}
