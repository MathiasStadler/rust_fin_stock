use chrono::prelude::*;
use dateparser::parse;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        parse("6:15pm UTC")?,
        Utc::now().date_naive().and_time(
            NaiveTime::from_hms_opt(18, 15, 0),
        ).unwrap(),
    );
    Ok(())
}