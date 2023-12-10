use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Lines},
};

use advent_of_code_2023::util::util::read_lines;

const INPUT: &str = "input/day_4.txt";

fn part_one() {
    let lines: Lines<BufReader<File>> = read_lines(INPUT).unwrap();
    let mut sum: usize = 0;

    for (row, line) in lines.enumerate() {
        let unwrapped_line = line.unwrap();
        let card_input_idx = unwrapped_line.find(':').unwrap();
        let line_input = &unwrapped_line[card_input_idx+1..].trim();
        println!("{}", line_input);

        let mut input_split = line_input.split("|");
        let winning_input: HashSet<usize> = input_split.next().unwrap().split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
        let game_input: HashSet<usize> = input_split.next().unwrap().split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
        let overlap: std::collections::hash_set::Intersection<'_, usize, std::collections::hash_map::RandomState> = winning_input.intersection(&game_input);
        let overlap_size = overlap.clone().count();
        let hand_score = if overlap_size > 0 {
            2usize.pow(overlap_size as u32 - 1)
        } else {
            0
        };

        sum += hand_score;
        

        println!("win:{:?} hand:{:?}, intersection:{:?}, score:{}", winning_input, game_input, overlap, hand_score);
    }
    println!("{sum}");
}

fn part_two() {
    
}

fn main() {
    part_one();
    part_two();
}
