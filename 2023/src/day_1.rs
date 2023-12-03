use advent_of_code_2023::util::util::read_lines;
use self::NumberAsString::*;

const INPUT: &str = "input/day_1.txt";

const ZERO_DIGIT: u8 = b'0';
const NINE_DIGIT: u8 = b'9';

const DEBUG_PRINT: bool = false;

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
                if DEBUG_PRINT {println!("{}", calibration_value);}
                let calibration_value_bytes = calibration_value.as_bytes();

                let mut left: u8 = 0;
                for left_idx in 0..calibration_value.len() {
                    let remaining_string = &calibration_value[left_idx..];
                    if DEBUG_PRINT {println!("{}", remaining_string);}

                    let single_char = calibration_value_bytes[left_idx];
                    if single_char >= ZERO_DIGIT && single_char <= NINE_DIGIT {
                        if DEBUG_PRINT {println!("Found a digit at {}: {}", left_idx, single_char as char);}
                        left = single_char - ZERO_DIGIT;
                        break;
                    }
                    
                    for num in NumberAsString::iterator() {
                        if remaining_string.starts_with(num.as_str()) {
                            if DEBUG_PRINT {println!("Found {} at {} ==> {:?}", num.as_str(), left_idx, num as u32);}
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
                    if DEBUG_PRINT {println!("{}", remaining_string);}

                    let single_char = calibration_value_bytes[right_idx];
                    if single_char >= ZERO_DIGIT && single_char <= NINE_DIGIT {
                        if DEBUG_PRINT {println!("Found a digit at {}: {}", right_idx, single_char as char);}
                        right = single_char - ZERO_DIGIT;
                        break;
                    }
                    
                    for num in NumberAsString::iterator() {
                        if remaining_string.starts_with(num.as_str()) {
                            if DEBUG_PRINT {println!("Found {} at {} ==> {:?}", num.as_str(), right_idx, num as u32);}
                            right = num as u8;
                            break;
                        }
                    }

                    if right != 0 {
                        break;
                    }
                }

                if DEBUG_PRINT {println!("left {} right {}", left, right);}
                if DEBUG_PRINT {println!("{}", left * 10 + right);}
                let line_number: usize = left as usize * 10 + right as usize;
                sum += line_number;
            }
        }
        println!("{}", sum);
    } else {
        panic!("file not found");
    }
    
}




