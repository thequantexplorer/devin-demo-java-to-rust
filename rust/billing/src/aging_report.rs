//! Port of `com.acme.billing.AgingReport`.

use chrono::NaiveDate;

use crate::invoice::Invoice;

#[derive(Debug, Default, Clone, Copy)]
pub struct AgingReport;

impl AgingReport {
    pub fn new() -> AgingReport {
        AgingReport
    }

    /// Buckets: 0 = current, 1 = 1-30, 2 = 31-60, 3 = 61-90, 4 = 90+ days past due.
    pub fn bucket(&self, invoices: &[Invoice], as_of: NaiveDate) -> [u32; 5] {
        let mut buckets = [0u32; 5];
        for inv in invoices {
            // Joda Days.daysBetween on two UTC midnights is the plain day difference.
            let days = (as_of - inv.due_at()).num_days();
            let i = match days {
                d if d <= 0 => 0,
                1..=30 => 1,
                31..=60 => 2,
                61..=90 => 3,
                _ => 4,
            };
            buckets[i] += 1;
        }
        buckets
    }
}
