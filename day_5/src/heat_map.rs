use std::collections::HashMap;

pub struct HeatMap{
    pub map: HashMap<(i32,i32), i32>
}

impl HeatMap {
    pub fn new() -> HeatMap {
        HeatMap {
            map: HashMap::new()
        }
    } 

    pub fn add_line(&mut self, mut begin : (i32, i32), end : (i32,i32)){
        while begin.0 != end.0 || begin.1 != end.1 {
            let counter = self.map.entry(begin).or_insert(0);
            *counter += 1;
            
            if begin.0 < end.0{
                begin.0 += 1;
            }else if begin.0 > end.0{
                begin.0 -= 1;
            }

            if begin.1 < end.1 {
                begin.1 += 1;
            }else if begin.1 > end.1{
                begin.1 -= 1;
            }
        }
        let counter = self.map.entry(end).or_insert(0);
        *counter += 1;
    }

    pub fn get_score(&mut self) -> i32{
        let mut rtn_score = 0;
        for (_key, value) in self.map.iter(){
            if *value > 1 {
                rtn_score += 1
            }
        }
        return rtn_score;
    }

}