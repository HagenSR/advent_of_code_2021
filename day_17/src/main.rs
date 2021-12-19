use std::fs;

struct PotentialVelocity {
    passes_through: bool,
    initial_x_vel: i32,
    initial_y_vel: i32,
    top_position: (i32, i32),
}
fn main() {
    let filename = "src/data/data.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let split: Vec<&str> = contents.split(" ").collect();
    let x_bounds: Vec<i32> = split[2]
        .replace("x=", "")
        .replace(",", "")
        .split("..")
        .map(|f| f.parse().unwrap())
        .collect();
    let y_bounds: Vec<i32> = split[3]
        .replace("y=", "")
        .split("..")
        .map(|f| f.parse().unwrap())
        .collect();

    let mut highest_position_seen: PotentialVelocity = PotentialVelocity {
        passes_through: false,
        initial_x_vel: -1,
        initial_y_vel: -1,
        top_position: (i32::MIN, i32::MIN),
    };

    let mut valid_positions = 0;

    for i in -2000..2000 {
        for j in -150..150 {
            let rtn = check_passes_in_box(&x_bounds, &y_bounds, i, j);
            if rtn.passes_through {
                valid_positions += 1;
                if rtn.top_position.1 > highest_position_seen.top_position.1 {
                    highest_position_seen = rtn;
                }
            }
        }
    }
    println!(
        "The return for X_vel {} and y_vel {} was {} with a top position of {}",
        highest_position_seen.initial_x_vel, highest_position_seen.initial_y_vel,
        highest_position_seen.passes_through,
        highest_position_seen.top_position.1
    );

    println!("There were a total of {} valid positions", valid_positions)
}

fn check_passes_in_box(
    x_bounds: &Vec<i32>,
    y_bounds: &Vec<i32>,
    x_velocity: i32,
    y_velocity: i32,
) -> PotentialVelocity {
    let mut rtn_vel = PotentialVelocity {
        passes_through: false,
        initial_x_vel: x_velocity,
        initial_y_vel: y_velocity,
        top_position: (i32::MIN, i32::MIN),
    };
    let mut current_step = (0, 0);
    let mut current_velocity = (x_velocity, y_velocity);
    while current_step.0 <= x_bounds[1] && y_bounds[0] <= current_step.1 {
        current_step.0 += current_velocity.0;
        current_step.1 += current_velocity.1;
        if rtn_vel.top_position.1 < current_step.1 {
            rtn_vel.top_position = current_step;
        }

        if current_step.0 >= x_bounds[0]
            && current_step.0 <= x_bounds[1]
            && current_step.1 >= y_bounds[0]
            && current_step.1 <= y_bounds[1]
        {
            rtn_vel.passes_through = true;
        }

        if current_velocity.0 < 0 {
            current_velocity.0 += 1;
        } else if current_velocity.0 > 0 {
            current_velocity.0 -= 1;
        }
        current_velocity.1 -= 1;
    }

    return rtn_vel;
}
