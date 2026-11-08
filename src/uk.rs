#[derive(Copy, Clone, Eq, PartialEq)]
enum Weekday {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

const fn is_leap_year(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

const fn days_in_month(y: i32, m: u32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(y) => 29,
        2 => 28,
        _ => 0,
    }
}

const fn weekday_from_ymd(y: i32, m: u32, d: u32) -> Weekday {
    const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if m < 3 { y - 1 } else { y };
    match (y + y / 4 - y / 100 + y / 400 + T[(m - 1) as usize] + d as i32).rem_euclid(7) {
        0 => Weekday::Sun,
        1 => Weekday::Mon,
        2 => Weekday::Tue,
        3 => Weekday::Wed,
        4 => Weekday::Thu,
        5 => Weekday::Fri,
        _ => Weekday::Sat,
    }
}

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

pub const fn is_bankholiday_ymd(y: i32, m: u32, d: u32) -> bool {
    if y < 1583 || y > 9999 || d < 1 || d > days_in_month(y, m) {
        return false;
    }

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

    const fn new_years_day(m: u32, d: u32) -> bool {
        m == 1 && d == 1
    }
    const fn new_years_sub(m: u32, d: u32) -> bool {
        m == 1 && d <= 3
    }
    const fn early_may(m: u32, d: u32) -> bool {
        m == 5 && d <= 7
    }
    const fn spring(m: u32, d: u32) -> bool {
        m == 5 && 31 - 7 < d
    }
    const fn summer(m: u32, d: u32) -> bool {
        m == 8 && 31 - 7 < d
    }
    const fn christmas_or_boxingday(day: Weekday, m: u32, d: u32) -> bool {
        m == 12
            && match day {
                Weekday::Mon | Weekday::Tue => d >= 25 && d < 29,
                _ => d >= 25 && d < 27,
            }
    }

    let day = weekday_from_ymd(y, m, d);

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
                || (matches!(day, Weekday::Fri) && is_good_friday(y, m, d))
        }
    }
}

pub fn is_bankholiday<T: BankHoliday>(date: &T) -> bool {
    date.is_bankholiday()
}

pub trait BankHoliday {
    fn is_bankholiday(&self) -> bool;
}

impl BankHoliday for (i32, u32, u32) {
    fn is_bankholiday(&self) -> bool {
        is_bankholiday_ymd(self.0, self.1, self.2)
    }
}

impl BankHoliday for computus::Date {
    fn is_bankholiday(&self) -> bool {
        is_bankholiday_ymd(self.year, self.month, self.day)
    }
}

#[cfg(feature = "chrono")]
impl BankHoliday for chrono::NaiveDate {
    fn is_bankholiday(&self) -> bool {
        use chrono::Datelike;
        is_bankholiday_ymd(self.year(), self.month(), self.day())
    }
}

#[cfg(feature = "chrono")]
impl BankHoliday for chrono::NaiveDateTime {
    fn is_bankholiday(&self) -> bool {
        use chrono::Datelike;
        is_bankholiday_ymd(self.year(), self.month(), self.day())
    }
}

#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> BankHoliday for chrono::DateTime<Tz> {
    fn is_bankholiday(&self) -> bool {
        use chrono::Datelike;
        is_bankholiday_ymd(self.year(), self.month(), self.day())
    }
}

#[cfg(feature = "chrono")]
#[allow(deprecated)]
impl<Tz: chrono::TimeZone> BankHoliday for chrono::Date<Tz> {
    fn is_bankholiday(&self) -> bool {
        use chrono::Datelike;
        is_bankholiday_ymd(self.year(), self.month(), self.day())
    }
}

#[cfg(feature = "jiff")]
impl BankHoliday for jiff::civil::Date {
    fn is_bankholiday(&self) -> bool {
        is_bankholiday_ymd(self.year() as i32, self.month() as u32, self.day() as u32)
    }
}

#[cfg(feature = "jiff")]
impl BankHoliday for jiff::civil::DateTime {
    fn is_bankholiday(&self) -> bool {
        is_bankholiday_ymd(self.year() as i32, self.month() as u32, self.day() as u32)
    }
}

#[cfg(feature = "jiff")]
impl BankHoliday for jiff::Zoned {
    fn is_bankholiday(&self) -> bool {
        is_bankholiday_ymd(self.year() as i32, self.month() as u32, self.day() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($name:ident, $year:expr, $dates:expr) => {
            #[test]
            fn $name() {
                let mut bhs = [(0, 0); 16];
                let mut len = 0;
                for m in 1..=12 {
                    for d in 1..=days_in_month($year, m) {
                        let is_bh = is_bankholiday_ymd($year, m, d);
                        #[cfg(feature = "chrono")]
                        {
                            let date = chrono::NaiveDate::from_ymd_opt($year, m, d).unwrap();
                            assert_eq!(is_bh, date.is_bankholiday());
                        }
                        #[cfg(feature = "jiff")]
                        {
                            let date = jiff::civil::Date::new($year, m as i8, d as i8).unwrap();
                            assert_eq!(is_bh, date.is_bankholiday());
                        }
                        if is_bh {
                            bhs[len] = (d, m);
                            len += 1;
                        }
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
