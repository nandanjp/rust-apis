use std::fmt::Display;

use chrono::{DateTime, NaiveDateTime, ParseError, Utc};

pub const TIME_FORMAT: &str = "%Y-%m-%d";
pub fn string_to_datetime<E: Clone + Display, F: Fn(ParseError) -> E>(
    date: &String,
    f: F,
) -> Result<DateTime<Utc>, E> {
    let date = NaiveDateTime::parse_from_str(date, TIME_FORMAT).map_err(f)?;
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(date, Utc))
}
