use aoc_lib;

fn main() {
    let mut answers = (0, 0);

    let input = aoc_lib::get_input();

    let mut houses_year_1: Vec<(i32, i32)> = vec![];
    let mut houses_year_2: Vec<(i32, i32)> = vec![];

    let mut pos_santa_year_1 = (0, 0);
    let mut pos_santa_year_2 = (0, 0);
    let mut pos_robo_santa = (0, 0);
    let mut robo = false;

    houses_year_1.push(pos_santa_year_1);
    houses_year_2.push(pos_santa_year_2);
    houses_year_2.push(pos_robo_santa);

    for c in input.chars() {
        let diff = match c {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => (0, 0),
        };

        pos_santa_year_1.0 += diff.0;
        pos_santa_year_1.1 += diff.1;
        if robo {
            pos_robo_santa.0 += diff.0;
            pos_robo_santa.1 += diff.1;
        } else {
            pos_santa_year_2.0 += diff.0;
            pos_santa_year_2.1 += diff.1;
        }

        houses_year_1.push(pos_santa_year_1);
        houses_year_2.push(pos_santa_year_2);
        houses_year_2.push(pos_robo_santa);

        robo = !robo;
    }

    // https://users.rust-lang.org/t/better-way-to-find-unique-values/38966
    let hm1: std::collections::HashSet<_> = houses_year_1.iter().collect();
    answers.0 = hm1.len().into();
    let hm2: std::collections::HashSet<_> = houses_year_2.iter().collect();
    answers.1 = hm2.len().into();

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
