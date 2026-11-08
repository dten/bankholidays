//! ### Overview
//!
//! Calculate Bank Holidays for England & Wales in Rust.
//!
//! ### Example
//!
//! ```rust
//! use bankholidays::uk::BankHoliday;
//! use chrono::NaiveDate;
//!
//! assert!(NaiveDate::from_ymd_opt(2024, 12, 25).unwrap().is_bankholiday());
//! assert!(!NaiveDate::from_ymd_opt(2024, 12, 24).unwrap().is_bankholiday());
//! ```
#![no_std]
#![allow(clippy::zero_prefixed_literal)]

pub mod uk;
