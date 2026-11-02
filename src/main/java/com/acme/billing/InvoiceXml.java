package com.acme.billing;

import javax.xml.bind.DatatypeConverter;

/** Minimal XML envelope for the ERP export. */
public class InvoiceXml {
    public String envelope(Invoice invoice) {
        String checksum = DatatypeConverter.printHexBinary(invoice.getId().getBytes());
        return "<invoice id=\"" + invoice.getId() + "\" customer=\"" + invoice.getCustomerId()
            + "\" checksum=\"" + checksum + "\"><subtotal>" + invoice.subtotal().getAmount().toPlainString()
            + "</subtotal></invoice>";
    }
}
