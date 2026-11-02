package com.acme.billing;

import java.io.IOException;
import java.nio.charset.Charset;
import sun.misc.BASE64Decoder;
import sun.misc.BASE64Encoder;

/** Encodes opaque payment references passed to the legacy PSP bridge. */
public class PaymentTokenCodec {
    private static final Charset UTF8 = Charset.forName("UTF-8");

    public String encode(String customerId, String invoiceId) {
        String raw = customerId + ":" + invoiceId;
        return new BASE64Encoder().encode(raw.getBytes(UTF8));
    }

    public String[] decode(String token) {
        try {
            byte[] bytes = new BASE64Decoder().decodeBuffer(token);
            return new String(bytes, UTF8).split(":", 2);
        } catch (IOException e) {
            throw new IllegalArgumentException("bad token", e);
        }
    }
}
