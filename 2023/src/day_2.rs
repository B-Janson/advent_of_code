use std::{io::{Lines, BufReader}, fs::File};

use advent_of_code_2023::util::util::read_lines;
use regex::Regex;

const INPUT: &str = "input/day_2.txt";

const RED_CUBES: usize = 12;
const GREEN_CUBES: usize = 13;
const BLUE_CUBES: usize = 14;

fn part_one(game_number_re: &Regex, red_cube_count_re: &Regex, green_cube_count_re: &Regex, blue_cube_count_re: &Regex) {
    let mut lines: Lines<BufReader<File>> = read_lines(INPUT).unwrap();
    let mut sum = 0;
    for line in lines {
        let unwrapped_line = line.unwrap();
        let mut valid_game = true;
        let mut game_information = unwrapped_line.split(":");

        // Get game number
        let game_number = game_information.next().unwrap();
        println!("{}", game_number);
        let number = game_number_re.captures(game_number);
        let game = number.unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
        println!("{}", game);

        // Get cube information
        let cube_information = game_information.next().unwrap().trim();
        println!("{}", cube_information); 
        println!("red");
        for red_counts in red_cube_count_re.captures_iter(cube_information) {
            let red = red_counts.get(1).unwrap().as_str().parse::<usize>().unwrap();
            println!("{}", red);
            if red > RED_CUBES {
                valid_game = false;
                break;
            }
        }

        println!("green");
        for green_counts in green_cube_count_re.captures_iter(cube_information) {
            let green = green_counts.get(1).unwrap().as_str().parse::<usize>().unwrap();
            println!("{}", green);
            if green > GREEN_CUBES {
                valid_game = false;
                break;
            }
        }

        println!("blue");
        for blue_counts in blue_cube_count_re.captures_iter(cube_information) {
            let blue = blue_counts.get(1).unwrap().as_str().parse::<usize>().unwrap();
            println!("{}", blue);
            if blue > BLUE_CUBES {
                valid_game = false;
                break;
            }
        }

        if valid_game {
            sum += game;
        }

        println!("{}", valid_game);
    }

    println!("part 1 sum: {}", sum);
}

fn part_two(game_number_re: &Regex, red_cube_count_re: &Regex, green_cube_count_re: &Regex, blue_cube_count_re: &Regex) {
    let mut lines: Lines<BufReader<File>> = read_lines(INPUT).unwrap();
    let mut sum: u32 = 0;
    for line in lines {
        let unwrapped_line = line.unwrap();
        let mut valid_game = true;
        let mut game_information = unwrapped_line.split(":");

        // Get game number
        let game_number = game_information.next().unwrap();
        println!("{}", game_number);
        let number = game_number_re.captures(game_number);
        let game = number.unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
        println!("{}", game);

        // Get cube information
        let cube_information = game_information.next().unwrap().trim();
        println!("{}", cube_information); 
        println!("red");
        let mut highest_red = 0;
        for red_counts in red_cube_count_re.captures_iter(cube_information) {
            let red = red_counts.get(1).unwrap().as_str().parse::<usize>().unwrap();
            println!("{}", red);
            if red > highest_red {
                highest_red = red;
            }
        }

        println!("green");
        let mut highest_green = 0;
        for green_counts in green_cube_count_re.captures_iter(cube_information) {
            let green = green_counts.get(1).unwrap().as_str().parse::<usize>().unwrap();
            println!("{}", green);
            if green > highest_green {
                highest_green = green;
            }
        }

        println!("blue");
        let mut highest_blue = 0;
        for blue_counts in blue_cube_count_re.captures_iter(cube_information) {
            let blue = blue_counts.get(1).unwrap().as_str().parse::<usize>().unwrap();
            println!("{}", blue);
            if blue > highest_blue {
                highest_blue = blue;
            }
        }

        let game_power: u32 = (highest_red * highest_green * highest_blue).try_into().unwrap();
        sum += game_power;
    }

    println!("part 2 sum: {}", sum);
}


fn main() {
    let game_number_re = Regex::new(r"Game ([0-9]*)").unwrap();
    let red_cube_count_re = Regex::new(r"(\d+) red").unwrap();
    let green_cube_count_re = Regex::new(r"(\d+) green").unwrap();
    let blue_cube_count_re = Regex::new(r"(\d+) blue").unwrap();
    

    part_one(&game_number_re, &red_cube_count_re, &green_cube_count_re, &blue_cube_count_re);
    part_two(&game_number_re, &red_cube_count_re, &green_cube_count_re, &blue_cube_count_re);
}
