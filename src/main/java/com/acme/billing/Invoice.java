package com.acme.billing;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Date;
import java.util.List;

public class Invoice {
    private final String id;
    private final String customerId;
    private final Date issuedAt;
    private final Date dueAt;
    private final List<LineItem> items = new ArrayList<LineItem>();
    private final String currency;

    public Invoice(String id, String customerId, String currency, Date issuedAt, Date dueAt) {
        this.id = id;
        this.customerId = customerId;
        this.currency = currency;
        this.issuedAt = issuedAt;
        this.dueAt = dueAt;
    }

    public void addItem(LineItem item) { items.add(item); }

    public Money subtotal() {
        Money total = Money.of("0", currency);
        for (LineItem item : items) {
            total = total.plus(item.total());
        }
        return total;
    }

    public String getId() { return id; }
    public String getCustomerId() { return customerId; }
    public Date getIssuedAt() { return issuedAt; }
    public Date getDueAt() { return dueAt; }
    public String getCurrency() { return currency; }
    public List<LineItem> getItems() { return Collections.unmodifiableList(items); }
}
