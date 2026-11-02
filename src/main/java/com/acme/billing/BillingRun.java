package com.acme.billing;

import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.Date;
import java.util.List;
import java.util.Random;

/** Monthly billing batch: builds invoices, computes tax, exports CSV. Also used as the perf benchmark. */
public class BillingRun {
    public static List<String> run(int invoiceCount, long seed) {
        Random rnd = new Random(seed);
        TaxCalculator tax = new TaxCalculator();
        DueDateCalculator due = new DueDateCalculator();
        InvoiceFormatter fmt = new InvoiceFormatter();
        String[] regions = {"US-CA", "US-NY", "US-TX", "DE", "GB"};
        List<String> rows = new ArrayList<String>(invoiceCount);
        Date issued = new Date(1735689600000L);
        for (int i = 0; i < invoiceCount; i++) {
            Invoice inv = new Invoice("INV-" + i, "CUST-" + rnd.nextInt(5000), "USD", issued, due.dueDate(issued, 30));
            int lines = 1 + rnd.nextInt(8);
            for (int l = 0; l < lines; l++) {
                inv.addItem(new LineItem("SKU-" + rnd.nextInt(900), 1 + rnd.nextInt(5),
                    new Money(new BigDecimal(rnd.nextInt(50000)).movePointLeft(2), "USD")));
            }
            rows.add(fmt.toCsvRow(inv, tax.taxFor(inv, regions[rnd.nextInt(regions.length)])));
        }
        return rows;
    }

    public static void main(String[] args) {
        int n = args.length > 0 ? Integer.parseInt(args[0]) : 200000;
        for (int warm = 0; warm < 3; warm++) run(n / 4, 7L);
        long start = System.nanoTime();
        List<String> rows = run(n, 42L);
        long ms = (System.nanoTime() - start) / 1000000L;
        System.out.println("java.version=" + System.getProperty("java.version") + " invoices=" + rows.size() + " elapsed_ms=" + ms);
    }
}
