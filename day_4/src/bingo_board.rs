pub struct BingoBoard{
    pub board: Vec<Vec<i32>>,
    pub id : i32
}

impl BingoBoard {
    pub fn new(bd : Vec<Vec<i32>>, new_id : i32) -> BingoBoard {
        BingoBoard {
            board: bd,
            id : new_id
        }
    } 

    pub fn check_number(&mut self, num : i32){
        for row_index in 0..5{
            for col_index in 0..5{
                if self.board[row_index][col_index] == num{
                    self.board[row_index][col_index] = -1;
                    break;
                }
            }
        }
    }

    pub fn check_board(&mut self) -> bool{
        for row in &self.board{
            let mut row_good = true;
            for col_val in row{
                if *col_val != -1{
                    row_good = false;
                    break;
                }
            }
            if row_good{
                return true;
            }
        }
        for index in 0..5{
            let mut col_good = true;
            for row in &self.board{
                if row[index] != -1{
                    col_good = false;
                    break;
                }
            }
            if col_good{
                return true;
            }
        }
        let mut diagnol_negative = true;
        let mut diagnol_positive = true;
        for index in 0..5{
            if self.board[index][index] != -1{
                diagnol_negative = false;
            }
            if self.board[4 - index][4-index] != -1{
                diagnol_positive = false;
            }
        }

        return diagnol_negative || diagnol_positive;
    }

    pub fn get_score(&mut self) -> i32{
        let mut rtn_score = 0;
        for row in &self.board{
            for col_val in row{
                if *col_val != -1{
                    rtn_score += *col_val;
                }
            }
        }
        return rtn_score;
    }


}
impl PartialEq for BingoBoard{
    fn eq(&self, other: &Self) -> bool{
        return self.id == other.id;
    }
}


impl Eq for BingoBoard{
}