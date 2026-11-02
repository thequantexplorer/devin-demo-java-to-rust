# acme-legacy-billing

Monthly invoicing batch for Acme (2016). It builds invoices, works out regional tax, applies net-30 terms, and exports CSV and XML for the ERP.

It runs on **Java 8**. The team wants to move the billing logic to **Rust**. Before anyone trusts a port, it has to print exactly the same invoices as the Java app.

## Commands

```sh
mvn test                              # JUnit 4 suite (needs JDK 8)
cargo test --manifest-path rust/Cargo.toml  # Rust port tests
scripts/golden-check.sh java          # Java app vs the golden fixtures
scripts/golden-check.sh rust          # Rust port vs the golden fixtures
```

The Java app uses `sun.misc.BASE64Encoder`, which only exists on JDK 8, so build it with JDK 8.

## Golden fixtures

`fixtures/` holds 6 cases: basic invoices, rounding, dates and aging, payment tokens, currency errors, and a 325-invoice batch run. Each has an `input.txt` and the `expected.txt` that the Java app prints for it. [fixtures/README.md](fixtures/README.md) explains the format.

The Java app matches all of them:

```text
$ scripts/golden-check.sh java
PASS 01-basic
PASS 02-rounding
PASS 03-dates-and-aging
PASS 04-tokens
PASS 05-currency-mismatch
PASS 06-batch
6/6 fixtures match (java)
```

The Rust port in `rust/` matches them too (see [PORTING.md](PORTING.md)):

```text
$ scripts/golden-check.sh rust
...
6/6 fixtures match (rust)
```

The **CI** GitHub Action runs the Java and Rust tests and both golden checks on every pull request. You can also start it from the **Actions** tab with **Run workflow**.

## Code

| Java class | what it does |
| --- | --- |
| `Money` | amount with 2 decimals, rounded half to even; plus, times, percent |
| `LineItem` | SKU, quantity, unit price |
| `Invoice` | header fields, line items, subtotal |
| `TaxCalculator` | tax rate per region (unknown region = 0%) |
| `DueDateCalculator` | net-N due date in UTC; Saturday and Sunday move to Monday |
| `InvoiceFormatter` | one CSV row per invoice |
| `InvoiceXml` | XML envelope with a hex checksum of the invoice ID |
| `PaymentTokenCodec` | Base64 payment token `customer:invoice` |
| `AgingReport` | counts invoices by days past due |
| `BillingRun` | the monthly batch, driven by `java.util.Random` |
| `GoldenCli` | reads a fixture input and prints the output above |

---

This repo is a starter for a Devin demo. See [DEVIN_DEMO.md](DEVIN_DEMO.md) for setup and the prompt.
