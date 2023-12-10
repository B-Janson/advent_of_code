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
    }
    println!("Part one: {sum}");
}

fn part_two() {
    let lines: Lines<BufReader<File>> = read_lines(INPUT).unwrap();
    let mut card_map: HashMap<usize, usize> = HashMap::new();
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


        let card_copies_for_hand = *card_map.get(&row).unwrap_or(&0);
        sum += card_copies_for_hand + 1;

        
        for blah in 0..card_copies_for_hand + 1 {
            for i in 0..overlap_size {
                let current = card_map.get(&(row + i + 1)).unwrap_or(&0);
                card_map.insert(row + i + 1, current + 1);
            }
        }
        

        println!("overlap_size:{overlap_size} sum:{sum} card_copies_for_hand:{card_copies_for_hand}, {:?}", card_map);
    }
    println!("Part two: {sum}");
}

fn main() {
    part_one();
    part_two();
}
