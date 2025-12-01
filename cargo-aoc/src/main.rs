use colored::Colorize;

fn get_default_help() -> String {
    let mut help = String::new();

    help = format!("{help}{}", "AOC project helper\n\n");
    help = format!("{help}{}", "Usage: ".bold().bright_green());
    help = format!("{help}{}", "cargo aoc ".bold().bright_cyan());
    help = format!("{help}{}", "[COMMAND]\n\n".cyan());
    help = format!("{help}{}", "Options:\n".bold().bright_green());
    help = format!("{help}{}", "    -V, --version".bold().bright_cyan());
    help = format!("{help}{}", "    Print version info and exit\n\n");
    help = format!("{help}{}", "Commands:\n".bold().bright_green());
    help = format!("{help}{}", "    new".bold().bright_cyan());
    help = format!("{help}{}", "    Create an aoc project in a new directory\n");
    help = format!("{help}{}", "    init".bold().bright_cyan());
    help = format!(
        "{help}{}",
        "   Create an aoc project in the current directory\n"
    );
    help = format!("{help}{}", "    year".bold().bright_cyan());
    help = format!("{help}{}", "   Add a new year to the current aoc project\n");
    help = format!("{help}{}", "    day".bold().bright_cyan());
    help = format!(
        "{help}{}",
        "    Add a new day to a year in the current aoc project\n\n"
    );
    help = format!("{help}{}", "See '");
    help = format!("{help}{}", "aoc help ".bold().bright_cyan());
    help = format!("{help}{}", "<command>".cyan());
    help = format!("{help}{}", "' for more information on a specific command.");

    help
}

fn main() {
    let mut args = std::env::args();

    args.next();
    args.next();

    if args.len() == 0 {
        println!("{}", get_default_help());
    } else {
        let cur_arg = args.next().unwrap();
        if cur_arg == "help" {
            if args.len() == 0 {
                println!("{}", get_default_help());
            } else {
                let cur_arg = args.next().unwrap();

                let mut help = String::new();

                if cur_arg == "new" {
                    help = format!("{help}{}", "Create a new AOC project at <path>\n\n");
                    help = format!("{help}{}", "Usage: ".bold().bright_green());
                    help = format!("{help}{}", "cargo aoc new ".bold().bright_cyan());
                    help = format!("{help}{}", "<PATH>\n\n".cyan());
                    help = format!("{help}{}", "Arguments:\n".bold().bright_green());
                    help = format!("{help}{}", "    <PATH>".cyan());
                } else if cur_arg == "init" {
                    help = format!(
                        "{help}{}",
                        "Create a new AOC project in the current directory\n\n"
                    );
                    help = format!("{help}{}", "Usage: ".bold().bright_green());
                    help = format!("{help}{}", "cargo aoc init".bold().bright_cyan());
                } else if cur_arg == "year" {
                    help = format!(
                        "{help}{}",
                        "Add a new year to the AOC project in the current directory\n\n"
                    );
                    help = format!("{help}{}", "Usage: ".bold().bright_green());
                    help = format!("{help}{}", "cargo aoc year ".bold().bright_cyan());
                    help = format!("{help}{}", "<YEAR>\n\n".cyan());
                    help = format!("{help}{}", "Arguments:\n".bold().bright_green());
                    help = format!("{help}{}", "    <YEAR>".cyan());
                } else if cur_arg == "day" {
                    help = format!(
                        "{help}{}",
                        "Add a new day to the specified year in the AOC project in the current directory\n\n"
                    );
                    help = format!("{help}{}", "Usage: ".bold().bright_green());
                    help = format!("{help}{}", "cargo aoc day ".bold().bright_cyan());
                    help = format!("{help}{}", "<YEAR> <DAY>\n\n".cyan());
                    help = format!("{help}{}", "Arguments:\n".bold().bright_green());
                    help = format!("{help}{}", "    <YEAR>\n".cyan());
                    help = format!("{help}{}", "    <DAY>".cyan());
                } else {
                    help = format!("{help}{}", "Unknown command. Run '");
                    help = format!("{help}{}", "cargo aoc help".bold().bright_cyan());
                    help = format!("{help}{}", "' for a list of commands.");
                }

                println!("{help}");
            }
        }
    }
}
