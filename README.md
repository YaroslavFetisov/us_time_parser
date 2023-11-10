# Date Parser

This is a Rust library for parsing date strings in the "YYYY-MM-DD" format. It utilizes the `pest` parser generator to provide a flexible and extensible way to parse date strings. The parsed date components (year, month, and day) are then validated to ensure they conform to the expected ranges.

## Parsing Process

The library performs the following parsing and validation steps:

1. The input string is expected to be in the "YYYY-MM-DD" format.
2. It checks the length of the input string to ensure it's exactly 10 characters.
3. It uses the `pest` parser to tokenize the input string and extract year, month, and day components.
4. Year, month, and day values are parsed and checked for validity.
5. The year is expected to be in the range of 1000 to 9999.
6. The month is expected to be in the range of 1 to 12.
7. The day is checked to be within the valid range for the given month and year.

## Usage

To use this library, you can include it as a dependency in your Rust project's `Cargo.toml` file. Once added, you can call the `parse_date` function to parse and validate date strings.

```rust
extern crate my_date_parser;

use my_date_parser::parse_date;

fn main() {
    let date_str = "2023-11-08";
    match parse_date(date_str) {
        Ok(()) => println!("Date is valid"),
        Err(err) => eprintln!("Error: {}", err),
    }
}
