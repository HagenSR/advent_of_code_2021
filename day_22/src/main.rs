use std::{fs};

struct Instruction {
    state: bool,
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
    z_start: i32,
    z_end: i32,
}

impl Clone for Instruction {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            x_start: self.x_start.clone(),
            x_end: self.x_end.clone(),
            y_start: self.y_start.clone(),
            y_end: self.y_end.clone(),
            z_start: self.z_start.clone(),
            z_end: self.z_end.clone(),
        }
    }
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        return self.state == other.state
            && self.x_start == other.x_start
            && self.x_end == other.x_end
            && self.y_start == other.y_start
            && self.y_end == other.y_end
            && self.z_start == other.z_start
            && self.z_end == other.z_end;
    }
}

impl Eq for Instruction {}

fn main() {
    let filename = "src/data/datamed.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let split_new_line: Vec<&str> = contents.split("\n").collect();
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in split_new_line {
        let split_space: Vec<&str> = line.split(" ").collect();
        let mut on_or_off = false;
        if split_space[0] == "on" {
            on_or_off = true;
        }
        let dimensions: Vec<Vec<i32>> = split_space[1]
            .replace("x", "")
            .replace("=", "")
            .replace("y", "")
            .replace("z", "")
            .split(",")
            .map(|res| res.split("..").map(|f| f.parse::<i32>().unwrap()).collect())
            .collect();

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

fn first(instructions: Vec<Instruction>) {
    let mut previously_calculated_instructions: Vec<Instruction> = Vec::new();
    for instruct in instructions.iter() {
        let mut new_intersections: Vec<Instruction> = previously_calculated_instructions.clone();
        for prev_instruct in previously_calculated_instructions.iter_mut() {
            if determine_colision(prev_instruct, instruct){
                let overlap = return_overlap(instruct, &prev_instruct);
                if !new_intersections.contains(&overlap){
                    new_intersections.push(overlap.clone());
                }
            }
        }
        previously_calculated_instructions = new_intersections;
        if instruct.state {
            previously_calculated_instructions.push(instruct.clone());
        }
    }
    println!(
        "There were {} on values",
        determine_final_area(previously_calculated_instructions)
    );
}

fn determine_colision(a: &Instruction, b: &Instruction) -> bool {
    return (a.x_start <= b.x_end && a.x_end >= b.x_start)
        && (a.y_start <= b.y_end && a.y_end >= b.y_start)
        && (a.z_start <= b.z_end && a.z_end >= b.z_start);
}

fn determine_final_area(list: Vec<Instruction>) -> f64 {
    let mut area = 0.0;
    for instruction in list {
        let tmp = calculate_area(&instruction);
        if instruction.state {
            area += tmp;
        } else {
            area -= tmp;
        }
    }
    return area;
}

fn return_overlap(a: &Instruction, b: &Instruction) -> Instruction {
    let x_start = a.x_start.max(b.x_start);
    let x_end = a.x_end.min(b.x_end);
    let y_start = a.y_start.max(b.y_start);
    let y_end = a.y_end.min(b.y_end) ;
    let z_start = a.z_start.max(b.z_start);
    let z_end = a.z_end.min(b.z_end);
    return Instruction {
        state: return_new_state(a.state, b.state),
        x_start,
        x_end,
        y_start,
        y_end,
        z_start,
        z_end,
    };
}

fn return_new_state(state_a : bool, state_b : bool) -> bool {
    if state_a == state_b {
        return !state_a
    }else if state_a && !state_b{
        return true;
    }else{
        return false;
    }
}

fn calculate_area(instruct: &Instruction) -> f64 {
    return (instruct.x_end as f64 - instruct.x_start as f64).abs()
        * (instruct.z_end as f64 - instruct.z_start as f64).abs()
        * (instruct.y_end as f64 - instruct.y_start as f64).abs();
}
