# Devin demo: Port a Java service to Rust

**What it shows:** Devin ports a legacy Java 8 billing app to a Rust crate. Golden fixtures made from the Java app prove the port prints exactly the same invoices. Devin also ports the unit tests, adds a CI job, and writes a short porting guide.

**Needs:** GitHub only (Devin installs JDK 8, Maven and Rust itself)

More demos: [devin-advanced-demos](https://github.com/thequantexplorer/devin-advanced-demos)

## Set it up

1. Click **Use this template → Create a new repository**, choose your GitHub account or org as the owner, and keep the name `devin-demo-java-to-rust`. The prompt below assumes that name. Use the template rather than a fork: pull requests on a fork go to the original repo by default.
2. In Devin, open [**Settings → Connections → GitHub**](https://app.devin.ai/settings/connections/github) and make sure Devin can access the new repository.

## Prompt

Go to [app.devin.ai](https://app.devin.ai), start a new session, and paste this. Replace `<you>` with your GitHub user or org.

```text
Port the billing logic in https://github.com/<you>/devin-demo-java-to-rust from Java to Rust.

First run scripts/golden-check.sh java (it needs JDK 8 and Maven) and scripts/golden-check.sh rust. The Java check passes. The Rust check fails because there is no port yet. Read fixtures/README.md and the golden-check skill in .agents/skills/.

Then:
1. Create a cargo workspace in rust/ with a library crate for the billing logic (one module per Java class where it makes sense) and a binary named golden that reads a fixture input file and prints the same output as com.acme.billing.GoldenCli.
2. Handle money exactly: use integer cents or a decimal crate, and round half to even like Money does. Do not use f32 or f64 for money anywhere.
3. Port every JUnit test in src/test/java to Rust tests with the same cases and expected values.
4. Make scripts/golden-check.sh rust pass all 6 fixtures. Do not change the fixtures, the Java code or the script.
5. Add a rust job to .github/workflows/ci.yml that runs cargo test and scripts/golden-check.sh rust. Keep the java job.
6. Write PORTING.md: a table that maps each Java class to its Rust module, and a short list of the behaviors that were easy to get wrong (rounding, Base64 line breaks, java.util.Random, dates) and how the port matches each one.

Open a pull request. In the description, paste the output of cargo test and of both golden checks, and say whether the CI checks are green.
```

## What to watch

- Devin runs both golden checks first. The Rust one fails because `rust/` does not exist.
- It uses integer cents or a decimal type for money, never `f64`, and reproduces `java.util.Random` so the batch fixture matches.
- When a fixture fails, it reads the diff and fixes the Rust code. It does not touch `expected.txt`.

## Done when

- `scripts/golden-check.sh rust` prints `6/6 fixtures match (rust)`, and the **CI** checks on the pull request are green, including the new rust job.
- The pull request has the Rust crate, the ported tests, the CI job and `PORTING.md`.
