//! Rust port of the Acme legacy billing logic (`com.acme.billing`).
//!
//! Money is held as integer cents and rounded half to even, like the Java
//! `Money` class. Dates are UTC calendar days. See `PORTING.md` at the repo root.

pub mod aging_report;
pub mod billing_run;
pub mod decimal;
pub mod due_date_calculator;
pub mod golden_cli;
pub mod invoice;
pub mod invoice_formatter;
pub mod invoice_xml;
pub mod java_random;
pub mod line_item;
pub mod money;
pub mod payment_token_codec;
pub mod tax_calculator;

pub use aging_report::AgingReport;
pub use decimal::Decimal;
pub use due_date_calculator::DueDateCalculator;
pub use invoice::Invoice;
pub use invoice_formatter::InvoiceFormatter;
pub use invoice_xml::InvoiceXml;
pub use java_random::JavaRandom;
pub use line_item::LineItem;
pub use money::{CurrencyMismatch, Money};
pub use payment_token_codec::PaymentTokenCodec;
pub use tax_calculator::TaxCalculator;
