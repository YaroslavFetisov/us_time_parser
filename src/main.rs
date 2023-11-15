use clap::{App, Arg, SubCommand};
use us_time_parser::*;

fn main() {
    let current_version = env!("CARGO_PKG_VERSION");

    let matches = App::new("Time Parser")
        .version(current_version)
        .author("Yaroslav Fetisov")
        .about("Parses different time components")
        .subcommand(
            SubCommand::with_name("parse_time")
                .about("Parses complete time")
                .arg(
                    Arg::with_name("time")
                        .help("Sets the input time to parse")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("parse_hour")
                .about("Parses hour")
                .arg(
                    Arg::with_name("hour")
                        .help("Sets the input hour to parse")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("parse_minute")
                .about("Parses minute")
                .arg(
                    Arg::with_name("minute")
                        .help("Sets the input minute to parse")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("parse_second")
                .about("Parses second")
                .arg(
                    Arg::with_name("second")
                        .help("Sets the input second to parse")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("parse_am_pm")
                .about("Parses AM/PM")
                .arg(
                    Arg::with_name("am_pm")
                        .help("Sets the input AM/PM to parse")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("help").about("Displays help and credits information"))
        .get_matches();

    match matches.subcommand() {
        ("parse_time", Some(parse_matches)) => {
            let time = parse_matches.value_of("time").unwrap(); // Отримуємо час
            match parse_time(time) {
                Ok(parsed_data) => {
                    println!("Parsed time: {:?}", parsed_data);
                    // Обробка успішного парсингу
                }
                Err(e) => {
                    eprintln!("Error parsing time: {}", e);
                    // Обробка помилки парсингу
                }
            }
        }
        ("parse_hour", Some(parse_matches)) => {
            let hour = parse_matches.value_of("hour").unwrap(); // Отримуємо години
            match parse_hour(hour) {
                Ok(parsed_data) => {
                    println!("Parsed hour: {:?}", parsed_data);
                    // Обробка успішного парсингу
                }
                Err(e) => {
                    eprintln!("Error parsing hour: {}", e);
                    // Обробка помилки парсингу
                }
            }
        }
        ("parse_minute", Some(parse_matches)) => {
            let minute = parse_matches.value_of("minute").unwrap(); // Отримуємо хвилини
            match parse_minute(minute) {
                Ok(parsed_data) => {
                    println!("Parsed minute: {:?}", parsed_data);
                    // Обробка успішного парсингу
                }
                Err(e) => {
                    eprintln!("Error parsing minute: {}", e);
                    // Обробка помилки парсингу
                }
            }
        }
        ("parse_second", Some(parse_matches)) => {
            let second = parse_matches.value_of("second").unwrap(); // Отримуємо секунди
            match parse_second(second) {
                Ok(parsed_data) => {
                    println!("Parsed second: {:?}", parsed_data);
                    // Обробка успішного парсингу
                }
                Err(e) => {
                    eprintln!("Error parsing second: {}", e);
                    // Обробка помилки парсингу
                }
            }
        }
        ("parse_am_pm", Some(parse_matches)) => {
            let am_pm = parse_matches.value_of("am_pm").unwrap(); // Отримуємо AM/PM
            match parse_am_pm(am_pm) {
                Ok(parsed_data) => {
                    println!("Parsed AM/PM: {:?}", parsed_data);
                    // Обробка успішного парсингу
                }
                Err(e) => {
                    eprintln!("Error parsing AM/PM: {}", e);
                    // Обробка помилки парсингу
                }
            }
        }
        ("help", _) => {
            println!("=== Help Information ===");
            println!("This command-line tool parses time from a file.");
            println!("Command usage:");
            println!("  parse -f <file_path> : Parses time from the specified file");
            println!("  help : Displays help and credits information\n");
            println!("{:?}", current_version);
            println!("Author: Yaroslav Fetisov");
        }
        _ => println!("Use 'help' for usage information"),
    }
}
