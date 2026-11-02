---
name: golden-check
description: How to prove a change to the billing code (Java or a port) still produces the exact same invoices. Use before opening any pull request that touches billing logic.
---

# Golden check

The files in `fixtures/` are the contract. `expected.txt` in each folder was made by the Java app. Every implementation must match it byte for byte.

1. Run `scripts/golden-check.sh java`. It must print `6/6 fixtures match (java)`. It needs JDK 8 and Maven (`sun.misc.BASE64Encoder` does not exist on newer JDKs).
2. If there is a port in `rust/`, run `scripts/golden-check.sh rust`. It must also print `6/6 fixtures match`.
3. Never edit `expected.txt` by hand, and never regenerate it to make a port pass. Only `scripts/golden-check.sh java --update` may write it, and only when the Java behavior is meant to change.
4. Money is exact. Never use `f32`/`f64` (or Java `double`) for amounts. Round half to even at 2 decimals, the way `Money` does.
5. When a fixture fails, read the diff the script prints. It shows the first lines that differ.
6. Put the output of both commands in the pull request description.

If Maven Central rate-limits you (HTTP 429), use the mirror `https://maven-central.storage-download.googleapis.com/maven2/` in `~/.m2/settings.xml`.
