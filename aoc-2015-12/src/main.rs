use aoc_lib;
use json::JsonValue;

fn main() {
    fn sum_children(json_value: JsonValue) -> i32 {
        let mut total = 0;
        if json_value.is_array() {
            for el in json_value.members() {
                if el.is_array() || el.is_object() {
                    total += sum_children(el.clone());
                } else if el.is_number() {
                    total += el.as_i32().unwrap();
                }
            }
        } else if json_value.is_object() {
            for (_, el) in json_value.entries() {
                if el.is_array() || el.is_object() {
                    total += sum_children(el.clone());
                } else if el.is_number() {
                    total += el.as_i32().unwrap();
                }
            }
        } else if json_value.is_number() {
            total += json_value.as_i32().unwrap();
        }

        total
    }

    fn sum_children_no_red(json_value: JsonValue) -> i32 {
        let mut total = 0;
        if json_value.is_array() {
            for el in json_value.members() {
                if el.is_array() || el.is_object() {
                    total += sum_children_no_red(el.clone());
                } else if el.is_number() {
                    total += el.as_i32().unwrap();
                }
            }
        } else if json_value.is_object() {
            let mut isnt_red = true;
            for (txt, el) in json_value.entries() {
                if txt == "red" {
                    isnt_red = false;
                } else if el.is_string() {
                    if el.as_str().unwrap() == "red" {
                        isnt_red = false;
                    }
                }
            }
            if isnt_red {
                for (_, el) in json_value.entries() {
                    if el.is_array() || el.is_object() {
                        total += sum_children_no_red(el.clone());
                    } else if el.is_number() {
                        total += el.as_i32().unwrap();
                    }
                }
            }
        } else if json_value.is_number() {
            total += json_value.as_i32().unwrap();
        }

        total
    }

    let mut answers = (0, 0);

    let input = String::from(aoc_lib::get_input().trim());

    answers.0 = sum_children(json::parse(&input).unwrap());

    answers.1 = sum_children_no_red(json::parse(&input).unwrap());

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
