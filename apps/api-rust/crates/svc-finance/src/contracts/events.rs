//! Finance domain events.

pub mod event_types {
    pub const PAYMENT_RECORDED: &str = "payment.recorded";
    pub const PAYMENT_REVERSED: &str = "payment.reversed";
    pub const FEE_ASSIGNED: &str = "fee.assigned";
    pub const FEE_WAIVED: &str = "fee.waived";
    pub const PAYMENT_OVERDUE: &str = "payment.overdue";
    pub const TREASURY_DEPOSIT: &str = "treasury.deposit";
    pub const TREASURY_WITHDRAWAL: &str = "treasury.withdrawal";
    pub const ACCOUNT_CREATED: &str = "account.created";
    pub const JOURNAL_ENTRY_POSTED: &str = "journal_entry.posted";
    pub const FEE_TEMPLATE_CREATED: &str = "fee_template.created";
    pub const FEE_TEMPLATE_UPDATED: &str = "fee_template.updated";
    pub const TREASURY_CREATED: &str = "treasury.created";
    pub const DISCOUNT_TEMPLATE_CREATED: &str = "discount_template.created";
}
