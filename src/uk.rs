use chrono::{Datelike, Weekday};

// Day number counting from March 1 (March has 31 days, so April 1 is 32)
const fn march_day(m: u32, d: u32) -> u32 {
    if m == 4 {
        31 + d
    } else {
        d
    }
}

const fn is_easter_monday(y: i32, m: u32, d: u32) -> bool {
    (m == 3 || m == 4)
        && match computus::gregorian(y) {
            Ok(easter) => march_day(m, d) == march_day(easter.month, easter.day) + 1,
            Err(_) => false,
        }
}

const fn is_good_friday(y: i32, m: u32, d: u32) -> bool {
    (m == 3 || m == 4)
        && match computus::gregorian(y) {
            Ok(easter) => march_day(m, d) + 2 == march_day(easter.month, easter.day),
            Err(_) => false,
        }
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
        (2022, 06, 03) => return true, // Extra for Jubilee
        (2022, 09, 19) => return true, // Extra for QE2 funeral
        (2023, 05, 08) => return true, // Extra for C3 coronation
        _ => {}
    }

    let new_years_day = |m, d| m == 1 && d == 1;
    let new_years_sub = |m, d| m == 1 && d <= 3;
    let early_may = |m, d| m == 5 && d <= 7;
    let spring = |m, d| m == 5 && 31 - 7 < d;
    let summer = |m, d| m == 8 && 31 - 7 < d;
    let christmas_or_boxingday = |day, m, d| {
        m == 12
            && match day {
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
                || is_easter_monday(y, m, d)
        }
        _ => {
            new_years_day(m, d)
                || christmas_or_boxingday(day, m, d)
                || (day == Weekday::Fri && is_good_friday(y, m, d))
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
    use chrono::{Datelike, Duration, NaiveDate};

    macro_rules! test {
        ($name:ident, $year:expr, $dates:expr) => {
            #[test]
            fn $name() {
                let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
                let jan1 = ymd($year, 1, 1);
                let days = if NaiveDate::from_ymd_opt($year, 2, 29).is_some() {
                    366
                } else {
                    365
                };
                let mut bhs = [(0, 0); 16];
                let mut len = 0;
                for date in (0..days).map(|i| jan1 + Duration::days(i)) {
                    if date.is_bankholiday() {
                        bhs[len] = (date.day(), date.month());
                        len += 1;
                    }
                }
                assert_eq!($dates, bhs[..len]);
            }
        };
    }

    test!(
        year_1995,
        1995,
        [
            (2, 1),
            (14, 4),
            (17, 4),
            (8, 5),
            (29, 5),
            (28, 8),
            (25, 12),
            (26, 12)
        ]
    );

    test!(
        year_1999,
        1999,
        [
            (1, 1),
            (2, 4),
            (5, 4),
            (3, 5),
            (31, 5),
            (30, 8),
            (27, 12),
            (28, 12),
            (31, 12)
        ]
    );

    test!(
        year_2002,
        2002,
        [
            (1, 1),
            (29, 3),
            (1, 4),
            (6, 5),
            (3, 6),
            (4, 6),
            (26, 8),
            (25, 12),
            (26, 12)
        ]
    );

    test!(
        year_2011,
        2011,
        [
            (3, 1),
            (22, 4),
            (25, 4),
            (29, 4),
            (2, 5),
            (30, 5),
            (29, 8),
            (26, 12),
            (27, 12)
        ]
    );

    test!(
        year_2012,
        2012,
        [
            (2, 1),
            (6, 4),
            (9, 4),
            (7, 5),
            (4, 6),
            (5, 6),
            (27, 8),
            (25, 12),
            (26, 12)
        ]
    );

    test!(
        year_2013,
        2013,
        [
            (1, 1),
            (29, 3),
            (1, 4),
            (6, 5),
            (27, 5),
            (26, 8),
            (25, 12),
            (26, 12)
        ]
    );

    test!(
        year_2014,
        2014,
        [
            (1, 1),
            (18, 4),
            (21, 4),
            (5, 5),
            (26, 5),
            (25, 8),
            (25, 12),
            (26, 12)
        ]
    );

    test!(
        year_2015,
        2015,
        [
            (1, 1),
            (3, 4),
            (6, 4),
            (4, 5),
            (25, 5),
            (31, 8),
            (25, 12),
            (28, 12)
        ]
    );

    test!(
        year_2016,
        2016,
        [
            (1, 1),
            (25, 3),
            (28, 3),
            (2, 5),
            (30, 5),
            (29, 8),
            (26, 12),
            (27, 12)
        ]
    );

    test!(
        year_2017,
        2017,
        [
            (2, 1),
            (14, 4),
            (17, 4),
            (1, 5),
            (29, 5),
            (28, 8),
            (25, 12),
            (26, 12)
        ]
    );

    test!(
        year_2018,
        2018,
        [
            (1, 1),
            (30, 3),
            (2, 4),
            (7, 5),
            (28, 5),
            (27, 8),
            (25, 12),
            (26, 12)
        ]
    );

    test!(
        year_2019,
        2019,
        [
            (1, 1),
            (19, 4),
            (22, 4),
            (6, 5),
            (27, 5),
            (26, 8),
            (25, 12),
            (26, 12)
        ]
    );

    test!(
        year_2020,
        2020,
        [
            (1, 1),
            (10, 4),
            (13, 4),
            (8, 5),
            (25, 5),
            (31, 8),
            (25, 12),
            (28, 12)
        ]
    );

    test!(
        year_2021,
        2021,
        [
            (1, 1),
            (2, 4),
            (5, 4),
            (3, 5),
            (31, 5),
            (30, 8),
            (27, 12),
            (28, 12)
        ]
    );

    test!(
        year_2022,
        2022,
        [
            (3, 1),
            (15, 4),
            (18, 4),
            (2, 5),
            (2, 6),
            (3, 6),
            (29, 8),
            (19, 9),
            (26, 12),
            (27, 12)
        ]
    );

    test!(
        year_2023,
        2023,
        [
            (2, 1),
            (7, 4),
            (10, 4),
            (1, 5),
            (8, 5),
            (29, 5),
            (28, 8),
            (25, 12),
            (26, 12)
        ]
    );

    test!(
        year_2024,
        2024,
        [
            (1, 1),
            (29, 3),
            (1, 4),
            (6, 5),
            (27, 5),
            (26, 8),
            (25, 12),
            (26, 12)
        ]
    );

    test!(
        year_2025,
        2025,
        [
            (1, 1),
            (18, 4),
            (21, 4),
            (5, 5),
            (26, 5),
            (25, 8),
            (25, 12),
            (26, 12)
        ]
    );

    test!(
        year_2026,
        2026,
        [
            (1, 1),
            (3, 4),
            (6, 4),
            (4, 5),
            (25, 5),
            (31, 8),
            (25, 12),
            (28, 12)
        ]
    );

    test!(
        year_2027,
        2027,
        [
            (1, 1),
            (26, 3),
            (29, 3),
            (3, 5),
            (31, 5),
            (30, 8),
            (27, 12),
            (28, 12)
        ]
    );

    test!(
        year_2028,
        2028,
        [
            (3, 1),
            (14, 4),
            (17, 4),
            (1, 5),
            (29, 5),
            (28, 8),
            (25, 12),
            (26, 12)
        ]
    );
}
