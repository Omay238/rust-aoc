#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn js_solve(year: i32, day: i32, input: String) -> JsValue {
    let solution = solve((year, day), input);
    wasm_bindgen::JsValue::from(vec![solution.0, solution.1])
}

pub fn solve(date: (i32, i32), mut input: String) -> (String, String) {
    input = String::from(input.trim());

    // while not all solutions are numbers, most are so this is easiest.
    let mut answers: (i32, i32) = (0, 0);

    match date.0 {
        2015 => match date.1 {
            1 => {
                let mut floor = 0;

                for (id, c) in input.chars().enumerate() {
                    if c == '(' {
                        floor += 1;
                    } else if c == ')' {
                        floor -= 1;
                    }
                    if floor < 0 && answers.1 == 0 {
                        answers.1 = (id + 1) as i32;
                    }
                }

                answers.0 = floor;
            }
            2 => {
                for present in input.split("\n") {
                    let mut split_present = present.split("x");

                    let length: i32 = split_present.next().unwrap().parse().unwrap();
                    let width: i32 = split_present.next().unwrap().parse().unwrap();
                    let height: i32 = split_present.next().unwrap().parse().unwrap();

                    answers.0 += 2 * length * width
                        + 2 * width * height
                        + 2 * height * length
                        + std::cmp::min(
                            length * width,
                            std::cmp::min(width * height, height * length),
                        );

                    answers.1 += std::cmp::min(
                        2 * length + 2 * width,
                        std::cmp::min(2 * width + 2 * height, 2 * height + 2 * length),
                    ) + length * width * height;
                }
            }
            3 => {
                let mut houses_year_1: Vec<(i32, i32)> = vec![];
                let mut houses_year_2: Vec<(i32, i32)> = vec![];

                let mut pos_santa_year_1 = (0, 0);
                let mut pos_santa_year_2 = (0, 0);
                let mut pos_robo_santa = (0, 0);
                let mut robo = false;

                houses_year_1.push(pos_santa_year_1);
                houses_year_2.push(pos_santa_year_2);
                houses_year_2.push(pos_robo_santa);

                for c in input.chars() {
                    let diff = match c {
                        '^' => (0, -1),
                        '>' => (1, 0),
                        'v' => (0, 1),
                        '<' => (-1, 0),
                        _ => (0, 0),
                    };

                    pos_santa_year_1.0 += diff.0;
                    pos_santa_year_1.1 += diff.1;
                    if robo {
                        pos_robo_santa.0 += diff.0;
                        pos_robo_santa.1 += diff.1;
                    } else {
                        pos_santa_year_2.0 += diff.0;
                        pos_santa_year_2.1 += diff.1;
                    }

                    houses_year_1.push(pos_santa_year_1);
                    houses_year_2.push(pos_santa_year_2);
                    houses_year_2.push(pos_robo_santa);

                    robo = !robo;
                }

                // https://users.rust-lang.org/t/better-way-to-find-unique-values/38966
                let hm1: std::collections::HashSet<_> = houses_year_1.iter().collect();
                answers.0 = hm1.len() as i32;
                let hm2: std::collections::HashSet<_> = houses_year_2.iter().collect();
                answers.1 = hm2.len() as i32;
            }
            4 => {
                use md5::Digest;

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
            }
            5 => {
                for line in input.split("\n") {
                    let l = line.trim();
                    let has_dbl = l.chars().zip(l.chars().skip(1)).any(|(a, b)| a == b);
                    let has_vowels = l.chars().filter(|&el| "aeiou".contains(el)).count() > 2;
                    let no_disallowed = !(l.contains("ab")
                        || l.contains("cd")
                        || l.contains("pq")
                        || l.contains("xy"));

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
            }
            6 => {
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
                                lights_part_2[light_y][light_x] =
                                    (lights_part_2[light_y][light_x] - 1).max(0);
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
                    .count() as i32;
                answers.1 = lights_part_2.iter().flatten().sum();
            }
            7 => {
                use std::collections::HashMap;

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
                    while completed_instructions.iter().filter(|el| **el).count()
                        < input.split("\n").count()
                    {
                        for line in input.split("\n").enumerate() {
                            if completed_instructions[line.0] == false {
                                let mut iter_completed = false;
                                let mut line_iter = line.1.split(" ");
                                if line.1.contains("AND") {
                                    if let Some(v1) = parse_arg(line_iter.next().unwrap(), &wires) {
                                        line_iter.next();
                                        if let Some(v2) =
                                            parse_arg(line_iter.next().unwrap(), &wires)
                                        {
                                            line_iter.next();
                                            wires.insert(
                                                String::from(line_iter.next().unwrap()),
                                                v1 & v2,
                                            );
                                            iter_completed = true;
                                        }
                                    }
                                } else if line.1.contains("OR") {
                                    if let Some(v1) = parse_arg(line_iter.next().unwrap(), &wires) {
                                        line_iter.next();
                                        if let Some(v2) =
                                            parse_arg(line_iter.next().unwrap(), &wires)
                                        {
                                            line_iter.next();
                                            wires.insert(
                                                String::from(line_iter.next().unwrap()),
                                                v1 | v2,
                                            );
                                            iter_completed = true;
                                        }
                                    }
                                } else if line.1.contains("LSHIFT") {
                                    if let Some(v1) = parse_arg(line_iter.next().unwrap(), &wires) {
                                        line_iter.next();
                                        if let Some(v2) =
                                            parse_arg(line_iter.next().unwrap(), &wires)
                                        {
                                            line_iter.next();
                                            wires.insert(
                                                String::from(line_iter.next().unwrap()),
                                                v1 << v2,
                                            );
                                            iter_completed = true;
                                        }
                                    }
                                } else if line.1.contains("RSHIFT") {
                                    if let Some(v1) = parse_arg(line_iter.next().unwrap(), &wires) {
                                        line_iter.next();
                                        if let Some(v2) =
                                            parse_arg(line_iter.next().unwrap(), &wires)
                                        {
                                            line_iter.next();
                                            wires.insert(
                                                String::from(line_iter.next().unwrap()),
                                                v1 >> v2,
                                            );
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

                let mut wires: HashMap<String, u16> = HashMap::new();

                let mut completed_instructions: Vec<bool> = vec![false; input.split("\n").count()];

                calculate_wires(&mut completed_instructions, &input, &mut wires);

                answers.0 = *wires.get("a").unwrap() as i32;

                completed_instructions = vec![false; input.split("\n").count()];
                wires = HashMap::new();
                wires.insert(String::from("b"), answers.0 as u16);

                for (i, line) in input.split("\n").enumerate() {
                    if line.ends_with("-> b") {
                        completed_instructions[i] = true;
                    }
                }

                calculate_wires(&mut completed_instructions, &input, &mut wires);

                answers.1 = *wires.get("a").unwrap() as i32;
            }
            8 => {
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

                for line in input.split("\n") {
                    answers.0 += get_storage(line) as i32;
                    answers.0 -= get_memory(line) as i32;

                    answers.1 += get_encoded(line) as i32;
                    answers.1 -= get_storage(line) as i32;
                }
            }
            9 => {
                use itertools::Itertools;
                use std::collections::HashMap;

                let mut dists: HashMap<(&str, &str), usize> = HashMap::new();
                let mut cities: Vec<&str> = Vec::new();

                for line in input.split("\n") {
                    let mut line_iter = line.split(" ");
                    let start = line_iter.next().unwrap();
                    line_iter.next();
                    let end = line_iter.next().unwrap();
                    line_iter.next();
                    let dist = line_iter.next().unwrap().parse().unwrap();

                    if !cities.contains(&start) {
                        cities.push(start);
                    }
                    if !cities.contains(&end) {
                        cities.push(end);
                    }

                    dists.insert((start, end), dist);
                    dists.insert((end, start), dist);
                }

                for perm in cities.iter().permutations(cities.len()) {
                    let mut total_dist = 0;
                    let mut valid = true;

                    for idx in 0..perm.len() - 1 {
                        if let Some(&dist) = dists.get(&(perm[idx], perm[idx + 1])) {
                            total_dist += dist;
                        } else {
                            valid = false;
                            break;
                        }
                    }

                    if valid {
                        answers.0 = answers.0.min(total_dist as i32);
                        answers.1 = answers.1.max(total_dist as i32);
                    }
                }
            }
            10 => {
                use itertools::Itertools;

                fn look_and_say(number: &str) -> String {
                    let groups = number.chars().chunk_by(|&c| c);

                    let mut out = String::new();

                    for group in groups.into_iter() {
                        out.push_str(&group.1.count().to_string());
                        out.push(group.0);
                    }
                    out
                }

                for _ in 0..40 {
                    input = look_and_say(&input);
                }

                answers.0 = input.len() as i32;

                for _ in 0..10 {
                    input = look_and_say(&input);
                }

                answers.1 = input.len() as i32;
            }
            11 => {
                let mut answers: (String, String) = (String::new(), String::new());

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
                        let str = format!(
                            "{}{}{}",
                            chars.nth(idx).unwrap(),
                            chars.next().unwrap(),
                            chars.next().unwrap()
                        );
                        if alphabet.contains(&str) {
                            return true;
                        }
                    }
                    false
                }

                while input.contains("i")
                    || input.contains("o")
                    || input.contains("l")
                    || !has_dbl_dbl(&input)
                    || !has_ascending_alphabet(&input)
                {
                    input = increment(&input);
                }

                answers.0 = input.clone();

                input = increment(&input);

                while input.contains("i")
                    || input.contains("o")
                    || input.contains("l")
                    || !has_dbl_dbl(&input)
                    || !has_ascending_alphabet(&input)
                {
                    input = increment(&input);
                }

                answers.1 = input;

                return answers;
            }
            12 => {
                fn sum_children(json_value: json::JsonValue) -> i32 {
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

                fn sum_children_no_red(json_value: json::JsonValue) -> i32 {
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

                answers.0 = sum_children(json::parse(&input).unwrap());

                answers.1 = sum_children_no_red(json::parse(&input).unwrap());
            }
            13 => {
                use itertools::Itertools;
                use std::collections::HashMap;

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
            }
            14 => {
                use std::collections::HashMap;

                let mut reindeer: HashMap<&str, (i32, i32, i32, i32, i32, i32, i32)> =
                    HashMap::new();

                for deer in input.split("\n") {
                    let mut deer_iter = deer.split(" ");

                    let name = deer_iter.next().unwrap();
                    let speed: i32 = deer_iter.nth(2).unwrap().parse().unwrap();
                    let move_time_allowed: i32 = deer_iter.nth(2).unwrap().parse().unwrap();
                    let rest_time_required: i32 = deer_iter.nth(6).unwrap().parse().unwrap();

                    reindeer.insert(
                        name,
                        (
                            move_time_allowed,
                            -1,
                            0,
                            0,
                            speed,
                            move_time_allowed,
                            rest_time_required,
                        ),
                    );
                }

                for _ in 0..=2503 {
                    let mut max_score = 0;
                    let mut max_deer: Vec<&str> = Vec::new();
                    for deer in reindeer.clone().keys() {
                        let mut value = *reindeer.get(deer).unwrap();
                        // 0: time left moving
                        // 1: time left resting
                        // 2: distance travelled
                        // 3: points

                        // 4: speed per second
                        // 5: time moving per rest
                        // 6: time resting per rest

                        if value.0 > 0 {
                            value.2 += value.4;

                            value.0 -= 1;
                            if value.0 == 0 {
                                value.1 = value.6;
                            }
                        } else if value.1 > 0 {
                            value.1 -= 1;
                            if value.1 == 0 {
                                value.0 = value.5;
                            }
                        }

                        if value.2 > max_score {
                            max_score = value.2;
                            max_deer = Vec::new();
                        }
                        if value.2 == max_score {
                            max_deer.push(deer);
                        }

                        reindeer.insert(deer, value);
                    }
                    for deer in max_deer {
                        let mut value = *reindeer.get(deer).unwrap();
                        value.3 += 1;
                        reindeer.insert(deer, value);
                    }
                }

                for deer in reindeer.clone().keys() {
                    let value = reindeer.get(deer).unwrap();
                    if value.2 > answers.0 {
                        answers.0 = value.2;
                    }
                    if value.3 > answers.1 {
                        answers.1 = value.3;
                    }
                }
            }
            15 => {
                let mut ingredients: Vec<(i32, i32, i32, i32, i32)> = Vec::new();

                for line in input.split("\n") {
                    let mut line_iter = line.split(" ");

                    let capacity: i32 =
                        line_iter.nth(2).unwrap().trim_matches(',').parse().unwrap();
                    let durability: i32 =
                        line_iter.nth(1).unwrap().trim_matches(',').parse().unwrap();
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
                            let durability =
                                0.max(durability_a + durability_b + durability_c + durability_d);
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
            }
            16 => {
                use std::collections::HashMap;

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

                    let sue_number: i32 =
                        line_iter.nth(1).unwrap().trim_matches(':').parse().unwrap();
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
            }
            17 => {
                use itertools::Itertools;

                let buckets = input
                    .split("\n")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect_vec();
                let mut found_min = false;
                for len in 1..=buckets.len() {
                    for combo in buckets.iter().cloned().combinations(len) {
                        if combo.iter().sum::<i32>() == 150 {
                            answers.0 += 1;
                            if found_min == false {
                                answers.1 += 1;
                            }
                        }
                    }
                    if answers.0 > 0 {
                        found_min = true;
                    }
                }
            }
            18 => {
                let mut grid = input
                    .split("\n")
                    .map(|x| x.chars().map(|y| y == '#').collect::<Vec<bool>>())
                    .collect::<Vec<Vec<bool>>>();

                for _ in 0..100 {
                    let mut holder_grid = Vec::new();
                    for y in 0..grid.len() {
                        holder_grid.push(Vec::new());
                        for x in 0..grid[y].len() {
                            let mut neighbors = 0;
                            if let Some(ym1) = grid.get(y - 1) {
                                if let Some(xm1) = ym1.get(x - 1) {
                                    if *xm1 {
                                        neighbors += 1
                                    }
                                }
                                if let Some(xp1) = ym1.get(x + 1) {
                                    if *xp1 {
                                        neighbors += 1
                                    }
                                }
                                if ym1[x] {
                                    neighbors += 1
                                }
                            }
                            if let Some(xm1) = grid[y].get(x - 1) {
                                if *xm1 {
                                    neighbors += 1
                                }
                            }
                            if let Some(xp1) = grid[y].get(x + 1) {
                                if *xp1 {
                                    neighbors += 1
                                }
                            }
                            if let Some(yp1) = grid.get(y + 1) {
                                if let Some(xm1) = yp1.get(x - 1) {
                                    if *xm1 {
                                        neighbors += 1
                                    }
                                }
                                if let Some(xp1) = yp1.get(x + 1) {
                                    if *xp1 {
                                        neighbors += 1
                                    }
                                }
                                if yp1[x] {
                                    neighbors += 1
                                }
                            }
                            if neighbors == 3 {
                                holder_grid[y].push(true);
                            } else if neighbors == 2 {
                                holder_grid[y].push(grid[y][x]);
                            } else {
                                holder_grid[y].push(false);
                            }
                        }
                    }
                    grid = holder_grid;
                }

                answers.0 = grid
                    .iter()
                    .map(|x| x.iter().map(|y| if *y { 1 } else { 0 }).sum::<i32>())
                    .sum();

                grid = input
                    .split("\n")
                    .map(|x| x.chars().map(|y| y == '#').collect::<Vec<bool>>())
                    .collect::<Vec<Vec<bool>>>();

                let w = grid[0].len() - 1;
                let h = grid.len() - 1;
                grid[0][0] = true;
                grid[0][w] = true;
                grid[h][0] = true;
                grid[h][w] = true;

                for _ in 0..100 {
                    let mut holder_grid = Vec::new();
                    for y in 0..grid.len() {
                        holder_grid.push(Vec::new());
                        for x in 0..grid[y].len() {
                            let mut neighbors = 0;
                            if let Some(ym1) = grid.get(y - 1) {
                                if let Some(xm1) = ym1.get(x - 1) {
                                    if *xm1 {
                                        neighbors += 1
                                    }
                                }
                                if let Some(xp1) = ym1.get(x + 1) {
                                    if *xp1 {
                                        neighbors += 1
                                    }
                                }
                                if ym1[x] {
                                    neighbors += 1
                                }
                            }
                            if let Some(xm1) = grid[y].get(x - 1) {
                                if *xm1 {
                                    neighbors += 1
                                }
                            }
                            if let Some(xp1) = grid[y].get(x + 1) {
                                if *xp1 {
                                    neighbors += 1
                                }
                            }
                            if let Some(yp1) = grid.get(y + 1) {
                                if let Some(xm1) = yp1.get(x - 1) {
                                    if *xm1 {
                                        neighbors += 1
                                    }
                                }
                                if let Some(xp1) = yp1.get(x + 1) {
                                    if *xp1 {
                                        neighbors += 1
                                    }
                                }
                                if yp1[x] {
                                    neighbors += 1
                                }
                            }
                            if neighbors == 3
                                || (x == 0 && y == 0)
                                || (x == 0 && y == grid.len() - 1)
                                || (x == grid[y].len() - 1 && y == grid.len() - 1)
                                || (x == grid[y].len() - 1 && y == 0)
                            {
                                holder_grid[y].push(true);
                            } else if neighbors == 2 {
                                holder_grid[y].push(grid[y][x]);
                            } else {
                                holder_grid[y].push(false);
                            }
                        }
                    }
                    grid = holder_grid;
                }

                answers.1 = grid
                    .iter()
                    .map(|x| x.iter().map(|y| if *y { 1 } else { 0 }).sum::<i32>())
                    .sum();
            }
            19 => {
                use std::collections::HashSet;

                fn exec_replacements(
                    replacements: &Vec<(String, String)>,
                    chemical: &String,
                ) -> HashSet<String> {
                    let mut chemicals = HashSet::new();
                    for replacement in replacements.iter() {
                        for idx in 0..=chemical.len() - replacement.0.len() + 1 {
                            if chemical
                                .chars()
                                .skip(idx)
                                .take(replacement.0.len())
                                .collect::<String>()
                                == replacement.0
                            {
                                chemicals.insert(
                                    chemical
                                        .chars()
                                        .take(idx)
                                        .chain(replacement.1.chars())
                                        .chain(chemical.chars().skip(idx + replacement.0.len()))
                                        .collect(),
                                );
                            }
                        }
                    }
                    chemicals
                }

                let mut chemical = String::new();
                let mut replacements = Vec::new();

                for line in input.lines() {
                    if line.contains("=>") {
                        let mut split = line.split(" => ");
                        replacements.push((
                            String::from(split.next().unwrap()),
                            String::from(split.next().unwrap()),
                        ))
                    } else if line.len() > 0 {
                        chemical = String::from(line);
                    }
                }

                answers.0 = exec_replacements(&replacements, &chemical).iter().count() as i32;

                // https://www.reddit.com/r/adventofcode/comments/3xflz8/comment/cy4h7ji
                answers.1 = (chemical.chars().filter(|c| c.is_uppercase()).count()
                    - chemical.matches("Rn").count()
                    - chemical.matches("Ar").count()
                    - 2 * chemical.matches("Y").count()
                    - 1) as i32;
            }
            20 => {
                fn get_factors(num: i32) -> Vec<i32> {
                    let mut factors = Vec::new();
                    let mut i = 1;

                    while i * i < num {
                        if num % i == 0 {
                            factors.push(i);

                            if i * i != num {
                                factors.push(num / i);
                            }
                        }
                        i += 1;
                    }

                    factors
                }

                let goal: i32 = input.parse().unwrap();

                while get_factors(answers.0).iter().sum::<i32>() * 10 < goal {
                    answers.0 += 1;
                }

                while get_factors(answers.1)
                    .iter()
                    .filter(|x| answers.1 / *x <= 50)
                    .sum::<i32>()
                    * 11
                    < goal
                {
                    answers.1 += 1;
                }
            }
            21 => {
                for weapon in 0..5 {
                    for armor in -1..5 {
                        for ring1 in -1..6 {
                            for ring2 in -1..6 {
                                if ring1 == ring2 && ring1 != -1 {
                                    continue;
                                }

                                let mut player_hp: i32 = 100;
                                let mut player_damage: i32 = 0;
                                let mut player_armor: i32 = 0;

                                let mut spent_money: i32 = 0;

                                match weapon {
                                    0 => {
                                        spent_money += 8;
                                        player_damage += 4;
                                    }
                                    1 => {
                                        spent_money += 10;
                                        player_damage += 5;
                                    }
                                    2 => {
                                        spent_money += 25;
                                        player_damage += 6;
                                    }
                                    3 => {
                                        spent_money += 40;
                                        player_damage += 7;
                                    }
                                    4 => {
                                        spent_money += 74;
                                        player_damage += 8;
                                    }
                                    _ => {}
                                }
                                match armor {
                                    0 => {
                                        spent_money += 13;
                                        player_armor += 1;
                                    }
                                    1 => {
                                        spent_money += 31;
                                        player_armor += 2;
                                    }
                                    2 => {
                                        spent_money += 53;
                                        player_armor += 3;
                                    }
                                    3 => {
                                        spent_money += 75;
                                        player_armor += 4;
                                    }
                                    4 => {
                                        spent_money += 102;
                                        player_armor += 5;
                                    }
                                    _ => {}
                                }
                                match ring1 {
                                    0 => {
                                        spent_money += 25;
                                        player_damage += 1;
                                    }
                                    1 => {
                                        spent_money += 50;
                                        player_damage += 2;
                                    }
                                    2 => {
                                        spent_money += 100;
                                        player_damage += 3;
                                    }
                                    3 => {
                                        spent_money += 20;
                                        player_armor += 1;
                                    }
                                    4 => {
                                        spent_money += 40;
                                        player_armor += 2;
                                    }
                                    5 => {
                                        spent_money += 80;
                                        player_armor += 3;
                                    }
                                    _ => {}
                                }
                                match ring2 {
                                    0 => {
                                        spent_money += 25;
                                        player_damage += 1;
                                    }
                                    1 => {
                                        spent_money += 50;
                                        player_damage += 2;
                                    }
                                    2 => {
                                        spent_money += 100;
                                        player_damage += 3;
                                    }
                                    3 => {
                                        spent_money += 20;
                                        player_armor += 1;
                                    }
                                    4 => {
                                        spent_money += 40;
                                        player_armor += 2;
                                    }
                                    5 => {
                                        spent_money += 80;
                                        player_armor += 3;
                                    }
                                    _ => {}
                                }

                                let mut input_iter = input.lines();
                                let mut boss_hp: i32 = input_iter
                                    .next()
                                    .unwrap()
                                    .split(": ")
                                    .last()
                                    .unwrap()
                                    .parse()
                                    .unwrap();
                                let boss_damage: i32 = input_iter
                                    .next()
                                    .unwrap()
                                    .split(": ")
                                    .last()
                                    .unwrap()
                                    .parse()
                                    .unwrap();
                                let boss_armor: i32 = input_iter
                                    .next()
                                    .unwrap()
                                    .split(": ")
                                    .last()
                                    .unwrap()
                                    .parse()
                                    .unwrap();

                                let mut current_move = false;

                                while player_hp > 0 && boss_hp > 0 {
                                    current_move = !current_move;

                                    if current_move {
                                        boss_hp -= (player_damage - boss_armor).max(1);
                                    } else {
                                        player_hp -= (boss_damage - player_armor).max(1);
                                    }
                                }

                                if current_move {
                                    if answers.0 == 0 {
                                        answers.0 = spent_money;
                                    }

                                    answers.0 = answers.0.min(spent_money);
                                } else {
                                    answers.1 = answers.1.max(spent_money);
                                }
                            }
                        }
                    }
                }
            }
            22 => {}
            23 => {}
            24 => {}
            25 => {}
            _ => {}
        },
        2016 => match date.1 {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            9 => {}
            10 => {}
            11 => {}
            12 => {}
            13 => {}
            14 => {}
            15 => {}
            16 => {}
            17 => {}
            18 => {}
            19 => {}
            20 => {}
            21 => {}
            22 => {}
            23 => {}
            24 => {}
            25 => {}
            _ => {}
        },
        2017 => match date.1 {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            9 => {}
            10 => {}
            11 => {}
            12 => {}
            13 => {}
            14 => {}
            15 => {}
            16 => {}
            17 => {}
            18 => {}
            19 => {}
            20 => {}
            21 => {}
            22 => {}
            23 => {}
            24 => {}
            25 => {}
            _ => {}
        },
        2018 => match date.1 {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            9 => {}
            10 => {}
            11 => {}
            12 => {}
            13 => {}
            14 => {}
            15 => {}
            16 => {}
            17 => {}
            18 => {}
            19 => {}
            20 => {}
            21 => {}
            22 => {}
            23 => {}
            24 => {}
            25 => {}
            _ => {}
        },
        2019 => match date.1 {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            9 => {}
            10 => {}
            11 => {}
            12 => {}
            13 => {}
            14 => {}
            15 => {}
            16 => {}
            17 => {}
            18 => {}
            19 => {}
            20 => {}
            21 => {}
            22 => {}
            23 => {}
            24 => {}
            25 => {}
            _ => {}
        },
        2020 => match date.1 {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            9 => {}
            10 => {}
            11 => {}
            12 => {}
            13 => {}
            14 => {}
            15 => {}
            16 => {}
            17 => {}
            18 => {}
            19 => {}
            20 => {}
            21 => {}
            22 => {}
            23 => {}
            24 => {}
            25 => {}
            _ => {}
        },
        2021 => match date.1 {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            9 => {}
            10 => {}
            11 => {}
            12 => {}
            13 => {}
            14 => {}
            15 => {}
            16 => {}
            17 => {}
            18 => {}
            19 => {}
            20 => {}
            21 => {}
            22 => {}
            23 => {}
            24 => {}
            25 => {}
            _ => {}
        },
        2022 => match date.1 {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            9 => {}
            10 => {}
            11 => {}
            12 => {}
            13 => {}
            14 => {}
            15 => {}
            16 => {}
            17 => {}
            18 => {}
            19 => {}
            20 => {}
            21 => {}
            22 => {}
            23 => {}
            24 => {}
            25 => {}
            _ => {}
        },
        2023 => match date.1 {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            9 => {}
            10 => {}
            11 => {}
            12 => {}
            13 => {}
            14 => {}
            15 => {}
            16 => {}
            17 => {}
            18 => {}
            19 => {}
            20 => {}
            21 => {}
            22 => {}
            23 => {}
            24 => {}
            25 => {}
            _ => {}
        },
        2024 => match date.1 {
            1 => {
                let mut nums_a: Vec<i32> = Vec::new();
                let mut nums_b: Vec<i32> = Vec::new();

                for line in input.lines() {
                    let mut line_iter = line.split(" ");
                    nums_a.push(line_iter.next().unwrap().parse().unwrap());
                    nums_b.push(line_iter.last().unwrap().parse().unwrap());
                }

                nums_a.sort_unstable();
                nums_b.sort_unstable();

                for num in nums_a.iter().zip(nums_b.clone()) {
                    answers.0 += (num.0 - num.1).abs();
                    answers.1 += num.0 * nums_b.iter().filter(|&x| x == num.0).count() as i32;
                }
            }
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            9 => {}
            10 => {}
            11 => {}
            12 => {}
            13 => {}
            14 => {}
            15 => {}
            16 => {}
            17 => {}
            18 => {}
            19 => {}
            20 => {}
            21 => {}
            22 => {}
            23 => {}
            24 => {}
            25 => {}
            _ => {}
        },
        2025 => match date.1 {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            9 => {}
            10 => {}
            11 => {}
            12 => {}
            _ => {}
        },
        _ => {}
    }
    (answers.0.to_string(), answers.1.to_string())
}
