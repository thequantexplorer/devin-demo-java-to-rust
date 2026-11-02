package com.acme.billing;

import static org.junit.Assert.assertEquals;

import java.math.BigDecimal;
import org.junit.Test;

public class MoneyTest {
    @Test public void addsSameCurrency() {
        assertEquals(Money.of("3.50", "USD"), Money.of("1.25", "USD").plus(Money.of("2.25", "USD")));
    }

    @Test(expected = IllegalArgumentException.class)
    public void rejectsCurrencyMismatch() {
        Money.of("1", "USD").plus(Money.of("1", "EUR"));
    }

    @Test public void bankersRounding() {
        assertEquals(Money.of("0.12", "USD"), Money.of("0.125", "USD"));
        assertEquals(Money.of("0.14", "USD"), Money.of("0.135", "USD"));
    }

    @Test public void percent() {
        assertEquals(Money.of("7.25", "USD"), Money.of("100", "USD").percent(new BigDecimal("7.25")));
    }
}
