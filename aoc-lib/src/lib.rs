use ureq;

fn get_name() -> (String, String) {
    let filename = std::env::current_exe().unwrap();
    let mut date_string = filename
        .to_str()
        .unwrap()
        .split("aoc-")
        .last()
        .unwrap()
        .split("-");
    let year = date_string.next().unwrap();
    let day = date_string.next().unwrap();
    (String::from(year), String::from(day))
}

pub fn print_day(part: u8) {
    let (year, day) = get_name();

    println!("{} day {} part {}", year, day, part)
}

pub fn get_input() -> String {
    let (year, day) = get_name();
    ureq::get(format!("https://adventofcode.com/{}/day/{}/input", year, day).as_str())
        .header(
            "Cookie",
            format!("session={}", std::env::var("SESSION").unwrap()),
        )
        .call()
        .unwrap()
        .body_mut()
        .read_to_string()
        .unwrap()
}
