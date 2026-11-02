package com.acme.billing;

import java.io.BufferedReader;
import java.io.FileInputStream;
import java.io.InputStreamReader;
import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.Date;
import java.util.List;
import java.util.TimeZone;

/**
 * Reads a fixture file (see fixtures/README.md) and prints the billing output for it.
 * The files in fixtures/ were made with this class and are the expected output for any port.
 */
public class GoldenCli {
    public static void main(String[] args) throws Exception {
        if (args.length != 1) {
            System.err.println("usage: GoldenCli <input.txt>");
            System.exit(2);
        }
        SimpleDateFormat iso = new SimpleDateFormat("yyyy-MM-dd");
        iso.setTimeZone(TimeZone.getTimeZone("UTC"));
        iso.setLenient(false);

        TaxCalculator tax = new TaxCalculator();
        DueDateCalculator due = new DueDateCalculator();
        InvoiceFormatter fmt = new InvoiceFormatter();
        InvoiceXml xml = new InvoiceXml();
        PaymentTokenCodec codec = new PaymentTokenCodec();
        AgingReport aging = new AgingReport();

        List<Invoice> invoices = new ArrayList<Invoice>();
        List<String> regions = new ArrayList<String>();
        StringBuilder out = new StringBuilder();

        BufferedReader in = new BufferedReader(new InputStreamReader(new FileInputStream(args[0]), "UTF-8"));
        String line;
        while ((line = in.readLine()) != null) {
            line = line.trim();
            if (line.isEmpty() || line.startsWith("#")) continue;
            String[] f = line.split("\\s+");
            if (f[0].equals("invoice")) {
                Date issued = iso.parse(f[4]);
                invoices.add(new Invoice(f[1], f[2], f[3], issued, due.dueDate(issued, Integer.parseInt(f[5]))));
                regions.add(f[6]);
            } else if (f[0].equals("item")) {
                Invoice inv = invoices.get(invoices.size() - 1);
                String currency = f.length > 4 ? f[4] : inv.getCurrency();
                inv.addItem(new LineItem(f[1], Integer.parseInt(f[2]), Money.of(f[3], currency)));
            } else if (f[0].equals("print")) {
                for (int i = 0; i < invoices.size(); i++) {
                    Invoice inv = invoices.get(i);
                    try {
                        String csv = fmt.toCsvRow(inv, tax.taxFor(inv, regions.get(i)));
                        String envelope = xml.envelope(inv);
                        out.append("csv,").append(csv).append('\n');
                        out.append("xml,").append(envelope).append('\n');
                    } catch (IllegalArgumentException e) {
                        out.append("error,").append(inv.getId()).append(',').append(e.getMessage()).append('\n');
                    }
                    String token = codec.encode(inv.getCustomerId(), inv.getId());
                    out.append("token,").append(token.replace("\r", "\\r").replace("\n", "\\n")).append('\n');
                }
            } else if (f[0].equals("aging")) {
                int[] b = aging.bucket(invoices, iso.parse(f[1]));
                out.append("aging,").append(f[1]);
                for (int n : b) out.append(',').append(n);
                out.append('\n');
            } else if (f[0].equals("batch")) {
                for (String row : BillingRun.run(Integer.parseInt(f[1]), Long.parseLong(f[2]))) {
                    out.append("batch,").append(row).append('\n');
                }
            } else {
                throw new IllegalArgumentException("unknown line: " + line);
            }
        }
        in.close();
        System.out.print(out);
    }
}
