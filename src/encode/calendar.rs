//! Proleptic Gregorian calendar helpers for rendering CBOR date tags.
//!
//! Dates are limited to years `-262144..=262143`, the range the tool has always supported.

use std::fmt;

const SECONDS_PER_DAY: i64 = 24 * 60 * 60;
const MIN_DAYS_SINCE_EPOCH: i64 = -96_465_658;
const MAX_DAYS_SINCE_EPOCH: i64 = 95_026_601;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ParseError {
    OutOfRange,
    InvalidCharacters,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::OutOfRange => f.write_str("input is out of range"),
            ParseError::InvalidCharacters => f.write_str("input contains invalid characters"),
        }
    }
}

/// Parses an RFC 3339 `full-date` (`YYYY-MM-DD`), additionally allowing a sign and more than four
/// year digits, and returns the number of days since 1970-01-01.
pub(super) fn parse_days_since_epoch(input: &str) -> Result<i64, ParseError> {
    let mut parts = input.rsplitn(3, '-');
    let (Some(day), Some(month), Some(year)) = (parts.next(), parts.next(), parts.next()) else {
        return Err(ParseError::InvalidCharacters);
    };

    let digits = year.strip_prefix(['+', '-']).unwrap_or(year);
    if digits.len() < 4
        || !all_digits(digits)
        || !all_digits_len(month, 2)
        || !all_digits_len(day, 2)
    {
        return Err(ParseError::InvalidCharacters);
    }

    let year: i64 = year
        .strip_prefix('+')
        .unwrap_or(year)
        .parse()
        .map_err(|_| ParseError::OutOfRange)?;
    let month: u8 = month.parse().map_err(|_| ParseError::InvalidCharacters)?;
    let day: u8 = day.parse().map_err(|_| ParseError::InvalidCharacters)?;

    if !(-262_144..=262_143).contains(&year) {
        return Err(ParseError::OutOfRange);
    }
    if !(1..=12).contains(&month) || day < 1 || day > days_in_month(year, month) {
        return Err(ParseError::OutOfRange);
    }

    Ok(days_from_civil(year, month, day))
}

/// Formats a date as `YYYY-MM-DD`, or `None` if it is outside the supported range.
pub(super) fn format_date(days_since_epoch: i64) -> Option<String> {
    if !(MIN_DAYS_SINCE_EPOCH..=MAX_DAYS_SINCE_EPOCH).contains(&days_since_epoch) {
        return None;
    }
    let (year, month, day) = civil_from_days(days_since_epoch);
    Some(format!("{}-{month:02}-{day:02}", Year(year)))
}

/// Formats a UTC timestamp as `YYYY-MM-DDTHH:MM:SS[.fff[fff[fff]]]Z`, or `None` if it is outside
/// the supported range.
pub(super) fn format_datetime(seconds_since_epoch: i64, nanoseconds: u32) -> Option<String> {
    let days = seconds_since_epoch.div_euclid(SECONDS_PER_DAY);
    let seconds_of_day = seconds_since_epoch.rem_euclid(SECONDS_PER_DAY);
    let date = format_date(days)?;
    let (hour, minute, second) = (
        seconds_of_day / 3600,
        seconds_of_day % 3600 / 60,
        seconds_of_day % 60,
    );
    Some(format!(
        "{date}T{hour:02}:{minute:02}:{second:02}{}Z",
        fraction(nanoseconds)
    ))
}

/// Formats the sub-second part with the shortest of 0, 3, 6 or 9 digits that is exact.
pub(super) fn fraction(nanoseconds: u32) -> String {
    let digits = format!("{nanoseconds:09}");
    let exact_digits = if nanoseconds == 0 {
        0
    } else if digits.ends_with("000000") {
        3
    } else if digits.ends_with("000") {
        6
    } else {
        9
    };
    if exact_digits == 0 {
        String::new()
    } else {
        format!(".{}", &digits[..exact_digits])
    }
}

struct Year(i64);

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if (0..=9999).contains(&self.0) {
            write!(f, "{:04}", self.0)
        } else {
            write!(f, "{:+05}", self.0)
        }
    }
}

fn all_digits(s: &str) -> bool {
    s.bytes().all(|b| b.is_ascii_digit())
}

fn all_digits_len(s: &str, len: usize) -> bool {
    s.len() == len && all_digits(s)
}

fn is_leap_year(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: i64, month: u8) -> u8 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

// Days <-> civil date conversions from http://howardhinnant.github.io/date_algorithms.html

fn days_from_civil(year: i64, month: u8, day: u8) -> i64 {
    let (month, day) = (i64::from(month), i64::from(day));
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year.rem_euclid(400);
    let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

fn civil_from_days(days: i64) -> (i64, u8, u8) {
    let days = days + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let mp = (5 * day_of_year + 2) / 153;
    let day = (day_of_year - (153 * mp + 2) / 5 + 1) as u8;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u8;
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_round_trip_over_supported_range() {
        for days in MIN_DAYS_SINCE_EPOCH..=MAX_DAYS_SINCE_EPOCH {
            let (year, month, day) = civil_from_days(days);
            assert_eq!(days_from_civil(year, month, day), days);
        }
    }

    #[test]
    fn range_bounds() {
        assert_eq!(
            parse_days_since_epoch("-262144-01-01"),
            Ok(MIN_DAYS_SINCE_EPOCH)
        );
        assert_eq!(
            parse_days_since_epoch("+262143-12-31"),
            Ok(MAX_DAYS_SINCE_EPOCH)
        );
        assert_eq!(format_date(MIN_DAYS_SINCE_EPOCH).unwrap(), "-262144-01-01");
        assert_eq!(format_date(MAX_DAYS_SINCE_EPOCH).unwrap(), "+262143-12-31");
        assert!(format_date(MIN_DAYS_SINCE_EPOCH - 1).is_none());
        assert!(format_date(MAX_DAYS_SINCE_EPOCH + 1).is_none());
    }

    #[test]
    fn rejects_invalid_dates() {
        assert_eq!(
            parse_days_since_epoch("2023-02-29"),
            Err(ParseError::OutOfRange)
        );
        assert_eq!(parse_days_since_epoch("2024-02-29"), Ok(19_782));
        assert_eq!(
            parse_days_since_epoch("2024-13-01"),
            Err(ParseError::OutOfRange)
        );
        assert_eq!(
            parse_days_since_epoch("24-01-01"),
            Err(ParseError::InvalidCharacters)
        );
        assert_eq!(
            parse_days_since_epoch("nonsense"),
            Err(ParseError::InvalidCharacters)
        );
    }

    #[test]
    fn formats_years_and_fractions() {
        assert_eq!(format_date(-719_528).unwrap(), "0000-01-01");
        assert_eq!(format_date(-719_529).unwrap(), "-0001-12-31");
        assert_eq!(
            format_datetime(-1, 500_000_000).unwrap(),
            "1969-12-31T23:59:59.500Z"
        );
        assert_eq!(
            format_datetime(0, 1_000).unwrap(),
            "1970-01-01T00:00:00.000001Z"
        );
        assert_eq!(
            format_datetime(0, 1).unwrap(),
            "1970-01-01T00:00:00.000000001Z"
        );
        assert_eq!(
            format_datetime(253_402_300_800, 0).unwrap(),
            "+10000-01-01T00:00:00Z"
        );
    }
}
