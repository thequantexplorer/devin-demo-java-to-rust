//! Port of `com.acme.billing.GoldenCli`: turns a fixture input (see `fixtures/README.md`)
//! into the billing output. Returns the whole output, or an error and no output, like the
//! Java class, which buffers everything and prints it at the end.

use std::fmt;

use chrono::NaiveDate;

use crate::aging_report::AgingReport;
use crate::billing_run;
use crate::due_date_calculator::DueDateCalculator;
use crate::invoice::Invoice;
use crate::invoice_formatter::{InvoiceFormatter, ISO_DATE};
use crate::invoice_xml::InvoiceXml;
use crate::line_item::LineItem;
use crate::money::{CurrencyMismatch, Money};
use crate::payment_token_codec::PaymentTokenCodec;
use crate::tax_calculator::TaxCalculator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliError(pub String);

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CliError {}

pub fn run(input: &str) -> Result<String, CliError> {
    let tax = TaxCalculator::new();
    let due = DueDateCalculator::new();
    let fmt = InvoiceFormatter::new();
    let xml = InvoiceXml::new();
    let codec = PaymentTokenCodec::new();
    let aging = AgingReport::new();

    let mut invoices: Vec<Invoice> = Vec::new();
    let mut regions: Vec<String> = Vec::new();
    let mut out = String::new();

    // BufferedReader.readLine ends a line at \n, \r or \r\n; blank lines are skipped anyway.
    for raw in input.split(['\n', '\r']) {
        // String.trim() strips every char <= ' '.
        let line = raw.trim_matches(|c: char| c <= ' ');
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // split("\\s+"): \s is [ \t\n\x0B\f\r] in Java.
        let f: Vec<&str> = line
            .split([' ', '\t', '\n', '\x0B', '\x0C', '\r'])
            .filter(|s| !s.is_empty())
            .collect();
        let field = |i: usize| {
            f.get(i)
                .copied()
                .ok_or_else(|| CliError(format!("missing field {i} in line: {line}")))
        };
        match f[0] {
            "invoice" => {
                let issued = parse_date(field(4)?)?;
                let net_days = parse_int(field(5)?)?;
                invoices.push(Invoice::new(
                    field(1)?,
                    field(2)?,
                    field(3)?,
                    issued,
                    due.due_date(issued, net_days),
                ));
                regions.push(field(6)?.to_string());
            }
            "item" => {
                let (sku, qty, price) = (field(1)?, parse_int(field(2)?)?, field(3)?);
                let inv = invoices
                    .last_mut()
                    .ok_or_else(|| CliError(format!("item before any invoice: {line}")))?;
                let currency = f.get(4).copied().unwrap_or(inv.currency()).to_string();
                let unit_price =
                    Money::parse(price, &currency).map_err(|e| CliError(e.to_string()))?;
                inv.add_item(LineItem::new(sku, qty, unit_price));
            }
            "print" => {
                for (inv, region) in invoices.iter().zip(&regions) {
                    let rows = || -> Result<(String, String), CurrencyMismatch> {
                        let csv = fmt.to_csv_row(inv, &tax.tax_for(inv, region)?)?;
                        Ok((csv, xml.envelope(inv)?))
                    };
                    match rows() {
                        Ok((csv, envelope)) => {
                            out.push_str(&format!("csv,{csv}\nxml,{envelope}\n"));
                        }
                        Err(e) => out.push_str(&format!("error,{},{e}\n", inv.id())),
                    }
                    let token = codec.encode(inv.customer_id(), inv.id());
                    let token = token.replace('\r', "\\r").replace('\n', "\\n");
                    out.push_str(&format!("token,{token}\n"));
                }
            }
            "aging" => {
                let as_of = field(1)?;
                let buckets = aging.bucket(&invoices, parse_date(as_of)?);
                out.push_str("aging,");
                out.push_str(as_of);
                for n in buckets {
                    out.push_str(&format!(",{n}"));
                }
                out.push('\n');
            }
            "batch" => {
                let count = usize::try_from(parse_int(field(1)?)?)
                    .map_err(|_| CliError(format!("negative batch count: {line}")))?;
                let seed: i64 = field(2)?
                    .parse()
                    .map_err(|_| CliError(format!("not a long: {}", f[2])))?;
                for row in billing_run::run(count, seed) {
                    out.push_str(&format!("batch,{row}\n"));
                }
            }
            _ => return Err(CliError(format!("unknown line: {line}"))),
        }
    }
    Ok(out)
}

fn parse_date(s: &str) -> Result<NaiveDate, CliError> {
    NaiveDate::parse_from_str(s, ISO_DATE).map_err(|_| CliError(format!("unparseable date: {s}")))
}

fn parse_int(s: &str) -> Result<i32, CliError> {
    s.parse()
        .map_err(|_| CliError(format!("for input string: \"{s}\"")))
}
