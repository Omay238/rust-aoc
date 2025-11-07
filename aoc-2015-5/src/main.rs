use aoc_lib;

fn main() {
    let mut answers = (0, 0);

    let input = String::from(aoc_lib::get_input().trim());

    for line in input.split("\n") {
        let l = line.trim();
        let has_dbl = l.chars().zip(l.chars().skip(1)).any(|(a, b)| a == b);
        let has_vowels = l.chars().filter(|&el| "aeiou".contains(el)).count() > 2;
        let no_disallowed =
            !(l.contains("ab") || l.contains("cd") || l.contains("pq") || l.contains("xy"));

        let zipped = l.chars().zip(l.chars().skip(1));
        let mut has_dbl_dbl = false;

        // i bet there's a way to do it like i did with the previous double thing, but i've come up blank
        for p in zipped.clone().enumerate() {
            for p2 in zipped.clone().enumerate() {
                if p.1 == p2.1 && p.0.abs_diff(p2.0) > 1 {
                    has_dbl_dbl = true;
                    break;
                }
            }
        }

        let has_rep = l.chars().zip(l.chars().skip(2)).any(|(a, b)| a == b);

        if has_dbl && has_vowels && no_disallowed {
            answers.0 += 1
        }

        if has_dbl_dbl && has_rep {
            answers.1 += 1
        }
    }

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
