use std::fs;

fn main() {
    let filename = "src/data/datasmall.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let split_new_line: Vec<&str> = contents.split(":").collect();
    let code: Vec<&str> = split_new_line[0]
        .split("")
        .filter(|str| *str != "")
        .collect();
    let mut map: Vec<Vec<&str>> = split_new_line[1]
        .split_inclusive("\n")
        .map(|f| {
            f.split_inclusive("")
                .map(|s| s.trim())
                .filter(|l| !l.is_empty() && !l.trim().is_empty())
                .collect::<Vec<&str>>()
        })
        .filter(|vec| vec.len() > 0)
        .collect();
    first(map.clone(), &code);
}

fn first(map: Vec<Vec<&str>>, code: &Vec<&str>) {
    let mut local_map = map.clone();
    let mut flip_void = true;
    for _x in 0..2 {
        pad_map(&mut local_map, flip_void);
        let mut new_map = local_map.clone();
        for i in 1..local_map.len() - 1 {
            for j in 1..local_map[0].len() - 1 {
                let matrix_string = get_matrix_string_around(&local_map, i as i32, j as i32);
                let value = string_to_binary_to_number(&matrix_string);
                new_map[i][j] = code[value as usize];
            }
        }
        if code[0] == "#"{
            flip_void = !flip_void;
        }
        local_map = new_map;
    }
    let occurances = count_pound_occurances(&local_map);
    println!("There were a total of {} pounds", occurances);
}

fn count_pound_occurances(map: &Vec<Vec<&str>>) -> i32 {
    let mut count = 0;
    for vec in map {
        for st in vec {
            if *st == "#" {
                count += 1;
            }
        }
    }
    return count;
}

fn get_matrix_string_around(map: &Vec<Vec<&str>>, i: i32, j: i32) -> String {
    let mut rtn: String = String::new();
    for x in -1i32..2 {
        for y in -1..2 {
            rtn += map[(i + x) as usize][(y + j) as usize].clone();
        }
    }
    return rtn;
}

fn string_to_binary_to_number(string: &str) -> i32 {
    let mut final_val: i32 = 0;
    let base: i32 = 2;
    for (index, res) in string.chars().enumerate() {
        if res == '#' {
            final_val += base.pow(((string.len() - index - 1) as usize) as u32) as i32;
        }
    }
    return final_val;
}

fn pad_map(map: &mut Vec<Vec<&str>>, fill_empty: bool) {
    let mut blank_vec = Vec::new();
    for _i in 0..map.len() {
        if fill_empty {
            blank_vec.push(".");
        } else {
            blank_vec.push("#");
        }
    }
    for _i in 0..2 {
        map.insert(0, blank_vec.clone());
        map.push(blank_vec.clone());
    }

    for vec in map {
        for _i in 0..2 {
            if fill_empty{
                vec.insert(0, ".");
                vec.push(".");
            }else{
                vec.insert(0, "#");
                vec.push("#");
            }

        }
    }
}

fn blank_map<'a>(height: usize, width: usize) -> Vec<Vec<&'a str>> {
    let mut return_map: Vec<Vec<&'a str>> = Vec::new();
    let mut new_vec: Vec<&'a str> = Vec::new();
    for _j in 0..width {
        new_vec.push(".");
    }
    for _i in 0..height {
        return_map.push(new_vec.clone());
    }
    return return_map;
}
