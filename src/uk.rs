use chrono::{NaiveDate, Datelike, Weekday};
use computus;

fn easter_ordinal(y: i32) -> u32 {
    let easter = computus::gregorian::month_day(y);
    NaiveDate::from_ymd(y, easter.0, easter.1).ordinal()
}

pub fn is_bankholiday<T: Datelike>(date: &T) -> bool {
    let day = date.weekday();
    let (y, m, d) = (date.year(), date.month(), date.day());

    let new_years_day = |m, d| m == 1 && d == 1;
    let new_years_sub = |m, d| m == 1 && d <= 3;
    let early_may = |m, d| m == 5 && d <= 3;
    let spring = |m, d| m == 5 && 31 - 7 < d;
    let summer = |m, d| m == 8 && 31 - 7 < d;
    let christmas_or_boxingday = |day, m, d| {
        m == 12 && match day {
            Weekday::Mon | Weekday::Tue => d >= 25 && d < 29,
            _ => d >= 25 && d < 27,
        }
    };

    match day {
        Weekday::Sat | Weekday::Sun => false,
        Weekday::Mon => {
               new_years_sub(m, d)
            || early_may(m, d)
            || spring(m, d)
            || summer(m, d)
            || christmas_or_boxingday(day, m, d)
            || easter_ordinal(y) + 1 == date.ordinal()
        }
        _ => {
               new_years_day(m, d)
            || christmas_or_boxingday(day, m, d)
            || easter_ordinal(y) == date.ordinal() + 2
        }
    }
}

pub trait BankHoliday {
    fn is_bankholiday(&self) -> bool;
}

impl<T: Datelike> BankHoliday for T {
    fn is_bankholiday(&self) -> bool {
        self::is_bankholiday(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Datelike, Duration};

    fn test_year(year: i32, holidays: &[(u32, u32)]) {
        let ymd = |y, m, d| NaiveDate::from_ymd(y, m, d);
        let jan1 = ymd(year, 1, 1);
        let days = if NaiveDate::from_ymd_opt(year, 2, 29).is_some() {
            366
        } else {
            365
        };
        for i in 0..days {
            let date = jan1 + Duration::days(i);
            let holiday = date.is_bankholiday();
            let expected = holidays.contains(&(date.day(), date.month()));
            assert!(expected == holiday,
                    format!("Expected {} for {} but got {}", expected, date, holiday));
            assert_eq!(is_bankholiday(&date), holiday);
        }
    }

    #[test]
    fn test_2016() {
        test_year(2016,
                  &[(1, 1), (25, 3), (28, 3), (2, 5), (30, 5), (29, 8), (26, 12), (27, 12)]);
    }
    #[test]
    fn test_2017() {
        test_year(2017,
                  &[(2, 1), (14, 4), (17, 4), (1, 5), (29, 5), (28, 8), (25, 12), (26, 12)]);
    }
}
