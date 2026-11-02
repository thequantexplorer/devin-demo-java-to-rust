# Porting notes: Java 8 to Rust

The Rust port lives in `rust/`: a cargo workspace with one library crate, `acme-billing`
(`rust/billing`, imported as `acme_billing`), and two binaries:

- `golden <input.txt>` prints the same bytes as `com.acme.billing.GoldenCli`. `scripts/golden-check.sh rust` runs it.
- `billing-run [n]` is the batch benchmark from `BillingRun.main`.

```sh
cargo test --manifest-path rust/Cargo.toml   # ported JUnit tests + unit tests + all fixtures
scripts/golden-check.sh rust                 # 6/6 fixtures match (rust)
```

## Class to module map

| Java class | Rust module | Rust items |
| --- | --- | --- |
| `Money` | `money` | `Money`, `CurrencyMismatch` |
| `java.math.BigDecimal` (inputs only) | `decimal` | `Decimal` |
| `LineItem` | `line_item` | `LineItem` |
| `Invoice` | `invoice` | `Invoice` |
| `TaxCalculator` | `tax_calculator` | `TaxCalculator` |
| `DueDateCalculator` | `due_date_calculator` | `DueDateCalculator` |
| `InvoiceFormatter` | `invoice_formatter` | `InvoiceFormatter` |
| `InvoiceXml` | `invoice_xml` | `InvoiceXml`, `print_hex_binary` |
| `PaymentTokenCodec` | `payment_token_codec` | `PaymentTokenCodec`, `legacy_base64_encode` |
| `AgingReport` | `aging_report` | `AgingReport` |
| `BillingRun` | `billing_run` (+ `src/bin/billing_run.rs` for `main`) | `run` |
| `java.util.Random` | `java_random` | `JavaRandom` |
| `GoldenCli` | `golden_cli` (+ `src/bin/golden.rs`) | `run` |
| `MoneyTest` | `tests/money_test.rs` | same 4 cases |
| `InvoiceTest` | `tests/invoice_test.rs` | same 8 cases |

Java exceptions become `Result`s: a currency mismatch is `Err(CurrencyMismatch)` from
`Money::plus`, `Invoice::subtotal`, `TaxCalculator::tax_for`, `InvoiceFormatter::to_csv_row`
and `InvoiceXml::envelope`, and its `Display` is the Java message (`currency mismatch: USD vs EUR`).

## Behaviors that were easy to get wrong

- **Rounding.** `Money` stores an `i128` count of cents; there is no `f32`/`f64` anywhere in the crate.
  Input strings are parsed exactly (`Decimal`, same grammar as `new BigDecimal(String)`, including
  exponents), then rounded half to even to 2 decimals (`0.125 -> 0.12`, `0.135 -> 0.14`,
  `-2.675 -> -2.68`). `times` and `percent` multiply in integers and do one half-even division,
  like `multiply(...).divide(100, 2, ROUND_HALF_EVEN)`. Amounts print like `toPlainString()`,
  so a negative tax that rounds to zero prints `0.00`, not `-0.00`.
- **Base64 line breaks.** `sun.misc.BASE64Encoder` encodes 57 bytes (76 characters) per line and
  writes the line separator after each *full* line only. `legacy_base64_encode` does the same: so a
  token of exactly 57 or 114 bytes ends with a trailing `\n`, and a shorter last line does not. The
  separator is `\n` because the fixtures were made on Linux. Unit tests compare against output
  captured from Java 8 for 0, 1, 56, 57, 58 and 114 bytes. `decode` strips whitespace first, like
  `decodeBuffer`.
- **java.util.Random.** `JavaRandom` is the same 48-bit LCG (`0x5DEECE66D`, `0xB`, initial scramble
  `seed ^ 0x5DEECE66D`), with both branches of `nextInt(bound)`: the power-of-two shortcut (used
  for `nextInt(8)`) and the rejection loop with Java's `int` overflow check (wrapping `i32`
  arithmetic). `billing_run::run` draws the numbers in the same order as Java evaluates the
  expressions: customer, line count, then per line SKU, quantity and price, then region.
  Unit tests check values captured from Java 8.
- **Dates.** Every `java.util.Date` in this code is midnight UTC, so the port uses `chrono::NaiveDate`
  (a UTC calendar day). Net-N adds days and then moves Saturday +2 and Sunday +1 to Monday,
  the same as `Calendar` in UTC (leap days included: 2024-01-30 + 30 = 2024-02-29). `isOverdue`
  is strictly after. Joda `Days.daysBetween` on two UTC midnights is the plain day difference,
  and the aging buckets use the same `<= 0 / 30 / 60 / 90` limits. `BillingRun`'s issue date is still
  built from the epoch millis `1735689600000`. Dates are parsed strictly as `YYYY-MM-DD`; Java's
  non-lenient `SimpleDateFormat` would also take unpadded fields or trailing text, which no fixture uses.
- **CLI details.** Lines are trimmed like `String.trim()` and split on Java's `\s` set. Like `GoldenCli`,
  the binary buffers all output and prints nothing if any line fails, and it exits non-zero.
  `InvoiceXml` writes the hex checksum in upper case and does not XML-escape attribute values,
  matching the Java.
