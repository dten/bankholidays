use chrono::{Datelike};

pub fn is_bankholiday<T: Datelike>(_: &T) -> bool {
    unimplemented!();
}

pub trait BankHoliday {
    fn is_bankholiday(&self) -> bool;
}

impl<T: Datelike> BankHoliday for T {
    fn is_bankholiday(&self) -> bool {
        self::is_bankholiday(self)
    }
}