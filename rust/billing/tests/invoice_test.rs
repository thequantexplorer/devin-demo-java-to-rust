//! Port of src/test/java/com/acme/billing/InvoiceTest.java.

use acme_billing::{
    billing_run, AgingReport, DueDateCalculator, Invoice, InvoiceFormatter, InvoiceXml, LineItem,
    Money, PaymentTokenCodec, TaxCalculator,
};
use chrono::NaiveDate;

fn d(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
}

fn sample() -> Invoice {
    let mut inv = Invoice::new("INV-1", "CUST-9", "USD", d("2025-01-31"), d("2025-03-03"));
    inv.add_item(LineItem::new("SKU-1", 2, Money::of("19.99", "USD")));
    inv.add_item(LineItem::new("SKU-2", 1, Money::of("5.00", "USD")));
    inv
}

#[test]
fn subtotal() {
    assert_eq!(Money::of("44.98", "USD"), sample().subtotal().unwrap());
}

#[test]
fn tax_by_region() {
    let tax = TaxCalculator::new();
    assert_eq!(
        Money::of("3.26", "USD"),
        tax.tax_for(&sample(), "US-CA").unwrap()
    );
    assert_eq!(
        Money::of("0.00", "USD"),
        tax.tax_for(&sample(), "US-OR").unwrap()
    );
}

#[test]
fn due_date_rolls_weekend_to_monday() {
    let due = DueDateCalculator::new();
    // 2025-01-31 + 30 = 2025-03-02 (Sunday) -> Monday 2025-03-03
    assert_eq!(d("2025-03-03"), due.due_date(d("2025-01-31"), 30));
    assert!(due.is_overdue(d("2025-03-03"), d("2025-03-04")));
    assert!(!due.is_overdue(d("2025-03-03"), d("2025-03-02")));
}

#[test]
fn csv_row() {
    let row = InvoiceFormatter::new()
        .to_csv_row(&sample(), &Money::of("3.26", "USD"))
        .unwrap();
    assert_eq!("INV-1,CUST-9,2025-01-31,2025-03-03,USD,44.98,3.26", row);
}

#[test]
fn token_round_trip() {
    let codec = PaymentTokenCodec::new();
    let parts = codec.decode(&codec.encode("CUST-9", "INV-1")).unwrap();
    assert_eq!("CUST-9", parts[0]);
    assert_eq!("INV-1", parts[1]);
}

#[test]
fn xml_envelope() {
    assert_eq!(
        "<invoice id=\"INV-1\" customer=\"CUST-9\" checksum=\"494E562D31\"><subtotal>44.98</subtotal></invoice>",
        InvoiceXml::new().envelope(&sample()).unwrap()
    );
}

#[test]
fn aging_buckets() {
    let a = Invoice::new("A", "C", "USD", d("2025-01-01"), d("2025-02-01"));
    let b = Invoice::new("B", "C", "USD", d("2025-01-01"), d("2025-01-01"));
    let buckets = AgingReport::new().bucket(&[a, b], d("2025-02-15"));
    assert_eq!(0, buckets[0]);
    assert_eq!(1, buckets[1]);
    assert_eq!(1, buckets[2]);
}

#[test]
fn billing_run_is_deterministic() {
    assert_eq!(billing_run::run(50, 1), billing_run::run(50, 1));
    assert_eq!(50, billing_run::run(50, 1).len());
}
