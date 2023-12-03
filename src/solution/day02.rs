use super::*;

pub fn run() {
    let lines = get_input("day02.txt").lines()
        .map(expect_line);

    let mut total_game_id_sum = 0;
    let mut impossible_game_id_sum = 0;
    let mut power_sum = 0;

    for line in lines {
        if let Some((game_id, handfuls)) = line.split_once(": ") {
            let game_id = game_id[5..].parse::<u32>()
                .expect("failed to get game id");

            total_game_id_sum += game_id;

            let mut is_impossible = false;
            let mut required_red = 0;
            let mut required_green = 0;
            let mut required_blue = 0;

            for handful in handfuls.split("; ") {
                for subset in handful.split(", ") {
                    let (quantity, color) = subset.split_once(' ')
                        .expect("failed to get quantity and color");
                    let quantity = quantity.parse::<u32>()
                        .expect("invalid quantity");

                    match color {
                        "red" => {
                            is_impossible = is_impossible || quantity > 12;
                            required_red = required_red.max(quantity);
                        },
                        "green" => {
                            is_impossible = is_impossible || quantity > 13;
                            required_green = required_green.max(quantity);
                        },
                        "blue" => {
                            is_impossible = is_impossible || quantity > 14;
                            required_blue = required_blue.max(quantity);
                        },
                        _ => panic!("invalid color")
                    }
                }
            }

            if is_impossible {
                impossible_game_id_sum += game_id;
            }

            power_sum += required_red * required_green * required_blue;
        }
    }

    let possible_game_id_sum = total_game_id_sum - impossible_game_id_sum;
    
    println!("[02p1] Sum of possible game IDs: {possible_game_id_sum}");
    println!("[02p2] Sum of powers of cube sets: {power_sum}");
}