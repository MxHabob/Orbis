//! Orbis Finance Domain — Ledger accounts, fee structures, payments, and treasuries.
//!
//! This crate owns:
//! - **Fee Templates**: Configurable school fee blueprints
//! - **Student Fees**: Allocated fee structures due by students
//! - **Discounts**: Fee deduction configurations and assignments
//! - **Payments**: Payments received for fees
//! - **Treasuries**: Bank accounts, cash registries
//! - **Ledger Accounts**: Chart of accounts, journal entries

pub mod entities;
pub mod params;
pub mod repository;
pub mod services;

// ?? New Microservice Modules ?????????????????????????????????
pub mod contracts;
pub mod ops;
pub mod validation;

