//! ### Overview
//!
//! Calculate Bank Holidays for England & Wales in Rust.
//!
//! ### Example
//!
//! ```rust
//! use bankholidays::uk::{is_bankholiday_ymd, BankHoliday};
//! use chrono::NaiveDate;
//!
//! assert!(const { is_bankholiday_ymd(2024, 12, 25) });
//! assert!(NaiveDate::from_ymd_opt(2024, 12, 25).unwrap().is_bankholiday());
//! assert!(!NaiveDate::from_ymd_opt(2024, 12, 24).unwrap().is_bankholiday());
//! ```
#![no_std]
#![allow(clippy::zero_prefixed_literal)]

pub mod uk;
