use toml_edit;

// https://nick.groenen.me/notes/recursively-copy-files-in-rust/
fn copy_recursively(
    source: impl AsRef<std::path::Path>,
    destination: impl AsRef<std::path::Path>,
) -> std::io::Result<()> {
    std::fs::create_dir_all(&destination)?;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    copy_recursively(
        std::path::Path::new("aoc-example"),
        std::path::Path::new(&format!("aoc-{}", std::env::args().last().unwrap())),
    )
    .unwrap();

    let cargo_text = std::fs::read_to_string("Cargo.toml").unwrap();
    let mut cargo_toml = cargo_text.parse::<toml_edit::DocumentMut>().unwrap();

    cargo_toml["workspace"]["members"]
        .as_array_mut()
        .unwrap()
        .push(format!("aoc-{}", std::env::args().last().unwrap()));

    std::fs::write("Cargo.toml", cargo_toml.to_string()).unwrap();

    let project_text = std::fs::read_to_string(format!(
        "aoc-{}/Cargo.toml",
        std::env::args().last().unwrap()
    ))
    .unwrap();
    let mut project_toml = project_text.parse::<toml_edit::DocumentMut>().unwrap();

    project_toml["package"]["name"] =
        toml_edit::value(format!("aoc-{}", std::env::args().last().unwrap()));

    std::fs::write(
        format!("aoc-{}/Cargo.toml", std::env::args().last().unwrap()),
        project_toml.to_string(),
    )
    .unwrap();
}
