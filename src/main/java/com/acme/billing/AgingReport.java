package com.acme.billing;

import java.util.Date;
import java.util.List;
import org.joda.time.DateTime;
import org.joda.time.Days;

public class AgingReport {
    /** Buckets: 0 = current, 1 = 1-30, 2 = 31-60, 3 = 61-90, 4 = 90+ days past due. */
    public int[] bucket(List<Invoice> invoices, Date asOf) {
        int[] buckets = new int[5];
        DateTime now = new DateTime(asOf);
        for (Invoice inv : invoices) {
            int days = Days.daysBetween(new DateTime(inv.getDueAt()), now).getDays();
            if (days <= 0) buckets[0]++;
            else if (days <= 30) buckets[1]++;
            else if (days <= 60) buckets[2]++;
            else if (days <= 90) buckets[3]++;
            else buckets[4]++;
        }
        return buckets;
    }
}
