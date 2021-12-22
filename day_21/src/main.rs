struct Player {
    position: i32,
    score: i32,
}
fn main() {
    let mut players: Vec<Player> = Vec::from([
        Player {
            position: 10,
            score: 0,
        },
        Player {
            position: 7,
            score: 0,
        },
    ]);
    let dice_size = 100;
    first(&mut players, dice_size);
}

fn first(players: &mut Vec<Player>, dice_size: i32) {
    let mut current_dice_position = 1;
    let mut die_rolled_times = 0;
    while players[0].score < 1000 && players[1].score < 1000 {
        for player in players.iter_mut() {
            player.position = (((player.position + add_dice_rolls(current_dice_position, dice_size)) - 1) % 10) + 1;
            player.score += player.position;
            current_dice_position += 3;
            die_rolled_times += 3;
            if player.score >= 1000{
                break;
            }
        }

    }
    println!("{}", i32::min(players[0].score, players[1].score) * &die_rolled_times);
}

fn add_dice_rolls(start: i32, dice_size: i32) -> i32 {
    let mut rtn = 0;
    for roll in start..start + 3 {
        rtn += ((roll - 1) % dice_size) + 1;
    }
    return rtn;
}
