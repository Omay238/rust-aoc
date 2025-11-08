use aoc_lib;

fn main() {
    fn increment(password: &str) -> String {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";

        let mut out = String::new();

        let mut change_next = true;
        for c in password.chars().rev() {
            let idx = alphabet.find(c).unwrap();
            if change_next {
                if idx == 25 {
                    out.push(alphabet.chars().nth(0).unwrap());
                } else {
                    change_next = false;
                    out.push(alphabet.chars().nth(idx + 1).unwrap());
                }
            } else {
                out.push(c);
            }
        }

        out.chars().rev().collect()
    }

    fn has_dbl_dbl(password: &str) -> bool {
        let zipped = password.chars().zip(password.chars().skip(1));

        for p in zipped.clone().enumerate() {
            for p2 in zipped.clone().enumerate() {
                if p.1.0 == p.1.1 && p2.1.0 == p2.1.1 && p.0.abs_diff(p2.0) > 1 {
                    return true;
                }
            }
        }

        false
    }

    fn has_ascending_alphabet(password: &str) -> bool {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";

        for idx in 0..password.len() - 3 {
            let mut chars = password.chars();
            let str = format!("{}{}{}", chars.nth(idx).unwrap(), chars.next().unwrap(), chars.next().unwrap());
            if alphabet.contains(&str) {
                return true;
            }
        }
        false
    }

    let mut answers = (String::new(), String::new());

    let mut input = String::from(aoc_lib::get_input().trim());

    while input.contains("i") || input.contains("o") || input.contains("l") || !has_dbl_dbl(&input) || !has_ascending_alphabet(&input) {
        input = increment(&input);
    }

    answers.0 = input.clone();

    input = increment(&input);
    
    while input.contains("i") || input.contains("o") || input.contains("l") || !has_dbl_dbl(&input) || !has_ascending_alphabet(&input) {
        input = increment(&input);
    }

    answers.1 = input;

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
