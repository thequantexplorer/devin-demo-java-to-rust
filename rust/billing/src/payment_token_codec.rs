//! Port of `com.acme.billing.PaymentTokenCodec`, which used `sun.misc.BASE64Encoder`.
//!
//! The legacy encoder writes 57 input bytes (76 output characters) per line and ends each
//! full line with the platform line separator (`\n` on Linux, where the fixtures were made).
//! A partial last line has no separator, so a full last line does get a trailing `\n`.

use std::fmt;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

const BYTES_PER_LINE: usize = 57;
const LINE_SEPARATOR: &str = "\n";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BadToken;

impl fmt::Display for BadToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("bad token")
    }
}

impl std::error::Error for BadToken {}

#[derive(Debug, Default, Clone, Copy)]
pub struct PaymentTokenCodec;

impl PaymentTokenCodec {
    pub fn new() -> PaymentTokenCodec {
        PaymentTokenCodec
    }

    pub fn encode(&self, customer_id: &str, invoice_id: &str) -> String {
        legacy_base64_encode(format!("{customer_id}:{invoice_id}").as_bytes())
    }

    /// Returns `[customer, invoice]`, split on the first `:` like `split(":", 2)`.
    pub fn decode(&self, token: &str) -> Result<Vec<String>, BadToken> {
        let compact: String = token.chars().filter(|c| !c.is_ascii_whitespace()).collect();
        let bytes = STANDARD.decode(compact).map_err(|_| BadToken)?;
        Ok(String::from_utf8_lossy(&bytes)
            .splitn(2, ':')
            .map(str::to_string)
            .collect())
    }
}

/// `new sun.misc.BASE64Encoder().encode(bytes)`.
pub fn legacy_base64_encode(bytes: &[u8]) -> String {
    let mut out = String::new();
    for line in bytes.chunks(BYTES_PER_LINE) {
        out.push_str(&STANDARD.encode(line));
        if line.len() == BYTES_PER_LINE {
            out.push_str(LINE_SEPARATOR);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn letters(len: usize) -> Vec<u8> {
        (0..len).map(|i| b'a' + (i % 26) as u8).collect()
    }

    // Reference output of sun.misc.BASE64Encoder on Java 8 (Linux).
    #[test]
    fn matches_sun_misc_line_breaks() {
        const L1: &str =
            "YWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXphYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5emFiY2Rl";
        assert_eq!(legacy_base64_encode(&letters(0)), "");
        assert_eq!(legacy_base64_encode(&letters(1)), "YQ==");
        assert_eq!(
            legacy_base64_encode(&letters(56)),
            "YWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXphYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5emFiY2Q="
        );
        assert_eq!(legacy_base64_encode(&letters(57)), format!("{L1}\n"));
        assert_eq!(legacy_base64_encode(&letters(58)), format!("{L1}\nZg=="));
        assert_eq!(
            legacy_base64_encode(&letters(114)),
            format!("{L1}\nZmdoaWprbG1ub3BxcnN0dXZ3eHl6YWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXphYmNkZWZnaGlq\n")
        );
    }

    #[test]
    fn decodes_multi_line_tokens() {
        let codec = PaymentTokenCodec::new();
        let customer = "CUSTOMER-ACME-WIDGETS-NORTH-AMERICA-ENTERPRISE-DIVISION";
        let invoice = "INV:with:colons";
        let token = codec.encode(customer, invoice);
        assert!(token.contains('\n'));
        assert_eq!(codec.decode(&token).unwrap(), vec![customer, invoice]);
        assert_eq!(codec.decode("!!").unwrap_err(), BadToken);
    }
}
