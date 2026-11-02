package com.acme.billing;

import java.math.BigDecimal;

public class LineItem {
    private final String sku;
    private final int quantity;
    private final Money unitPrice;

    public LineItem(String sku, int quantity, Money unitPrice) {
        this.sku = sku;
        this.quantity = quantity;
        this.unitPrice = unitPrice;
    }

    public Money total() { return unitPrice.times(new BigDecimal(quantity)); }
    public String getSku() { return sku; }
    public int getQuantity() { return quantity; }
    public Money getUnitPrice() { return unitPrice; }
}
