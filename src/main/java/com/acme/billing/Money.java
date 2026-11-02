package com.acme.billing;

import java.math.BigDecimal;
import java.math.RoundingMode;

public final class Money {
    private final BigDecimal amount;
    private final String currency;

    public Money(BigDecimal amount, String currency) {
        this.amount = amount.setScale(2, BigDecimal.ROUND_HALF_EVEN);
        this.currency = currency;
    }

    public static Money of(String amount, String currency) {
        return new Money(new BigDecimal(amount), currency);
    }

    public Money plus(Money other) {
        requireSameCurrency(other);
        return new Money(amount.add(other.amount), currency);
    }

    public Money times(BigDecimal factor) {
        return new Money(amount.multiply(factor), currency);
    }

    public Money percent(BigDecimal pct) {
        return new Money(amount.multiply(pct).divide(new BigDecimal(100), 2, BigDecimal.ROUND_HALF_EVEN), currency);
    }

    public BigDecimal getAmount() { return amount; }
    public String getCurrency() { return currency; }

    private void requireSameCurrency(Money other) {
        if (!currency.equals(other.currency)) {
            throw new IllegalArgumentException("currency mismatch: " + currency + " vs " + other.currency);
        }
    }

    @Override public boolean equals(Object o) {
        if (!(o instanceof Money)) return false;
        Money m = (Money) o;
        return amount.compareTo(m.amount) == 0 && currency.equals(m.currency);
    }

    @Override public int hashCode() { return amount.stripTrailingZeros().hashCode() * 31 + currency.hashCode(); }

    @Override public String toString() { return currency + " " + amount.toPlainString(); }
}
