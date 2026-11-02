#!/usr/bin/env bash
# Runs every fixture in fixtures/ through one implementation and compares
# its output with fixtures/<case>/expected.txt, byte for byte.
#
#   scripts/golden-check.sh java            check the Java app
#   scripts/golden-check.sh rust            check the Rust port in rust/
#   scripts/golden-check.sh java --update   rewrite expected.txt from the Java app
set -euo pipefail
cd "$(dirname "$0")/.."

impl="${1:-}"
update="${2:-}"

case "$impl" in
  java)
    mvn -q -B -DskipTests package
    mvn -q -B dependency:build-classpath -Dmdep.outputFile=target/classpath.txt
    run() { java -Duser.timezone=UTC -cp "target/classes:$(cat target/classpath.txt)" com.acme.billing.GoldenCli "$1"; }
    ;;
  rust)
    if [ ! -f rust/Cargo.toml ]; then
      echo "FAIL: rust/Cargo.toml not found. Nobody has ported the billing code to Rust yet."
      echo "The port must be a cargo workspace in rust/ with a binary named 'golden' that takes"
      echo "a fixture input file and prints its output (see fixtures/README.md)."
      exit 1
    fi
    cargo build -q --release --manifest-path rust/Cargo.toml --bin golden
    run() { rust/target/release/golden "$1"; }
    ;;
  *)
    echo "usage: scripts/golden-check.sh java|rust [--update]" >&2
    exit 2
    ;;
esac

failed=0
total=0
for dir in fixtures/*/; do
  name="$(basename "$dir")"
  total=$((total + 1))
  if [ "$update" = "--update" ]; then
    [ "$impl" = java ] || { echo "--update only works with java" >&2; exit 2; }
    run "$dir/input.txt" > "$dir/expected.txt"
    echo "wrote $dir""expected.txt"
    continue
  fi
  actual="$(mktemp)"
  if ! run "$dir/input.txt" > "$actual"; then
    echo "FAIL $name (the program exited with an error)"
    failed=$((failed + 1))
  elif cmp -s "$actual" "$dir/expected.txt"; then
    echo "PASS $name"
  else
    echo "FAIL $name"
    diff -u --label expected --label actual "$dir/expected.txt" "$actual" | head -20 || true
    failed=$((failed + 1))
  fi
  rm -f "$actual"
done

[ "$update" = "--update" ] && exit 0
echo "$((total - failed))/$total fixtures match ($impl)"
[ "$failed" -eq 0 ]
