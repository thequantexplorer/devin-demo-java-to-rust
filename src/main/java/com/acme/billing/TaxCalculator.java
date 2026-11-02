package com.acme.billing;

import java.math.BigDecimal;
import java.util.HashMap;
import java.util.Map;

public class TaxCalculator {
    private static final Map<String, BigDecimal> RATES = new HashMap<String, BigDecimal>();
    static {
        RATES.put("US-CA", new BigDecimal("7.25"));
        RATES.put("US-NY", new BigDecimal("4.00"));
        RATES.put("US-TX", new BigDecimal("6.25"));
        RATES.put("DE", new BigDecimal("19.00"));
        RATES.put("GB", new BigDecimal("20.00"));
    }

    public Money taxFor(Invoice invoice, String region) {
        BigDecimal rate = RATES.get(region);
        if (rate == null) {
            rate = new BigDecimal("0");
        }
        return invoice.subtotal().percent(rate);
    }
}
