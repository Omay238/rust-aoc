use aoc_lib;

fn main() {
    fn get_storage(literal: &str) -> usize {
        let literal = literal.trim();
        literal.len()
    }

    fn get_memory(literal: &str) -> usize {
        let literal = literal.trim();
        let chars: Vec<char> = literal.chars().collect();
        let mut total = 0;
        let mut c = 1;
        while c < literal.len() - 1 {
            if chars[c] == '\\' {
                if chars[c + 1] == '\\' {
                    c += 1;
                } else if chars[c + 1] == '"' {
                    c += 1;
                } else if chars[c + 1] == 'x' {
                    c += 3;
                }
            }
            c += 1;
            total += 1;
        }
        total
    }

    fn get_encoded(literal: &str) -> usize {
        // + 2 because of the starting/ending quotes
        literal.replace("\\", "\\\\").replace("\"", "\\\"").len() + 2
    }

    let mut answers = (0, 0);

    let input = String::from(aoc_lib::get_input().trim());

    for line in input.split("\n") {
        answers.0 += get_storage(line);
        answers.0 -= get_memory(line);

        answers.1 += get_encoded(line);
        answers.1 -= get_storage(line);
    }

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
