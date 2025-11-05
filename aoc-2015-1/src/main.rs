use aoc_lib;

fn main() {
    let mut answers = (0, 0);

    let input = aoc_lib::get_input();

    let mut floor = 0;

    for (id, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
        if floor < 0 && answers.1 == 0 {
            answers.1 = id + 1;
        }
    }

    answers.0 = floor;

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
