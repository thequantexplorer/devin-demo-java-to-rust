package com.acme.billing;

import java.util.Calendar;
import java.util.Date;
import java.util.TimeZone;

/** Net-N payment terms. Weekends roll forward to Monday. */
public class DueDateCalculator {
    private static final TimeZone UTC = TimeZone.getTimeZone("UTC");

    public Date dueDate(Date issuedAt, int netDays) {
        Calendar cal = Calendar.getInstance(UTC);
        cal.setTime(issuedAt);
        cal.add(Calendar.DAY_OF_MONTH, netDays);
        int dow = cal.get(Calendar.DAY_OF_WEEK);
        if (dow == Calendar.SATURDAY) cal.add(Calendar.DAY_OF_MONTH, 2);
        if (dow == Calendar.SUNDAY) cal.add(Calendar.DAY_OF_MONTH, 1);
        return cal.getTime();
    }

    public boolean isOverdue(Date dueAt, Date now) {
        return now.after(dueAt);
    }
}
