//! Port of `com.acme.billing.Invoice`.

use chrono::NaiveDate;

use crate::line_item::LineItem;
use crate::money::{CurrencyMismatch, Money};

/// `java.util.Date` fields become `NaiveDate`: every date in this code is midnight UTC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invoice {
    id: String,
    customer_id: String,
    currency: String,
    issued_at: NaiveDate,
    due_at: NaiveDate,
    items: Vec<LineItem>,
}

impl Invoice {
    pub fn new(
        id: &str,
        customer_id: &str,
        currency: &str,
        issued_at: NaiveDate,
        due_at: NaiveDate,
    ) -> Invoice {
        Invoice {
            id: id.to_string(),
            customer_id: customer_id.to_string(),
            currency: currency.to_string(),
            issued_at,
            due_at,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: LineItem) {
        self.items.push(item);
    }

    /// Sum of the line totals. Fails on the first item whose currency differs from the invoice's.
    pub fn subtotal(&self) -> Result<Money, CurrencyMismatch> {
        self.items
            .iter()
            .try_fold(Money::of("0", &self.currency), |total, item| {
                total.plus(&item.total())
            })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn customer_id(&self) -> &str {
        &self.customer_id
    }

    pub fn issued_at(&self) -> NaiveDate {
        self.issued_at
    }

    pub fn due_at(&self) -> NaiveDate {
        self.due_at
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn items(&self) -> &[LineItem] {
        &self.items
    }
}
