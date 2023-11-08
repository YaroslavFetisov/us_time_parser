extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use anyhow::{anyhow, Result};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DateParser;

pub fn parse_date(input: &str) -> Result<()> {
  if input.len() != 10 {
      return Err(anyhow!("Invalid date length"));
  }

  let pairs = DateParser::parse(Rule::date, input)?;
  let mut year_value: Option<u32> = None;
  let mut month_value: Option<u32> = None;

  for pair in pairs {
      match pair.as_rule() {
          Rule::year => {
              year_value = Some(pair.as_str().parse().map_err(|_| anyhow!("Failed to parse year as u32"))?);
              if let Some(year) = year_value {
                  if year < 1000 || year > 9999 {
                      return Err(anyhow!("Year must be between 1000 and 9999"));
                  }
              }
          }
          Rule::month => {
              month_value = Some(pair.as_str().parse().map_err(|_| anyhow!("Failed to parse month as u32"))?);
              if let Some(month) = month_value {
                  if month < 1 || month > 12 {
                      return Err(anyhow!("Month must be between 1 and 12"));
                  }
              }
          }
          Rule::day => {
              let day_value: u32 = pair.as_str().parse().map_err(|_| anyhow!("Failed to parse day as u32"))?;
              if let (Some(year), Some(month)) = (year_value, month_value) {
                  let max_day = max_day_in_month(year, month);
                  if day_value < 1 || day_value > max_day {
                      return Err(anyhow!("Day must be between 1 and {}", max_day));
                  }
              }
          }
          _ => {}
      }
  }

  Ok(())
}


fn max_day_in_month(year: u32, month: u32) -> u32 {
  match month {
      1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
      4 | 6 | 9 | 11 => 30,
      2 => {
          if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
              29
          } else {
              28
          }
      },
      _ => 0,
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_date() {
        let input = "2023-11-08";
        assert!(parse_date(input).is_ok());
    }
    
    #[test]
    fn test_invalid_date() {
        let input = "2023-11-0802";
        assert!(parse_date(input).is_err());
    }
    #[test]
    fn test_empty_input() {
        let input = "";
        assert!(parse_date(input).is_err());
    }
    #[test]
    fn test_valid_max_days_non_leap_year() {
        let input = "2023-02-29";
        assert!(parse_date(input).is_err());
    }

    #[test]
    fn test_valid_max_days_leap_year() {
        let input = "2024-02-29";
        assert!(parse_date(input).is_ok());
    }

    #[test]
    fn test_valid_max_days_30_days_month() {
        let input = "2023-04-31";
        assert!(parse_date(input).is_err());
    }
}
