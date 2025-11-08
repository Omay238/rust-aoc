use aoc_lib;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let mut answers = (0, 0);

    let input = String::from(aoc_lib::get_input().trim());

    let mut people: Vec<&str> = Vec::new();

    let mut relations: HashMap<(&str, &str), i32> = HashMap::new();

    for line in input.split("\n") {
        let mut line_iter = line.split(" ");
        let src_person = line_iter.next().unwrap();
        if !people.contains(&src_person) {
            people.push(src_person);
        }
        line_iter.next();
        let sign = if line_iter.next().unwrap() == "lose" {
            -1
        } else {
            1
        };
        let score = line_iter.next().unwrap().parse::<i32>().unwrap() * sign;
        let dst_person = line_iter.last().unwrap();

        relations.insert((src_person, dst_person.trim_matches('.')), score);
    }

    for perm in people.iter().permutations(people.len()) {
        let mut relation_status = 0;

        for (idx, person) in perm.iter().enumerate() {
            relation_status += relations
                .get(&(**person, *perm[(idx + 1) % (perm.len())]))
                .unwrap();
            relation_status += relations
                .get(&(*perm[(idx + 1) % (perm.len())], **person))
                .unwrap();
        }

        if relation_status > answers.0 {
            answers.0 = relation_status
        }
    }

    for person in people.clone() {
        relations.insert((person, "me"), 0);
        relations.insert(("me", person), 0);
    }

    people.push("me");

    for perm in people.iter().permutations(people.len()) {
        let mut relation_status = 0;

        for (idx, person) in perm.iter().enumerate() {
            relation_status += relations
                .get(&(**person, *perm[(idx + 1) % (perm.len())]))
                .unwrap();
            relation_status += relations
                .get(&(*perm[(idx + 1) % (perm.len())], **person))
                .unwrap();
        }

        if relation_status > answers.1 {
            answers.1 = relation_status
        }
    }

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
