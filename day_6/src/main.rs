use std::{
    fs
};

fn main() {
    first();
}

fn first() {
    let filename = "src/data.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut state: Vec<i32> = contents.split(",").map(|fish| fish.trim().parse().unwrap()).collect();

    // Laternfish in the ocean reproduce every 6 days. A new laternfish reproduces after 8 days. 
    //How many lanternfish are there after 80 and 256 days?
    for i in 0..256 {
        let mut new_fish = 0;
        for fish in 0..state.len(){
            let value = state[fish];
            if value == 0 {
                new_fish += 1;
                state[fish] = 6;
            } else {
                state[fish] -= 1;
            }
        }
        for new in 0..new_fish {
            state.push(8);
        }
        if i == 79 {
            println!("There are {} Fish after {} days", state.len(), i);
        }
    }
    println!("There are {} Fish after {} days", state.len(), 256);

}
