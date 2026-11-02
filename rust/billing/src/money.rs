//! Port of `com.acme.billing.Money`: an amount with exactly 2 decimals, rounded half to even.
//!
//! The amount is stored as integer cents (`i128`), so no floating point is involved.

use std::fmt;

use crate::decimal::{div_round_half_even, pow10, Decimal, ParseDecimalError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Money {
    cents: i128,
    currency: String,
}

/// The Java code throws `IllegalArgumentException("currency mismatch: A vs B")`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrencyMismatch {
    pub expected: String,
    pub actual: String,
}

impl fmt::Display for CurrencyMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "currency mismatch: {} vs {}", self.expected, self.actual)
    }
}

impl std::error::Error for CurrencyMismatch {}

impl Money {
    /// `new Money(BigDecimal, currency)`: rounds the amount to 2 decimals, half to even.
    pub fn new(amount: Decimal, currency: &str) -> Money {
        Money::from_cents(amount.to_cents(), currency)
    }

    pub fn from_cents(cents: i128, currency: &str) -> Money {
        Money {
            cents,
            currency: currency.to_string(),
        }
    }

    /// `Money.of(String, currency)`.
    ///
    /// # Panics
    /// If `amount` is not a valid decimal (Java throws `NumberFormatException`).
    /// Use [`Money::parse`] to handle that case.
    pub fn of(amount: &str, currency: &str) -> Money {
        Money::parse(amount, currency).unwrap_or_else(|e| panic!("{e}"))
    }

    pub fn parse(amount: &str, currency: &str) -> Result<Money, ParseDecimalError> {
        Ok(Money::new(amount.parse()?, currency))
    }

    pub fn plus(&self, other: &Money) -> Result<Money, CurrencyMismatch> {
        self.require_same_currency(other)?;
        let cents = self.cents.checked_add(other.cents).expect("money overflow");
        Ok(Money::from_cents(cents, &self.currency))
    }

    /// `amount * factor`, rounded to 2 decimals half to even.
    pub fn times(&self, factor: Decimal) -> Money {
        let product = self
            .cents
            .checked_mul(factor.mantissa())
            .expect("money overflow");
        Money::from_cents(scale_down(product, factor.scale()), &self.currency)
    }

    /// `amount * pct / 100`, rounded to 2 decimals half to even.
    pub fn percent(&self, pct: Decimal) -> Money {
        let product = self
            .cents
            .checked_mul(pct.mantissa())
            .expect("money overflow");
        Money::from_cents(scale_down(product, pct.scale() + 2), &self.currency)
    }

    pub fn cents(&self) -> i128 {
        self.cents
    }

    /// `getAmount().toPlainString()`, e.g. `"44.98"`, `"-0.36"`, `"0.00"`.
    pub fn amount(&self) -> String {
        let sign = if self.cents < 0 { "-" } else { "" };
        let abs = self.cents.unsigned_abs();
        format!("{sign}{}.{:02}", abs / 100, abs % 100)
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    fn require_same_currency(&self, other: &Money) -> Result<(), CurrencyMismatch> {
        if self.currency != other.currency {
            return Err(CurrencyMismatch {
                expected: self.currency.clone(),
                actual: other.currency.clone(),
            });
        }
        Ok(())
    }
}

/// Divides by `10^digits`, rounding half to even.
fn scale_down(value: i128, digits: u32) -> i128 {
    match pow10(digits) {
        Some(divisor) => div_round_half_even(value, divisor),
        None => 0,
    }
}

/// `toString()`: `"USD 44.98"`.
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.currency, self.amount())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_string() {
        assert_eq!(Money::of("-5.01", "USD").amount(), "-5.01");
        assert_eq!(Money::of("-0.05", "USD").amount(), "-0.05");
        assert_eq!(Money::of("0", "USD").amount(), "0.00");
        assert_eq!(Money::of("499999999.96", "USD").amount(), "499999999.96");
        assert_eq!(Money::of("1", "USD").to_string(), "USD 1.00");
    }

    #[test]
    fn percent_of_negative_rounds_to_zero_without_sign() {
        // Java: -0.05 * 7.25 / 100 = -0.003625 -> "0.00"
        let tax = Money::of("-0.05", "USD").percent("7.25".parse().unwrap());
        assert_eq!(tax.amount(), "0.00");
    }

    #[test]
    fn times_rounds_half_even() {
        assert_eq!(
            Money::of("0.25", "USD").times("0.5".parse().unwrap()),
            Money::of("0.12", "USD")
        );
        assert_eq!(
            Money::of("19.99", "USD").times(Decimal::from(2)),
            Money::of("39.98", "USD")
        );
    }
}
