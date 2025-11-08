use aoc_lib;
use itertools::Itertools;

fn main() {
    fn look_and_say(number: &str) -> String {
        let groups = number.chars().chunk_by(|&c| c);

        let mut out = String::new();

        for group in groups.into_iter() {
            out.push_str(&group.1.count().to_string());
            out.push(group.0);
        }
        out
    }

    let mut answers = (0, 0);

    let mut input = String::from(aoc_lib::get_input().trim());

    for _ in 0..40 {
        input = look_and_say(&input);
    }

    answers.0 = input.len();

    for _ in 0..10 {
        input = look_and_say(&input);
    }

    answers.1 = input.len();

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
