use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;

// THIS IS REALLY BAD CODE
// Please look away
fn main() {
    let filename = "src/data.txt";
    first(filename);
    second(filename);
}

fn first(filename: &str) {
    let gamma_res: [i32; 12] = read_file(filename);
    let res = binary_array_to_int_epsilon(gamma_res);
    println!("{}", res[0] * res[1])
}

fn second(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut vec: Vec<String> = Vec::new();
    for line in reader.lines() {
        vec.push(line.unwrap());
    }
    let mut final_gamma: Vec<String> = vec.clone();
    for i in 0..12 {
        let match_char = determine_most_common(&final_gamma, i, false);
        final_gamma = final_gamma
            .into_iter()
            .filter(|res| res.chars().nth(i as usize).unwrap() == match_char)
            .collect();
        if final_gamma.len() == 1 {
            break;
        }
    }

    let mut final_epsilon: Vec<String> = vec.clone();
    for i in 0..12 {
        let match_char = determine_most_common(&final_epsilon, i, true);
        final_epsilon = final_epsilon
            .into_iter()
            .filter(|res| res.chars().nth(i as usize).unwrap() == match_char)
            .collect();
        if final_epsilon.len() == 1 {
            break;
        }
    }
    let dec_gamma = binary_string_to_int(&final_gamma[0]);
    let dec_epsilon = binary_string_to_int(&final_epsilon[0]);
    println!("{}", dec_gamma * dec_epsilon);
}

fn binary_array_to_int_epsilon(gamma_res: [i32; 12]) -> [i32; 2] {
    let mut gamma_val: i32 = 0;
    let mut epsilon_val: i32 = 0;
    let base: i32 = 2;
    for (index, res) in gamma_res.into_iter().enumerate() {
        if res == 1 {
            gamma_val += base.pow(((gamma_res.len() - index - 1) as usize) as u32) as i32;
        } else {
            epsilon_val += base.pow(((gamma_res.len() - index - 1) as usize) as u32) as i32;
        }
    }
    return [gamma_val, epsilon_val];
}

fn binary_string_to_int(gamma_res: &str) -> i32 {
    let mut final_val: i32 = 0;
    let base: i32 = 2;
    for (index, res) in gamma_res.chars().enumerate() {
        if res == '1' {
            final_val += base.pow(((gamma_res.len() - index - 1) as usize) as u32) as i32;
        }
    }
    return final_val;
}

fn determine_most_common(list: &Vec<String>, index: i32, swp: bool) -> char {
    let mut rtn = 0;
    for i in list {
        if i.chars().nth(index as usize).unwrap() == '1' {
            rtn += 1;
        }
    }
    if swp {
        rtn = list.len() - rtn;
    }
    let decision = (rtn as f32 / (list.len()) as f32) as f32;
    if decision >= 0.5 && !swp {
        return '1';
    } else if !swp {
        return '0';
    } else if decision > 0.5 {
        return '1';
    } else {
        return '0';
    };
}

fn read_file(filename: &str) -> [i32; 12] {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // array for counting the occurance of bits
    let mut arr: [f32; 12] = [0.0; 12];
    let mut row_count: f32 = 0.0;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let tmp = line.unwrap();
        let split: Chars = tmp.chars();
        for (index, char) in split.enumerate() {
            if char == '1' {
                arr[index] += 1.0;
            }
        }
        row_count += 1.0;
    }

    let mut gamma_res: [i32; 12] = [0; 12];
    for (index, res) in arr.into_iter().enumerate() {
        if res / row_count >= 0.5 {
            gamma_res[index] = 1;
        } else {
            gamma_res[index] = 0;
        }
    }
    return gamma_res;
}
