use aoc_lib;
use std::collections::HashMap;

fn main() {
    let mut answers = (0, 0);

    let input = String::from(aoc_lib::get_input().trim());

    let mut data: HashMap<&str, i32> = HashMap::new();
    data.insert("children:", 3);
    data.insert("cats:", 7);
    data.insert("samoyeds:", 2);
    data.insert("pomeranians:", 3);
    data.insert("akitas:", 0);
    data.insert("vizslas:", 0);
    data.insert("goldfish:", 5);
    data.insert("trees:", 3);
    data.insert("cars:", 2);
    data.insert("perfumes:", 1);

    for line in input.split("\n") {
        let mut line_iter = line.split(" ");

        let sue_number: i32 = line_iter.nth(1).unwrap().trim_matches(':').parse().unwrap();
        let item_1 = line_iter.next().unwrap();
        let item_n1: i32 = line_iter.next().unwrap().trim_matches(',').parse().unwrap();
        let item_2 = line_iter.next().unwrap();
        let item_n2: i32 = line_iter.next().unwrap().trim_matches(',').parse().unwrap();
        let item_3 = line_iter.next().unwrap();
        let item_n3: i32 = line_iter.next().unwrap().trim_matches(',').parse().unwrap();

        if *data.get(item_1).unwrap() == item_n1
            && *data.get(item_2).unwrap() == item_n2
            && *data.get(item_3).unwrap() == item_n3
        {
            answers.0 = sue_number;
        }

        let item_1_right = if vec!["pomeranians:", "goldfish:"].contains(&item_1) {
            *data.get(item_1).unwrap() > item_n1
        } else if vec!["cats:", "trees:"].contains(&item_1) {
            *data.get(item_1).unwrap() < item_n1
        } else {
            *data.get(item_1).unwrap() == item_n1
        };
        let item_2_right = if vec!["pomeranians:", "goldfish:"].contains(&item_2) {
            *data.get(item_2).unwrap() > item_n2
        } else if vec!["cats:", "trees:"].contains(&item_2) {
            *data.get(item_2).unwrap() < item_n2
        } else {
            *data.get(item_2).unwrap() == item_n2
        };
        let item_3_right = if vec!["pomeranians:", "goldfish:"].contains(&item_3) {
            *data.get(item_3).unwrap() > item_n3
        } else if vec!["cats:", "trees:"].contains(&item_3) {
            *data.get(item_3).unwrap() < item_n3
        } else {
            *data.get(item_3).unwrap() == item_n3
        };

        if item_1_right && item_2_right && item_3_right {
            answers.1 = sue_number;
        }
    }

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
