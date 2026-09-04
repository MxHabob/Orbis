//! Orbis Finance Domain — Ledger accounts, fee structures, payments, and cash accounts.
//!
//! This crate owns:
//! - **Fee Templates**: Configurable school fee blueprints
//! - **Student Fees**: Allocated fee structures due by students
//! - **Discounts**: Fee deduction configurations and assignments
//! - **Payments**: Payments received for fees
//! - **Cash accounts**: Bank accounts and cash registries represented in the ledger
//! - **Ledger Accounts**: Chart of accounts, journal entries

pub mod entities;
pub mod params;
pub mod repository;
pub mod validation;

pub mod contracts;
pub mod ops;
