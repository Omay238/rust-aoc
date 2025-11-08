use itertools::Itertools;
use aoc_lib;

fn main() {
    let mut answers = (0, 0);

    let input = String::from(aoc_lib::get_input().trim());

    let buckets = input.split("\n").map(|x| x.parse::<i32>().unwrap()).collect_vec();
    let mut found_min = false;
    for len in 1..=buckets.len() {
        for combo in buckets.iter().cloned().combinations(len) {
            if combo.iter().sum::<i32>() == 150 {
                answers.0 += 1;
                if found_min == false {
                    answers.1 += 1;
                }
            }
        }
        if answers.0 > 0 {
            found_min = true;
        }
    }

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
