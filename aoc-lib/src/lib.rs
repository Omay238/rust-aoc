pub fn get_input(year: String, day: String, session: Option<String>) -> String {
    let session = session.unwrap_or(std::env::var("SESSION").unwrap());
    ureq::get(format!("https://adventofcode.com/{}/day/{}/input", year, day).as_str())
        .header("Cookie", format!("session={}", session))
        .call()
        .unwrap()
        .body_mut()
        .read_to_string()
        .unwrap()
}
