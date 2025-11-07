use aoc_lib;

fn main() {
    let mut answers = (0, 0);

    let input = String::from(aoc_lib::get_input().trim());

    let mut lights_part_1 = vec![vec![false; 1000]; 1000];
    let mut lights_part_2 = vec![vec![0; 1000]; 1000];

    for line in input.split("\n") {
        let mut components = line.trim().split(" ");

        let mode: u8;

        if components.next().unwrap() == "turn" {
            if components.next().unwrap() == "on" {
                mode = 0;
            } else {
                mode = 1;
            }
        } else {
            mode = 2;
        }

        let mut start = components.next().unwrap().split(",");
        let start_x: usize = start.next().unwrap().parse().unwrap();
        let start_y: usize = start.next().unwrap().parse().unwrap();
        components.next();
        let mut end = components.next().unwrap().split(",");
        let end_x: usize = end.next().unwrap().parse().unwrap();
        let end_y: usize = end.next().unwrap().parse().unwrap();

        for light_x in start_x..end_x + 1 {
            for light_y in start_y..end_y + 1 {
                if mode == 0 {
                    lights_part_1[light_y][light_x] = true;
                    lights_part_2[light_y][light_x] += 1;
                } else if mode == 1 {
                    lights_part_1[light_y][light_x] = false;
                    lights_part_2[light_y][light_x] = (lights_part_2[light_y][light_x] - 1).max(0);
                } else if mode == 2 {
                    lights_part_1[light_y][light_x] = !lights_part_1[light_y][light_x];
                    lights_part_2[light_y][light_x] += 2;
                }
            }
        }
    }

    answers.0 = lights_part_1
        .iter()
        .flatten()
        .filter(|n| n == &&true)
        .count();
    answers.1 = lights_part_2.iter().flatten().sum();

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
