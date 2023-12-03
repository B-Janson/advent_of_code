use std::{fs::File, path::Path, io::{self, BufRead}};
use self::NumberAsString::*;

const INPUT: &str = "input/day_1.txt";

const ZERO_DIGIT: u8 = b'0';
const NINE_DIGIT: u8 = b'9';

#[derive(Debug, Clone, Copy)]
pub enum NumberAsString {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl NumberAsString {
    fn as_str(&self) -> &str {
        match *self {
            NumberAsString::Zero => "zero",
            NumberAsString::One => "one",
            NumberAsString::Two => "two",
            NumberAsString::Three => "three",
            NumberAsString::Four => "four",
            NumberAsString::Five => "five",
            NumberAsString::Six => "six",
            NumberAsString::Seven => "seven",
            NumberAsString::Eight => "eight",
            NumberAsString::Nine => "nine",
        }
    }

    fn iterator() -> impl Iterator<Item = NumberAsString> {
        static NUMBERS: [NumberAsString; 10] = [Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine];
        NUMBERS.iter().copied()
    }
}

fn main() {
    if let Ok(lines) = read_lines(INPUT) {
        let mut sum: usize = 0;
        for line in lines {
            if let Ok(calibration_value) = line {
                println!("{}", calibration_value);
                let calibration_value_bytes = calibration_value.as_bytes();

                let mut left: u8 = 0;
                for left_idx in 0..calibration_value.len() {
                    let remaining_string = &calibration_value[left_idx..];
                    println!("{}", remaining_string);

                    let single_char = calibration_value_bytes[left_idx];
                    if single_char >= ZERO_DIGIT && single_char <= NINE_DIGIT {
                        println!("Found a digit at {}: {}", left_idx, single_char as char);
                        left = single_char - ZERO_DIGIT;
                        break;
                    }
                    
                    for num in NumberAsString::iterator() {
                        if remaining_string.starts_with(num.as_str()) {
                            println!("Found {} at {} ==> {:?}", num.as_str(), left_idx, num as u32);
                            left = num as u8;
                            break;
                        }
                    }

                    if left != 0 {
                        break;
                    }
                }

                let mut right: u8 = 0;
                for right_idx in (0..calibration_value.len()).rev() {
                    let remaining_string = &calibration_value[right_idx..];
                    println!("{}", remaining_string);

                    let single_char = calibration_value_bytes[right_idx];
                    if single_char >= ZERO_DIGIT && single_char <= NINE_DIGIT {
                        println!("Found a digit at {}: {}", right_idx, single_char as char);
                        right = single_char - ZERO_DIGIT;
                        break;
                    }
                    
                    for num in NumberAsString::iterator() {
                        if remaining_string.starts_with(num.as_str()) {
                            println!("Found {} at {} ==> {:?}", num.as_str(), right_idx, num as u32);
                            right = num as u8;
                            break;
                        }
                    }

                    if right != 0 {
                        break;
                    }
                }

                println!("left {} right {}", left, right);
                println!("{}", left * 10 + right);
                let line_number: usize = left as usize * 10 + right as usize;
                sum += line_number;
            }
        }
        println!("{}", sum);
    } else {
        panic!("file not found");
    }
    
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


