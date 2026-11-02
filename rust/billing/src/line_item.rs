//! Port of `com.acme.billing.LineItem`.

use crate::decimal::Decimal;
use crate::money::Money;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineItem {
    sku: String,
    quantity: i32,
    unit_price: Money,
}

impl LineItem {
    pub fn new(sku: &str, quantity: i32, unit_price: Money) -> LineItem {
        LineItem {
            sku: sku.to_string(),
            quantity,
            unit_price,
        }
    }

    pub fn total(&self) -> Money {
        self.unit_price.times(Decimal::from(self.quantity))
    }

    pub fn sku(&self) -> &str {
        &self.sku
    }

    pub fn quantity(&self) -> i32 {
        self.quantity
    }

    pub fn unit_price(&self) -> &Money {
        &self.unit_price
    }
}
