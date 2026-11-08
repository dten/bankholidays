//! ### Overview
//!
//! Calculate Bank Holidays for England & Wales in Rust.
//!
//! ### Features
//!
//! Optional features:
//!
//! - [`chrono`][]: Enable checking `chrono` date/datetime types via [`uk::BankHoliday`]
//! - [`jiff`][]: Enable checking `jiff` date/datetime types via [`uk::BankHoliday`]
//!
//! ### Example
//!
//! ```rust
//! use bankholidays::uk::{is_bankholiday_ymd, BankHoliday};
//!
//! assert!(const { is_bankholiday_ymd(2024, 12, 25) });
//! assert!((2024, 12, 25).is_bankholiday());
//! assert!(!(2024, 12, 24).is_bankholiday());
//!
//! // With `chrono` feature
//! #[cfg(feature = "chrono")] {
//!     use chrono::NaiveDate;
//!     assert!(NaiveDate::from_ymd_opt(2024, 12, 25).unwrap().is_bankholiday());
//! }
//! // With `jiff` feature
//! #[cfg(feature = "jiff")] {
//!     use jiff::civil::Date;
//!     assert!(Date::new(2024, 12, 25).unwrap().is_bankholiday());
//! }
//! ```
#![no_std]
#![allow(clippy::zero_prefixed_literal)]

pub mod uk;
