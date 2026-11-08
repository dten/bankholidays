# bankholidays

[![crates.io](https://img.shields.io/crates/v/bankholidays.svg)](https://crates.io/crates/bankholidays)
[![Documentation](https://docs.rs/bankholidays/badge.svg)](https://docs.rs/bankholidays)

Bank Holidays for England &amp; Wales in Rust

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
bankholidays = "1.3.1"
```

```rust
use bankholidays::uk::{is_bankholiday_ymd, BankHoliday};
use chrono::NaiveDate;

assert!(const { is_bankholiday_ymd(2024, 12, 25) });
assert!(NaiveDate::from_ymd_opt(2024, 12, 25).unwrap().is_bankholiday());
assert!(!NaiveDate::from_ymd_opt(2024, 12, 24).unwrap().is_bankholiday());
```
