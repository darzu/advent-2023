use std::fs;

static RED: &str = "red";
static GREEN: &str = "green";
static BLUE: &str = "blue";

pub fn day2() {
    println!("day2");

    let file_path = "./day2/input.txt";

    let input = fs::read_to_string(file_path).unwrap();

    let mut sum = 0;
    for (game_idx, game) in input.lines().enumerate() {
        let mut max_red: u32 = 1;
        let mut max_green: u32 = 1;
        let mut max_blue: u32 = 1;

        let sets = game.split(':').nth(1).unwrap().split(';');
        for set in sets {
            // println!("{}", set);
            let colors = set.split(',').map(|s| s.trim().split(' '));
            for mut count_and_color in colors {
                let count_str = count_and_color.nth(0).unwrap();
                let count = count_str.parse::<u32>().unwrap();
                let color = count_and_color.nth(0).unwrap();
                if color == RED && count > max_red {
                    max_red = count
                } else if color == GREEN && count > max_green {
                    max_green = count
                } else if color == BLUE && count > max_blue {
                    max_blue = count
                }
            }
        }

        // part 1:
        // if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
        //     let game_id = game_idx + 1;
        //     sum += game_id;
        // }

        // part 2
        let power = max_red * max_green * max_blue;
        sum += power;
    }
    println!("sum: {}", sum);
}
