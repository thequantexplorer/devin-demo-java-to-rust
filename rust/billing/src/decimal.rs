//! Exact decimal numbers, parsed the way `new java.math.BigDecimal(String)` parses them.
//!
//! A `Decimal` is `mantissa * 10^-scale`. It is only used as an input type
//! (prices, tax rates, quantities); `Money` turns it into integer cents.

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decimal {
    mantissa: i128,
    scale: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDecimalError {
    input: String,
}

impl fmt::Display for ParseDecimalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid decimal: {:?}", self.input)
    }
}

impl std::error::Error for ParseDecimalError {}

impl Decimal {
    pub const fn new(mantissa: i128, scale: u32) -> Decimal {
        Decimal { mantissa, scale }
    }

    pub const fn mantissa(&self) -> i128 {
        self.mantissa
    }

    pub const fn scale(&self) -> u32 {
        self.scale
    }

    /// Rounds to 2 decimals, half to even, and returns the result in hundredths.
    /// Same as `BigDecimal.setScale(2, ROUND_HALF_EVEN)`.
    pub fn to_cents(&self) -> i128 {
        if self.scale <= 2 {
            self.mantissa
                .checked_mul(pow10(2 - self.scale).expect("scale <= 2"))
                .expect("amount out of range")
        } else {
            match pow10(self.scale - 2) {
                Some(divisor) => div_round_half_even(self.mantissa, divisor),
                // 10^39 > 2 * i128::MAX, so any mantissa rounds to zero.
                None => 0,
            }
        }
    }
}

impl From<i32> for Decimal {
    fn from(v: i32) -> Decimal {
        Decimal::new(v as i128, 0)
    }
}

impl From<i64> for Decimal {
    fn from(v: i64) -> Decimal {
        Decimal::new(v as i128, 0)
    }
}

impl FromStr for Decimal {
    type Err = ParseDecimalError;

    /// Accepts the `BigDecimal(String)` grammar: optional sign, digits with an
    /// optional `.`, and an optional exponent (`1.5E-2`, `1E+3`).
    fn from_str(s: &str) -> Result<Decimal, ParseDecimalError> {
        let err = || ParseDecimalError {
            input: s.to_string(),
        };
        let (number, exponent) = match s.find(['e', 'E']) {
            Some(i) => (&s[..i], parse_exponent(&s[i + 1..]).ok_or_else(err)?),
            None => (s, 0i64),
        };
        let (negative, digits) = match number.as_bytes().first() {
            Some(b'-') => (true, &number[1..]),
            Some(b'+') => (false, &number[1..]),
            _ => (false, number),
        };
        let (int_part, frac_part) = match digits.find('.') {
            Some(i) => (&digits[..i], &digits[i + 1..]),
            None => (digits, ""),
        };
        let all_digits = |p: &str| p.bytes().all(|b| b.is_ascii_digit());
        if int_part.len() + frac_part.len() == 0 || !all_digits(int_part) || !all_digits(frac_part)
        {
            return Err(err());
        }
        let mut mantissa: i128 = 0;
        for b in int_part.bytes().chain(frac_part.bytes()) {
            mantissa = mantissa
                .checked_mul(10)
                .and_then(|m| m.checked_add((b - b'0') as i128))
                .ok_or_else(err)?;
        }
        if negative {
            mantissa = -mantissa;
        }
        let scale = frac_part.len() as i64 - exponent;
        if scale >= 0 {
            let scale = u32::try_from(scale).map_err(|_| err())?;
            Ok(Decimal::new(mantissa, scale))
        } else {
            let factor = u32::try_from(-scale).ok().and_then(pow10).ok_or_else(err)?;
            let mantissa = mantissa.checked_mul(factor).ok_or_else(err)?;
            Ok(Decimal::new(mantissa, 0))
        }
    }
}

fn parse_exponent(s: &str) -> Option<i64> {
    let digits = s.strip_prefix(['+', '-']).unwrap_or(s);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok()
}

pub(crate) fn pow10(exp: u32) -> Option<i128> {
    10i128.checked_pow(exp)
}

/// `numerator / divisor` rounded half to even (banker's rounding). `divisor` must be > 0.
pub(crate) fn div_round_half_even(numerator: i128, divisor: i128) -> i128 {
    debug_assert!(divisor > 0);
    let quotient = numerator / divisor;
    let remainder = (numerator % divisor).unsigned_abs();
    let half = divisor.unsigned_abs();
    let twice = remainder * 2;
    let round_away = twice > half || (twice == half && quotient % 2 != 0);
    if !round_away {
        quotient
    } else if numerator < 0 {
        quotient - 1
    } else {
        quotient + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cents(s: &str) -> i128 {
        s.parse::<Decimal>().unwrap().to_cents()
    }

    // Reference values printed by Java 8:
    // new BigDecimal(s).setScale(2, ROUND_HALF_EVEN).toPlainString()
    #[test]
    fn matches_big_decimal_set_scale() {
        assert_eq!(cents("-0.125"), -12);
        assert_eq!(cents("-0.135"), -14);
        assert_eq!(cents("1E+3"), 100_000);
        assert_eq!(cents("1.5E-2"), 2);
        assert_eq!(cents("0.0050"), 0);
        assert_eq!(cents("-2.675"), -268);
        assert_eq!(cents("2.675"), 268);
        assert_eq!(cents("1.005"), 100);
        assert_eq!(cents("7"), 700);
        assert_eq!(cents("12.5"), 1250);
        assert_eq!(cents("+3.10"), 310);
        assert_eq!(cents(".5"), 50);
        assert_eq!(cents("5."), 500);
        assert_eq!(cents("0.000000000000000000000000000000000000000001"), 0);
    }

    #[test]
    fn rejects_what_big_decimal_rejects() {
        for bad in [
            "", "-", ".", "1.2.3", "abc", "1e", "1e+", "1,5", " 1", "--1",
        ] {
            assert!(bad.parse::<Decimal>().is_err(), "{bad:?} should not parse");
        }
    }

    #[test]
    fn half_even_division() {
        assert_eq!(div_round_half_even(5, 10), 0);
        assert_eq!(div_round_half_even(15, 10), 2);
        assert_eq!(div_round_half_even(25, 10), 2);
        assert_eq!(div_round_half_even(26, 10), 3);
        assert_eq!(div_round_half_even(-25, 10), -2);
        assert_eq!(div_round_half_even(-35, 10), -4);
        assert_eq!(div_round_half_even(-36, 10), -4);
        assert_eq!(div_round_half_even(-3625, 1_000_000), 0);
    }
}
