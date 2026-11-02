# Golden fixtures

Each folder has an `input.txt` and an `expected.txt`. `expected.txt` is what the Java app (`com.acme.billing.GoldenCli`) prints for that input. Any other implementation must print exactly the same bytes.

```sh
scripts/golden-check.sh java            # Java app vs expected.txt
scripts/golden-check.sh rust            # Rust port vs expected.txt
scripts/golden-check.sh java --update   # rewrite expected.txt from the Java app (needs JDK 8)
```

## How a port is run

For `rust`, the script runs `cargo build --release --manifest-path rust/Cargo.toml --bin golden` and then `rust/target/release/golden <input.txt>`. The binary prints to stdout and exits 0.

## Input lines

Blank lines and lines starting with `#` are skipped. Fields are split on spaces.

| line | meaning |
| --- | --- |
| `invoice <id> <customer> <currency> <issued YYYY-MM-DD> <net days> <region>` | start an invoice; the due date comes from `DueDateCalculator` |
| `item <sku> <qty> <unit price> [currency]` | add a line item to the last invoice; currency defaults to the invoice's |
| `print` | print every invoice so far, in order |
| `aging <as-of YYYY-MM-DD>` | print the aging buckets for every invoice so far |
| `batch <count> <seed>` | print `BillingRun.run(count, seed)` |

## Output lines

For each invoice, `print` writes:

```text
csv,<InvoiceFormatter.toCsvRow with TaxCalculator.taxFor(region)>
xml,<InvoiceXml.envelope>
token,<PaymentTokenCodec.encode(customer, invoice)>
```

If the invoice mixes currencies, the `csv` and `xml` lines are replaced by `error,<id>,<message>`. The legacy Base64 encoder adds a line break after every 76 characters; in the `token` line each break is written as the two characters `\n`.

`aging` writes `aging,<as-of>,<current>,<1-30>,<31-60>,<61-90>,<90+>`. `batch` writes one `batch,<csv row>` line per invoice.

All dates are UTC.
