use aoc_lib;
use std::collections::HashMap;

fn main() {
    fn parse_arg(arg: &str, wires: &HashMap<String, u16>) -> Option<u16> {
        if arg.parse::<u16>().is_ok() {
            return Some(arg.parse().unwrap());
        } else if wires.contains_key(arg) {
            return Some(*wires.get(arg).unwrap());
        }
        None
    }

    fn calculate_wires(
        completed_instructions: &mut Vec<bool>,
        input: &str,
        wires: &mut HashMap<String, u16>,
    ) {
        while completed_instructions.iter().filter(|el| **el).count() < input.split("\n").count() {
            for line in input.split("\n").enumerate() {
                if completed_instructions[line.0] == false {
                    let mut iter_completed = false;
                    let mut line_iter = line.1.split(" ");
                    if line.1.contains("AND") {
                        if let Some(v1) = parse_arg(line_iter.next().unwrap(), &wires) {
                            line_iter.next();
                            if let Some(v2) = parse_arg(line_iter.next().unwrap(), &wires) {
                                line_iter.next();
                                wires.insert(String::from(line_iter.next().unwrap()), v1 & v2);
                                iter_completed = true;
                            }
                        }
                    } else if line.1.contains("OR") {
                        if let Some(v1) = parse_arg(line_iter.next().unwrap(), &wires) {
                            line_iter.next();
                            if let Some(v2) = parse_arg(line_iter.next().unwrap(), &wires) {
                                line_iter.next();
                                wires.insert(String::from(line_iter.next().unwrap()), v1 | v2);
                                iter_completed = true;
                            }
                        }
                    } else if line.1.contains("LSHIFT") {
                        if let Some(v1) = parse_arg(line_iter.next().unwrap(), &wires) {
                            line_iter.next();
                            if let Some(v2) = parse_arg(line_iter.next().unwrap(), &wires) {
                                line_iter.next();
                                wires.insert(String::from(line_iter.next().unwrap()), v1 << v2);
                                iter_completed = true;
                            }
                        }
                    } else if line.1.contains("RSHIFT") {
                        if let Some(v1) = parse_arg(line_iter.next().unwrap(), &wires) {
                            line_iter.next();
                            if let Some(v2) = parse_arg(line_iter.next().unwrap(), &wires) {
                                line_iter.next();
                                wires.insert(String::from(line_iter.next().unwrap()), v1 >> v2);
                                iter_completed = true;
                            }
                        }
                    } else if line.1.contains("NOT") {
                        line_iter.next();
                        if let Some(v) = parse_arg(line_iter.next().unwrap(), &wires) {
                            line_iter.next();
                            wires.insert(String::from(line_iter.next().unwrap()), !v);
                            iter_completed = true;
                        }
                    } else {
                        if let Some(v) = parse_arg(line_iter.next().unwrap(), &wires) {
                            line_iter.next();
                            wires.insert(String::from(line_iter.next().unwrap()), v);
                            iter_completed = true;
                        }
                    }

                    if iter_completed {
                        completed_instructions[line.0] = true;
                    }
                }
            }
        }
    }

    let mut answers = (0, 0);

    let input = String::from(aoc_lib::get_input().trim());

    let mut wires: HashMap<String, u16> = HashMap::new();

    let mut completed_instructions: Vec<bool> = vec![false; input.split("\n").count()];

    calculate_wires(&mut completed_instructions, &input, &mut wires);

    answers.0 = *wires.get("a").unwrap();

    completed_instructions = vec![false; input.split("\n").count()];
    wires = HashMap::new();
    wires.insert(String::from("b"), answers.0);

    for (i, line) in input.split("\n").enumerate() {
        if line.ends_with("-> b") {
            completed_instructions[i] = true;
        }
    }

    calculate_wires(&mut completed_instructions, &input, &mut wires);

    answers.1 = *wires.get("a").unwrap();

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
