//! Port of `com.acme.billing.InvoiceXml`: the ERP XML envelope.

use std::fmt::Write;

use crate::invoice::Invoice;
use crate::money::CurrencyMismatch;

#[derive(Debug, Default, Clone, Copy)]
pub struct InvoiceXml;

impl InvoiceXml {
    pub fn new() -> InvoiceXml {
        InvoiceXml
    }

    /// Like the Java code, attribute values are not XML-escaped.
    pub fn envelope(&self, invoice: &Invoice) -> Result<String, CurrencyMismatch> {
        Ok(format!(
            "<invoice id=\"{}\" customer=\"{}\" checksum=\"{}\"><subtotal>{}</subtotal></invoice>",
            invoice.id(),
            invoice.customer_id(),
            print_hex_binary(invoice.id().as_bytes()),
            invoice.subtotal()?.amount(),
        ))
    }
}

/// `DatatypeConverter.printHexBinary`: upper-case hex, two digits per byte.
pub fn print_hex_binary(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut s, b| {
            let _ = write!(s, "{b:02X}");
            s
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_is_upper_case() {
        assert_eq!(print_hex_binary(b"INV-1"), "494E562D31");
        assert_eq!(print_hex_binary(&[0x00, 0xab, 0xff]), "00ABFF");
        assert_eq!(print_hex_binary(b""), "");
    }
}
