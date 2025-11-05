use aoc_lib;
use md5;
use md5::Digest;

fn main() {
    let mut answers = (0, 0);

    let input = String::from(aoc_lib::get_input().trim());

    let mut num = 0;

    let mut running = true;

    while running {
        num += 1;
        let mut hasher = md5::Md5::new();
        hasher.update(format!("{}{}", input, num));
        let result = format!("{:x}", hasher.finalize());
        if result.starts_with("00000") && answers.0 == 0 {
            answers.0 = num;
        }
        if result.starts_with("000000") {
            answers.1 = num;
            running = false;
        }
    }

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
