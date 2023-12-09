use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Lines},
};

use advent_of_code_2023::util::util::read_lines;

const INPUT: &str = "input/day_3.txt";

fn part_one() {
    let symbols: HashSet<char> = vec!['_', '%', '-', '*', '$', '@', '&', '/', '+', '=', '#']
        .into_iter()
        .collect();
    let numbers_as_chars = '0'..='9';

    let mut sum: usize = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut symbol_locations: HashSet<(usize, usize)> = HashSet::new();
    let mut numbers: HashMap<(usize, usize), usize> = HashMap::new();

    let lines: Lines<BufReader<File>> = read_lines(INPUT).unwrap();

    for (row, line) in lines.enumerate() {
        let unwrapped_line = line.unwrap();
        let mut row_input: Vec<char> = Vec::new();
        for (col, c) in unwrapped_line.chars().enumerate() {
            row_input.push(c);
            if symbols.contains(&c) {
                symbol_locations.insert((row, col));
            }

            if numbers_as_chars.contains(&c) {
                if col == 0 || !numbers_as_chars.contains(row_input.get(col - 1).unwrap()) {
                    let end_num_idx =
                        &unwrapped_line[col..].find(|c: char| symbols.contains(&c) || c == '.');
                    let full_number: &str;
                    if let Some(num_idx) = end_num_idx {
                        full_number = &unwrapped_line[col..col + num_idx];
                    } else {
                        full_number = &unwrapped_line[col..];
                    }

                    numbers.insert((row, col), full_number.parse::<usize>().unwrap());
                }
            }
        }
        grid.push(row_input);
    }

    let surrounding: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (grid_location, val) in numbers {
        for col_iter in 0..num_digits(val) {
            let new_grid_location = (grid_location.0, grid_location.1 + col_iter);

            let mut skip: HashSet<usize> = HashSet::new();
            if new_grid_location.0 == 0 {
                skip.insert(0);
                skip.insert(1);
                skip.insert(2);
            }

            if new_grid_location.0 == grid.len() - 1 {
                skip.insert(6);
                skip.insert(7);
                skip.insert(8);
            }

            if new_grid_location.1 == 0 {
                skip.insert(0);
                skip.insert(3);
                skip.insert(6);
            }

            if new_grid_location.1 == grid[new_grid_location.1].len() - 1 {
                skip.insert(2);
                skip.insert(5);
                skip.insert(8);
            }

            // println!("grid_location: {:?} --> value:{} --> skip: {:?}", new_grid_location, val, skip);

            let mut found_symbol = false;

            for (idx, surrounding_cell) in surrounding.iter().enumerate() {
                if !skip.contains(&idx) {
                    let curr_row = surrounding_cell.0 + new_grid_location.0 as i32;
                    let curr_col = surrounding_cell.1 + new_grid_location.1 as i32;
                    // println!("{curr_row}, {curr_col}");
                    let adjusted_idxs = (curr_row as usize, curr_col as usize);
                    if symbol_locations.contains(&adjusted_idxs) {
                        found_symbol = true;
                        // println!("found symbol {:?} for value {} at {:?}", symbol_locations.get(&adjusted_idxs), val, &adjusted_idxs);
                        break;
                    }
                }
            }

            if found_symbol {
                sum += val;
                break;
            }
        }
    }

    println!("Part one: {}", sum);
}

fn part_two() {
    let symbols: HashSet<char> = vec!['*'].into_iter().collect();
    let numbers_as_chars = '0'..='9';

    let mut sum: usize = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut symbol_locations: HashSet<(usize, usize)> = HashSet::new();
    let mut numbers: HashMap<(usize, usize), usize> = HashMap::new();

    let lines: Lines<BufReader<File>> = read_lines(INPUT).unwrap();

    for (row, line) in lines.enumerate() {
        let unwrapped_line = line.unwrap();
        let mut row_input: Vec<char> = Vec::new();
        for (col, c) in unwrapped_line.chars().enumerate() {
            row_input.push(c);
            if symbols.contains(&c) {
                symbol_locations.insert((row, col));
            }

            if numbers_as_chars.contains(&c) {
                if col > 0 && numbers.contains_key(&(row, col - 1)) {
                    numbers.insert((row, col), *numbers.get(&(row, col - 1)).unwrap());
                } else {
                    let end_num_idx =
                        &unwrapped_line[col..].find(|c: char| !numbers_as_chars.contains(&c));
                    let full_number: &str;
                    if let Some(num_idx) = end_num_idx {
                        full_number = &unwrapped_line[col..col + num_idx];
                    } else {
                        full_number = &unwrapped_line[col..];
                    }

                    numbers.insert((row, col), full_number.parse::<usize>().unwrap());
                }
            }
        }
        grid.push(row_input);
    }

    // println!("{:?}", numbers);
    // println!("{:?}", symbol_locations);

    let surrounding: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (star_row, star_col) in symbol_locations {
        let mut skip: HashSet<usize> = HashSet::new();
        if star_row == 0 {
            skip.insert(0);
            skip.insert(1);
            skip.insert(2);
        }

        if star_row == grid.len() - 1 {
            skip.insert(6);
            skip.insert(7);
            skip.insert(8);
        }

        if star_col == 0 {
            skip.insert(0);
            skip.insert(3);
            skip.insert(6);
        }

        if star_col == grid[star_col].len() - 1 {
            skip.insert(2);
            skip.insert(5);
            skip.insert(8);
        }

        println!("grid_location: {:?} --> skip: {:?}", (star_row, star_col), skip);

        let mut found_numbers: Vec<usize> = Vec::new();

        for (idx, surrounding_cell) in surrounding.iter().enumerate() {
            if !skip.contains(&idx) {
                let curr_row = surrounding_cell.0 + star_row as i32;
                let curr_col = surrounding_cell.1 + star_col as i32;
                println!("{curr_row}, {curr_col}");
                let adjusted_idxs = (curr_row as usize, curr_col as usize);
                if numbers.contains_key(&adjusted_idxs) && !found_numbers.contains(numbers.get(&adjusted_idxs).unwrap()) {
                    println!("found value {} at {:?}", numbers.get(&adjusted_idxs).unwrap(), &adjusted_idxs);
                    found_numbers.push(*numbers.get(&adjusted_idxs).unwrap());
                }
                // if symbol_locations.contains(&adjusted_idxs) {
                //     found_symbol = true;
                //     // 
                //     break;
                // }
            }
        }

        if found_numbers.len() == 2 {
            let result = found_numbers.get(0).unwrap() * found_numbers.get(1).unwrap();
            sum += result;
            println!("Multiplyting {:?} * {:?} = {}", found_numbers.get(0), found_numbers.get(1), result);
        }

        // if found_symbol {
        //     sum += val;
        //     break;
        // }
    }

    println!("Part two: {}", sum);
}

fn num_digits(mut num: usize) -> usize {
    let mut count = 0;
    while num != 0 {
        num /= 10;
        count += 1;
    }
    return count;
}

fn main() {
    part_one();
    part_two();
}
