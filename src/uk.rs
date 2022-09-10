use chrono::{NaiveDate, Datelike, Weekday};
use computus;

fn easter_ordinal(y: i32) -> u32 {
    let easter = computus::gregorian(y).expect("computus error");
    NaiveDate::from_ymd(y, easter.month, easter.day).ordinal()
}

pub fn is_bankholiday<T: Datelike>(date: &T) -> bool {

    let day = date.weekday();
    let (y, m, d) = (date.year(), date.month(), date.day());

    // Special cases
    match (y, m, d) {
        (1995, 05, 01) => return false, // Moved for VE Day
        (1995, 05, 08) => return true,
        (1999, 12, 31) => return true,  // Extra for Millennium
        (2002, 05, 27) => return false, // Moved for Jubilee
        (2002, 06, 03) => return true,
        (2002, 06, 04) => return true,  // Extra For Jubilee
        (2011, 04, 29) => return true,  // Extra For Royal Wedding
        (2012, 05, 28) => return false, // Moved for Jubilee
        (2012, 06, 04) => return true,
        (2012, 06, 05) => return true,  // Extra For Jubilee
        (2020, 05, 04) => return false, // Move for VE Day
        (2020, 05, 08) => return true,
        (2022, 05, 30) => return false, // Move for Jubilee
        (2022, 06, 02) => return true,
        (2022, 06, 03) => return true,  // Extra for Jubilee
        (2022, 09, 19) => return true,  // Extra for QE2 funeral
        _ => {}
    }

    let new_years_day = |m, d| m == 1 && d == 1;
    let new_years_sub = |m, d| m == 1 && d <= 3;
    let early_may = |m, d| m == 5 && d <= 7;
    let spring = |m, d| m == 5 && 31 - 7 < d;
    let summer = |m, d| m == 8 && 31 - 7 < d;
    let christmas_or_boxingday = |day, m, d| {
        m == 12 &&
        match day {
            Weekday::Mon | Weekday::Tue => d >= 25 && d < 29,
            _ => d >= 25 && d < 27,
        }
    };

    match day {
        Weekday::Sat | Weekday::Sun => false,
        Weekday::Mon => {
            new_years_sub(m, d) || early_may(m, d) || spring(m, d) || summer(m, d) ||
            christmas_or_boxingday(day, m, d) ||
            ((m == 3 || m == 4) && easter_ordinal(y) + 1 == date.ordinal())
        }
        _ => {
            new_years_day(m, d) || christmas_or_boxingday(day, m, d) ||
            (day == Weekday::Fri && (m == 3 || m == 4) && easter_ordinal(y) == date.ordinal() + 2)
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

    macro_rules! test {
        ($name:ident, $year:expr, $dates:expr) => {
            #[test]
            fn $name() {
                let ymd = |y, m, d| NaiveDate::from_ymd(y, m, d);
                let jan1 = ymd($year, 1, 1);
                let days = if NaiveDate::from_ymd_opt($year, 2, 29).is_some() {
                    366
                } else {
                    365
                };
                for i in 0..days {
                    let date = jan1 + Duration::days(i);
                    let holiday = date.is_bankholiday();
                    let expected = $dates.contains(&(date.day(), date.month()));
                    assert!(expected == holiday,
                            format!("Expected {} for {} but got {}", expected, date, holiday));
                    assert_eq!(is_bankholiday(&date), holiday);
                }
            }
        }
    }

    test!(year_1999, 1999,
          [(1, 1), (2, 4), (5, 4), (3, 5), (31, 5), (30, 8), (27, 12), (28, 12), (31, 12)]);

    test!(year_2002, 2002,
          [(1, 1), (29, 3), (1, 4), (6, 5), (3, 6), (4, 6), (26, 8), (25, 12), (26, 12)]);

    test!(year_2012, 2012,
          [(2, 1), (6, 4), (9, 4), (7, 5), (4, 6), (5, 6), (27, 8), (25, 12), (26, 12)]);

    test!(year_2013, 2013,
          [(1, 1), (29, 3), (1, 4), (6, 5), (27, 5), (26, 8), (25, 12), (26, 12)]);

    test!(year_2014, 2014,
          [(1, 1), (18, 4), (21, 4), (5, 5), (26, 5), (25, 8), (25, 12), (26, 12)]);

    test!(year_2015, 2015,
          [(1, 1), (3, 4), (6, 4), (4, 5), (25, 5), (31, 8), (25, 12), (28, 12)]);

    test!(year_2016, 2016,
          [(1, 1), (25, 3), (28, 3), (2, 5), (30, 5), (29, 8), (26, 12), (27, 12)]);

    test!(year_2017, 2017,
          [(2, 1), (14, 4), (17, 4), (1, 5), (29, 5), (28, 8), (25, 12), (26, 12)]);

    test!(year_2018, 2018,
          [(1, 1), (30, 3), (2, 4), (7, 5), (28, 5), (27, 8), (25, 12), (26, 12)]);

    test!(year_2020, 2020,
          [(1, 1), (10, 4), (13, 4), (8, 5), (25, 5), (31, 8), (25, 12), (28, 12)]);

    test!(year_2022, 2022,
          [(3, 1), (15, 4), (18, 4), (2, 5), (2, 6), (3, 6), (29, 8), (19, 9), (26, 12), (27, 12)]);

}
