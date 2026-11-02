//! Port of `com.acme.billing.InvoiceFormatter`: one CSV row per invoice.

use crate::invoice::Invoice;
use crate::money::{CurrencyMismatch, Money};

pub(crate) const ISO_DATE: &str = "%Y-%m-%d";

#[derive(Debug, Default, Clone, Copy)]
pub struct InvoiceFormatter;

impl InvoiceFormatter {
    pub fn new() -> InvoiceFormatter {
        InvoiceFormatter
    }

    /// `id,customer,issued,due,currency,subtotal,tax`
    pub fn to_csv_row(&self, invoice: &Invoice, tax: &Money) -> Result<String, CurrencyMismatch> {
        Ok(format!(
            "{},{},{},{},{},{},{}",
            invoice.id(),
            invoice.customer_id(),
            invoice.issued_at().format(ISO_DATE),
            invoice.due_at().format(ISO_DATE),
            invoice.currency(),
            invoice.subtotal()?.amount(),
            tax.amount(),
        ))
    }
}
