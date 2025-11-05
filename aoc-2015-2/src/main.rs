use aoc_lib;

fn main() {
    let mut answers = (0, 0);

    let input = aoc_lib::get_input();

    for present in input.split("\n") {
        if present.len() > 1 {
            let mut split_present = present.split("x");

            let length: i32 = split_present.next().unwrap().parse().unwrap();
            let width: i32 = split_present.next().unwrap().parse().unwrap();
            let height: i32 = split_present.next().unwrap().parse().unwrap();

            answers.0 += 2 * length * width
                + 2 * width * height
                + 2 * height * length
                + std::cmp::min(
                    length * width,
                    std::cmp::min(width * height, height * length),
                );

            answers.1 += std::cmp::min(
                2 * length + 2 * width,
                std::cmp::min(2 * width + 2 * height, 2 * height + 2 * length),
            ) + length * width * height;
        }
    }

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
