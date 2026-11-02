//! Port of `com.acme.billing.BillingRun`: the monthly batch, driven by `java.util.Random`.

use chrono::{DateTime, NaiveDate};

use crate::due_date_calculator::DueDateCalculator;
use crate::invoice::Invoice;
use crate::invoice_formatter::InvoiceFormatter;
use crate::java_random::JavaRandom;
use crate::line_item::LineItem;
use crate::money::Money;
use crate::tax_calculator::TaxCalculator;

const REGIONS: [&str; 5] = ["US-CA", "US-NY", "US-TX", "DE", "GB"];
const ISSUED_AT_MILLIS: i64 = 1_735_689_600_000;

/// `BillingRun.run(invoiceCount, seed)`. The random draws happen in exactly the Java order.
pub fn run(invoice_count: usize, seed: i64) -> Vec<String> {
    let mut rnd = JavaRandom::new(seed);
    let tax = TaxCalculator::new();
    let due = DueDateCalculator::new();
    let fmt = InvoiceFormatter::new();
    let mut rows = Vec::with_capacity(invoice_count);
    let issued = issued_at();
    for i in 0..invoice_count {
        let customer = format!("CUST-{}", rnd.next_int_bounded(5000));
        let mut inv = Invoice::new(
            &format!("INV-{i}"),
            &customer,
            "USD",
            issued,
            due.due_date(issued, 30),
        );
        let lines = 1 + rnd.next_int_bounded(8);
        for _ in 0..lines {
            let sku = format!("SKU-{}", rnd.next_int_bounded(900));
            let quantity = 1 + rnd.next_int_bounded(5);
            let unit_price = Money::from_cents(i128::from(rnd.next_int_bounded(50000)), "USD");
            inv.add_item(LineItem::new(&sku, quantity, unit_price));
        }
        let region = REGIONS[rnd.next_int_bounded(REGIONS.len() as i32) as usize];
        let tax = tax
            .tax_for(&inv, region)
            .expect("batch invoices are all USD");
        rows.push(
            fmt.to_csv_row(&inv, &tax)
                .expect("batch invoices are all USD"),
        );
    }
    rows
}

/// `new Date(1735689600000L)`: 2025-01-01T00:00:00Z.
fn issued_at() -> NaiveDate {
    DateTime::from_timestamp_millis(ISSUED_AT_MILLIS)
        .expect("valid timestamp")
        .date_naive()
}
