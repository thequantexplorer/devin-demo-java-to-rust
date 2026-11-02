package com.acme.billing;

import java.text.SimpleDateFormat;
import java.util.TimeZone;

public class InvoiceFormatter {
    // Shared across threads by the batch exporter.
    private static final SimpleDateFormat ISO = new SimpleDateFormat("yyyy-MM-dd");
    static { ISO.setTimeZone(TimeZone.getTimeZone("UTC")); }

    public String toCsvRow(Invoice invoice, Money tax) {
        StringBuffer sb = new StringBuffer();
        sb.append(invoice.getId()).append(',');
        sb.append(invoice.getCustomerId()).append(',');
        sb.append(ISO.format(invoice.getIssuedAt())).append(',');
        sb.append(ISO.format(invoice.getDueAt())).append(',');
        sb.append(invoice.getCurrency()).append(',');
        sb.append(invoice.subtotal().getAmount().toPlainString()).append(',');
        sb.append(tax.getAmount().toPlainString());
        return sb.toString();
    }
}
