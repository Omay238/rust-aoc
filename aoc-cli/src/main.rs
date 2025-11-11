fn main() {
    let mut args = std::env::args();
    args.next();
    let year: i32 = args.next().unwrap().parse().unwrap();
    let day: i32 = args.next().unwrap().parse().unwrap();

    let answers = aoc_solutions::solve(
        (year, day),
        aoc_lib::get_input(year.to_string(), day.to_string(), None),
    );

    println!("year {} day {} part {}: {}", year, day, 1, answers.0);
    println!("year {} day {} part {}: {}", year, day, 2, answers.1);
}
