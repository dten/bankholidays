extern crate chrono;
extern crate computus;

use chrono::{NaiveDate, Datelike};

pub mod uk;

fn easter_ordinal(y: i32) -> u32 {
    let easter = computus::gregorian::month_day(y);
    NaiveDate::from_ymd(y, easter.0, easter.1).ordinal()
}
