# US Time Parser

The US Time Parser is a Rust-based utility designed to parse various time components from input strings. It utilizes the `pest` crate for parsing grammar and `clap` for handling command-line arguments.

[us_time_parser Crate on crates.io](https://crates.io/crates/us_time_parser)

## Parsing Components

The parser can extract the following time components:

- **Complete Time:** Parses complete time strings in the format `HH:MM:SS AM/PM`, where `HH` represents hours, `MM` represents minutes, `SS` represents seconds (optional), and `AM/PM` represents the time of day.
- **Hour:** Parses hours in a 24-hour format (00-23).
- **Minute:** Parses minutes in the range of 00-59.
- **Second:** Parses seconds in the range of 00-59 (optional in the complete time format).
- **AM/PM:** Parses the time of day indicator (AM or PM in various formats).

## Parsing Process

The parsing process involves defining parsing rules in the `grammar.pest` file using the Pest parser generator. Each time component has its specific grammar rules defined, allowing the parser to identify and extract the relevant parts from the input string.

The Rust functions in `lib.rs` utilize the defined parsing rules to parse specific time components. These functions return the parsed components as iterators of Pest tokens, allowing further processing or validation of the parsed data.

## Usage

### Incorporating into Your Rust Project

To use the US Time Parser utility in your Rust project, follow these steps:

1. **Add as Dependency:**

    Include the `us_time_parser` crate as a dependency in your `Cargo.toml` file:

    ```toml
    [dependencies]
    us_time_parser = "x.x.x"  # Replace "x.x.x" with the latest version
    ```

2. **Parsing Time Components:**

    Import the `us_time_parser` crate into your project:

    ```rust
    use us_time_parser::*;
    ```

    You can now utilize the parsing functions provided by the `us_time_parser` crate to extract specific time components from strings in your project.

    ```rust
    let parsed_hour = parse_hour("15");
    let parsed_minute = parse_minute("45");
    // Handle the parsed components as needed
    ```

### Command-line Interface

The US Time Parser utility also provides a command-line interface (CLI) for interactive parsing:

1. **Install Dependencies:**

    Ensure you have Rust installed on your system.

2. **Clone the Repository:**

    Clone the Time Parser repository to your local machine:

    ```bash
    git clone https://github.com/yourusername/your-timestamp-parser.git
    ```

3. **Build and Run:**

    Navigate to the project directory and build the project:

    ```bash
    cd your-timestamp-parser
    cargo build --release
    ```

4. **Command Usage:**

    Run the utility with various subcommands to parse different time components:

    ```bash
    # Parse complete time
    target/release/your-time-parser parse_time "12:45 PM"

    # Parse hour
    target/release/your-time-parser parse_hour "14"

    # Parse minute
    target/release/your-time-parser parse_minute "30"

    # Parse second
    target/release/your-time-parser parse_second "55"

    # Parse AM/PM
    target/release/your-time-parser parse_am_pm "AM"
    ```

    Replace `"your-time-parser"` with the name of the executable generated in your project.

### Contribution

Feel free to contribute to the Time Parser utility by opening issues or submitting pull requests on the repository. Your contributions are highly appreciated!
