//! Finance domain events.

pub mod event_types {
    pub const PAYMENT_RECORDED: &str = "payment.recorded";
    pub const PAYMENT_REVERSED: &str = "payment.reversed";
    pub const FEE_ASSIGNED: &str = "fee.assigned";
    pub const FEE_WAIVED: &str = "fee.waived";
    pub const PAYMENT_OVERDUE: &str = "payment.overdue";
    pub const TREASURY_DEPOSIT: &str = "treasury.deposit";
    pub const TREASURY_WITHDRAWAL: &str = "treasury.withdrawal";
}
