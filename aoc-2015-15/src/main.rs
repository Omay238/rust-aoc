use aoc_lib;

fn main() {
    let mut answers = (0, 0);

    let input = String::from(aoc_lib::get_input().trim());

    let mut ingredients: Vec<(i32, i32, i32, i32, i32)> = Vec::new();

    for line in input.split("\n") {
        let mut line_iter = line.split(" ");

        let capacity: i32 = line_iter.nth(2).unwrap().trim_matches(',').parse().unwrap();
        let durability: i32 = line_iter.nth(1).unwrap().trim_matches(',').parse().unwrap();
        let flavor: i32 = line_iter.nth(1).unwrap().trim_matches(',').parse().unwrap();
        let texture: i32 = line_iter.nth(1).unwrap().trim_matches(',').parse().unwrap();
        let calories: i32 = line_iter.nth(1).unwrap().parse().unwrap();

        ingredients.push((capacity, durability, flavor, texture, calories));
    }

    for a in 0..=100 {
        for b in 0..=100 - a {
            for c in 0..=100 - a - b {
                let d = 100 - a - b - c;
                let capacity_a = ingredients[0].0 * a;
                let durability_a = ingredients[0].1 * a;
                let flavor_a = ingredients[0].2 * a;
                let texture_a = ingredients[0].3 * a;
                let calories_a = ingredients[0].4 * a;

                let capacity_b = ingredients[1].0 * b;
                let durability_b = ingredients[1].1 * b;
                let flavor_b = ingredients[1].2 * b;
                let texture_b = ingredients[1].3 * b;
                let calories_b = ingredients[1].4 * b;

                let capacity_c = ingredients[2].0 * c;
                let durability_c = ingredients[2].1 * c;
                let flavor_c = ingredients[2].2 * c;
                let texture_c = ingredients[2].3 * c;
                let calories_c = ingredients[2].4 * c;

                let capacity_d = ingredients[3].0 * d;
                let durability_d = ingredients[3].1 * d;
                let flavor_d = ingredients[3].2 * d;
                let texture_d = ingredients[3].3 * d;
                let calories_d = ingredients[3].4 * d;

                let capacity = 0.max(capacity_a + capacity_b + capacity_c + capacity_d);
                let durability = 0.max(durability_a + durability_b + durability_c + durability_d);
                let flavor = 0.max(flavor_a + flavor_b + flavor_c + flavor_d);
                let texture = 0.max(texture_a + texture_b + texture_c + texture_d);
                let calories = 0.max(calories_a + calories_b + calories_c + calories_d);

                let score = capacity * durability * flavor * texture;

                if score > answers.0 {
                    answers.0 = score;
                }
                if score > answers.1 && calories == 500 {
                    answers.1 = score;
                }
            }
        }
    }

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}
