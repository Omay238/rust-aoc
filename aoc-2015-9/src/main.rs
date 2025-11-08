use aoc_lib;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let mut answers = (usize::MAX, 0);

    let input = String::from(aoc_lib::get_input().trim());

    let mut dists: HashMap<(&str, &str), usize> = HashMap::new();
    let mut cities: Vec<&str> = Vec::new();

    for line in input.split("\n") {
        let mut line_iter = line.split(" ");
        let start = line_iter.next().unwrap();
        line_iter.next();
        let end = line_iter.next().unwrap();
        line_iter.next();
        let dist = line_iter.next().unwrap().parse().unwrap();

        if !cities.contains(&start) {
            cities.push(start);
        }
        if !cities.contains(&end) {
            cities.push(end);
        }

        dists.insert((start, end), dist);
        dists.insert((end, start), dist);
    }

    for perm in cities.iter().permutations(cities.len()) {
        let mut total_dist = 0;
        let mut valid = true;

        for idx in 0..perm.len() - 1 {
            if let Some(&dist) = dists.get(&(perm[idx], perm[idx + 1])) {
                total_dist += dist;
            } else {
                valid = false;
                break;
            }
        }

        if valid {
            answers.0 = answers.0.min(total_dist);
            answers.1 = answers.1.max(total_dist);
        }
    }

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
