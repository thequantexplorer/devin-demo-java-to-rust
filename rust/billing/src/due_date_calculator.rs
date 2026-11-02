//! Port of `com.acme.billing.DueDateCalculator`: net-N terms in UTC, weekends roll to Monday.

use chrono::{Datelike, Duration, NaiveDate, Weekday};

#[derive(Debug, Default, Clone, Copy)]
pub struct DueDateCalculator;

impl DueDateCalculator {
    pub fn new() -> DueDateCalculator {
        DueDateCalculator
    }

    pub fn due_date(&self, issued_at: NaiveDate, net_days: i32) -> NaiveDate {
        let due = issued_at + Duration::days(i64::from(net_days));
        match due.weekday() {
            Weekday::Sat => due + Duration::days(2),
            Weekday::Sun => due + Duration::days(1),
            _ => due,
        }
    }

    /// `now.after(dueAt)`: strictly later.
    pub fn is_overdue(&self, due_at: NaiveDate, now: NaiveDate) -> bool {
        now > due_at
    }
}
