use std::collections::HashMap;
use aoc_lib;

fn main() {
    let mut answers = (0, 0);

    let input = String::from(aoc_lib::get_input().trim());

    let mut reindeer: HashMap<&str, (i32, i32, i32, i32, i32, i32, i32)> = HashMap::new();

    for deer in input.split("\n") {
        let mut deer_iter = deer.split(" ");

        let name = deer_iter.next().unwrap();
        let speed: i32 = deer_iter.nth(2).unwrap().parse().unwrap();
        let move_time_allowed: i32 = deer_iter.nth(2).unwrap().parse().unwrap();
        let rest_time_required: i32 = deer_iter.nth(6).unwrap().parse().unwrap();

        reindeer.insert(name, (move_time_allowed, -1, 0, 0, speed, move_time_allowed, rest_time_required));
    }

    for s in 0..=2503 {
        let mut max_score = 0;
        let mut max_deer: Vec<&str> = Vec::new();
        for deer in reindeer.clone().keys() {
            let mut value = *reindeer.get(deer).unwrap();
            // 0: time left moving
            // 1: time left resting
            // 2: distance travelled
            // 3: points

            // 4: speed per second
            // 5: time moving per rest
            // 6: time resting per rest

            if value.0 > 0 {
                value.2 += value.4;

                value.0 -= 1;
                if value.0 == 0 {
                    value.1 = value.6;
                }
            } else if value.1 > 0 {
                value.1 -= 1;
                if value.1 == 0 {
                    value.0 = value.5;
                }
            }
            
            if value.2 > max_score {
                max_score = value.2;
                max_deer = Vec::new();
            }
            if value.2 == max_score {
                max_deer.push(deer);
            }

            reindeer.insert(deer, value);
        }
        for deer in max_deer {
            let mut value = *reindeer.get(deer).unwrap();
            value.3 += 1;
            reindeer.insert(deer, value);
        }
    }

    for deer in reindeer.clone().keys() {
        let value = reindeer.get(deer).unwrap();
        if value.2 > answers.0 {
            answers.0 = value.2;
        }
        if value.3 > answers.1 {
            answers.1 = value.3;
        }
    }

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
