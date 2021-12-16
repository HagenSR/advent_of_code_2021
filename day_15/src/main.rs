use std::fs;


fn main() {
    let filename = "src/data/datasmall.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Collect file into 2d array
    let mut map: Vec<Vec<i32>> = contents
        .split_inclusive("\n")
        .map(|f| {
            f.split_inclusive("")
                .filter(|l| !l.is_empty() && !l.trim().is_empty())
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // Determine what the local mins are, only considering cardinal directions
    first(&mut map);

    // Determine "basins" around local mins, which are areas that don't have height nine
    // Print product of top 3 basin sizes
    //second(&map, local_mins);
}

fn first(map: &mut Vec<Vec<i32>>) {
    let mut to_check: Vec<i32> = Vec::new();
    let weights : Vec<((i32, i32), i32)> = Vec::new();
    for i in map{
        for j in map{
            weights[]

        }
    }
}

fn path_find(map: &mut Vec<Vec<i32>>) {
    let mut to_check: Vec<(i32, i32)> = Vec::new();
    to_check.push((0,0));
    let end_pos =  (map.len() as i32, map.last().unwrap().len() as i32);
    while *to_check.last().unwrap() != end_pos{

    }

}
