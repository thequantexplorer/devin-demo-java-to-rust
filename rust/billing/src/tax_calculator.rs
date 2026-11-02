//! Port of `com.acme.billing.TaxCalculator`.

use crate::decimal::Decimal;
use crate::invoice::Invoice;
use crate::money::{CurrencyMismatch, Money};

#[derive(Debug, Default, Clone, Copy)]
pub struct TaxCalculator;

impl TaxCalculator {
    pub fn new() -> TaxCalculator {
        TaxCalculator
    }

    /// Tax rate in percent. Unknown regions are 0%.
    pub fn rate_for(&self, region: &str) -> Decimal {
        match region {
            "US-CA" => Decimal::new(725, 2),
            "US-NY" => Decimal::new(400, 2),
            "US-TX" => Decimal::new(625, 2),
            "DE" => Decimal::new(1900, 2),
            "GB" => Decimal::new(2000, 2),
            _ => Decimal::new(0, 0),
        }
    }

    pub fn tax_for(&self, invoice: &Invoice, region: &str) -> Result<Money, CurrencyMismatch> {
        Ok(invoice.subtotal()?.percent(self.rate_for(region)))
    }
}
