//! `billing-run [n]`: the batch benchmark from `BillingRun.main`.

use std::time::Instant;

use acme_billing::billing_run;

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .map(|s| s.parse().expect("invoice count"))
        .unwrap_or(200_000);
    for _ in 0..3 {
        billing_run::run(n / 4, 7);
    }
    let start = Instant::now();
    let rows = billing_run::run(n, 42);
    let ms = start.elapsed().as_millis();
    println!("rust invoices={} elapsed_ms={ms}", rows.len());
}
