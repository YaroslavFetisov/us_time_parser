use us_time_parser::*;

fn main() {
    let date_str = "2023-11-08";
    match parse_date(date_str) {
        Ok(()) => println!("Date is valid"),
        Err(err) => eprintln!("Error: {}", err),
    }
}