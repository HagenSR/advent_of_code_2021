use std::{fs, collections::BTreeMap};

struct Instruction {
    state: bool,
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
    z_start: i32,
    z_end: i32,
}

fn main() {
    let filename = "src/data/data.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let split_new_line: Vec<&str> = contents.split("\n").collect();
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in split_new_line {
        let split_space: Vec<&str> = line.split(" ").collect();
        let mut on_or_off = false;
        if split_space[0] == "on" {
            on_or_off = true;
        }
        let mut dimensions: Vec<Vec<i32>> = split_space[1]
            .replace("x", "")
            .replace("=", "")
            .replace("y", "")
            .replace("z", "")
            .split(",")
            .map(|res| res.split("..").map(|f| f.parse::<i32>().unwrap()).collect())
            .collect();

        for vec in dimensions.iter_mut(){
            if vec[0] < -50{
                vec[0] = -50;
            }else if vec[0] > 50{
                vec[0] = 50;
            }

            if vec[1] < -50{
                vec[1] = -51;
            }else if vec[1] > 50{
                vec[1] = 49;
            }
        }
        let instruct: Instruction = Instruction {
            state: on_or_off,
            x_start: dimensions[0][0],
            x_end: dimensions[0][1] + 1,
            y_start: dimensions[1][0],
            y_end: dimensions[1][1] + 1,
            z_start: dimensions[2][0],
            z_end: dimensions[2][1] + 1,
        };
        instructions.push(instruct);
    }

    first(instructions);
}

fn first(instructions : Vec<Instruction>){
    let mut flipped : BTreeMap<(i32, i32, i32), bool> = BTreeMap::new();
    let mut counter = 0;
    for instruct in instructions{
        for x in instruct.x_start..instruct.x_end{
            for y in instruct.y_start..instruct.y_end{
                for z in instruct.z_start..instruct.z_end{

                    let value = flipped.entry((x,y,z)).or_insert(instruct.state);
                    *value = instruct.state;
                }
            }
        }
        counter+=1;
        println!("Instruct {}", counter);
    }
    println!("There were {} on values", count_on_values(flipped));
}

fn count_on_values(flipped : BTreeMap<(i32, i32, i32), bool>) -> i32 {
    let mut rtn = 0;
    for value in flipped.values(){
        if *value{
            rtn += 1;
        }
    }
    return  rtn;
}
