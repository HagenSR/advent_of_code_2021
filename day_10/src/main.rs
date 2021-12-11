use std::{fs, panic};

fn main() {
    let filename = "src/data/data.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Collect file into 2d array
    let map: Vec<&str> = contents
        .split("\n")
        .filter(|l| !l.is_empty() && !l.trim().is_empty())
        .collect();

    // Determine what the local mins are, only considering cardinal directions
    let good_lines = first(&map);
    second(good_lines);
}

fn first<'a>(map: &'a Vec<&str>) -> Vec<&'a str> {
    let mut rtn: Vec<&str> = Vec::new();
    let mut bad_chars : Vec<char> = Vec::new();
    let mut good = true;
    for line in map {
        good = true;
        let mut stack: Vec<char> = Vec::new();
        for char in line.chars() {
            if ['(', '[', '{', '<'].contains(&char) {
                stack.push(char);
            } else {
                let popped = stack.pop().unwrap();
                if popped == '(' && char != ')'  {
                    good = false;
                    bad_chars.push(char);
                    break;
                }
                else if popped == '[' && char != ']'  {
                    good = false;
                    bad_chars.push(char);
                    break;
                }
                else if popped == '{' && char != '}'  {
                    good = false;
                    bad_chars.push(char);
                    break;
                }
                else if popped == '<' && char != '>'  {
                    good = false;
                    bad_chars.push(char);
                    break;
                }
            }
        }
        if good {
            rtn.push(line);
        }
    }
    println!("The score for the file was {}", get_score(bad_chars));
    return rtn;
}

fn second(lines :Vec<&str>){
    let mut scores : Vec<i128> = Vec::new();
    for line in lines{
        let mut unmatched : Vec<char> = Vec::new();
        for char in line.chars(){
            if ['(', '[', '{', '<'].contains(&char) {
                unmatched.push(char);
            } else {
                let popped = unmatched.pop().unwrap();
            }
        }
        scores.push(get_score_2(&mut unmatched));
    }
    scores.sort();
    let middle = scores.len() / 2;
    println!("The middle score is {} ", scores[middle]);

}

fn get_score(bad_chars : Vec<char>) -> i32 {
    let mut score = 0;
    for char in bad_chars{
        match char {
            ')' => score += 3,
            ']' => score += 57,
            '}' => score += 1197,
            '>' => score += 25137,
            _ => score += 0
        }
    }
    return score;
}

fn get_score_2(unmatched :&mut Vec<char>) -> i128{
    let mut score : i128 = 0;
    unmatched.reverse();
    for char in unmatched{
        score *= 5;
        match char {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => score += 0
        }
    }
    return score;
}
