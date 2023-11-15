use anyhow::*;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct TimeParser;

// Parse the input string as a time using the 'time' rule
pub fn parse_time(input: &str) -> anyhow::Result<pest::iterators::Pairs<'_, Rule>> {
    let parse = TimeParser::parse(Rule::time, input)?;
    Ok(parse)
}

// Parse the input string as an hour using the 'hour' rule
pub fn parse_hour(input: &str) -> anyhow::Result<pest::iterators::Pairs<'_, Rule>> {
    let parse = TimeParser::parse(Rule::hour, input)?;
    Ok(parse)
}

// Parse the input string as a minute using the 'minute' rule
pub fn parse_minute(input: &str) -> anyhow::Result<pest::iterators::Pairs<'_, Rule>> {
    let parse = TimeParser::parse(Rule::minute, input)?;
    Ok(parse)
}

// Parse the input string as a second using the 'second' rule
pub fn parse_second(input: &str) -> anyhow::Result<pest::iterators::Pairs<'_, Rule>> {
    let parse = TimeParser::parse(Rule::second, input)?;
    Ok(parse)
}

// Parse the input string as AM/PM using the 'am_pm' rule
pub fn parse_am_pm(input: &str) -> anyhow::Result<pest::iterators::Pairs<'_, Rule>> {
    let parse = TimeParser::parse(Rule::am_pm, input)?;
    Ok(parse)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_time_valid() {
        assert!(parse_time("12:34").is_ok());
        assert!(parse_time("23:59:59").is_ok());
        assert!(parse_time("11:45 PM").is_ok());
        assert!(parse_time("11:45a.m.").is_ok());
    }

    #[test]
    fn test_parse_time_invalid() {
        assert!(parse_time("25:00").is_err());
        assert!(parse_time("12:60:00").is_err());
        assert!(parse_time("3:45 BM").is_err());
    }

    #[test]
    fn test_parse_hour_valid() {
        assert!(parse_hour("12").is_ok());
        assert!(parse_hour("00").is_ok());
        assert!(parse_hour("23").is_ok());
    }

    #[test]
    fn test_parse_hour_invalid() {
        assert!(parse_hour("24").is_err());
        assert!(parse_hour("9").is_err());
        assert!(parse_hour("60").is_err());
    }

    #[test]
    fn test_parse_minute_valid() {
        assert!(parse_minute("00").is_ok());
        assert!(parse_minute("59").is_ok());
        assert!(parse_minute("05").is_ok());
    }

    #[test]
    fn test_parse_minute_invalid() {
        assert!(parse_minute("60").is_err());
        assert!(parse_minute("abcd").is_err());
    }

    #[test]
    fn test_parse_second_valid() {
        assert!(parse_second("00").is_ok());
        assert!(parse_second("59").is_ok());
        assert!(parse_second("05").is_ok());
    }

    #[test]
    fn test_parse_second_invalid() {
        assert!(parse_second("60").is_err());
        assert!(parse_second("abcd").is_err());
    }

    #[test]
    fn test_parse_am_pm_valid() {
        assert!(parse_am_pm("AM").is_ok());
        assert!(parse_am_pm("PM").is_ok());
        assert!(parse_am_pm("am").is_ok());
        assert!(parse_am_pm("pm").is_ok());
    }

    #[test]
    fn test_parse_am_pm_invalid() {
        assert!(parse_am_pm("xxx").is_err());
        assert!(parse_am_pm("noon").is_err());
    }
}
