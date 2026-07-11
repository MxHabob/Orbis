//! Finance domain operations — the single source of truth.
//!
//! Covers: Ledger, Treasury, Fees, Payments, Discounts, Currencies, Journal Entries.

pub mod fee_assigner;
pub mod ledger;
pub mod treasury;
pub mod fee_template;
pub mod student_fee;
pub mod payment;
pub mod discount;
pub mod currency;
pub mod journal;
