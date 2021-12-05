use std::fs::File;
use std::io::{BufRead, BufReader};
use std::panic;
use crate::bingo_board::BingoBoard;
mod bingo_board;

fn main() {
    first();
}

// Figure out which bingo board wins first and last
fn first(){
    let filename = "src/data.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut board_calls :Vec<i32> = Vec::new();
    let mut new_board : Vec<Vec<i32>> = Vec::new();
    let mut board_list : Vec<BingoBoard> = Vec::new();
    for line in reader.lines() {
        if board_calls.len() == 0 {
            board_calls = line.unwrap().split(",").map(|x| x.parse().unwrap()).collect();
        }
        else{
            let str = line.unwrap();
            if str == "" && new_board.len() != 0{
                board_list.push(BingoBoard::new(new_board, board_list.len() as i32));
                new_board = Vec::new();
            }
            else if str != "" {
                let new_row : Vec<i32> = str.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
                new_board.push(new_row);
            }
        }
    }
    let score = check_boards(&board_calls, &mut board_list);
    println!("{}",score);
    let last_score = find_last_board(&board_calls, &mut board_list);
    println!("{}", last_score);
}  

fn check_boards(board_calls : &Vec<i32>, board_list : &mut Vec<BingoBoard>) -> i32 {
    for board_call in board_calls{
        for board in board_list.iter_mut(){
            board.check_number(*board_call);
            if board.check_board(){
                return board.get_score() * board_call;
            }
        }
    }
    panic!();
}

fn find_last_board(board_calls : &Vec<i32>, board_list : &mut Vec<BingoBoard>) -> i32 {
    let mut called_boards : Vec<i32> = Vec::new();
    for board_call in board_calls{
        for board in board_list.iter_mut(){
            board.check_number(*board_call);
            if board.check_board() && !called_boards.contains(&board.id){
                called_boards.push(board.id);
            }
        }
        if called_boards.len() == board_list.len(){
            let last_board = board_list.into_iter().filter(|board| board.id == *called_boards.last().unwrap());
            return last_board.last().unwrap().get_score() * board_call;
        }
    }
    panic!();
}