//! Port of src/test/java/com/acme/billing/MoneyTest.java.

use acme_billing::{CurrencyMismatch, Decimal, Money};

#[test]
fn adds_same_currency() {
    assert_eq!(
        Money::of("3.50", "USD"),
        Money::of("1.25", "USD")
            .plus(&Money::of("2.25", "USD"))
            .unwrap()
    );
}

#[test]
fn rejects_currency_mismatch() {
    let err = Money::of("1", "USD")
        .plus(&Money::of("1", "EUR"))
        .unwrap_err();
    assert_eq!(
        err,
        CurrencyMismatch {
            expected: "USD".into(),
            actual: "EUR".into()
        }
    );
    assert_eq!(err.to_string(), "currency mismatch: USD vs EUR");
}

#[test]
fn bankers_rounding() {
    assert_eq!(Money::of("0.12", "USD"), Money::of("0.125", "USD"));
    assert_eq!(Money::of("0.14", "USD"), Money::of("0.135", "USD"));
}

#[test]
fn percent() {
    assert_eq!(
        Money::of("7.25", "USD"),
        Money::of("100", "USD").percent("7.25".parse::<Decimal>().unwrap())
    );
}
