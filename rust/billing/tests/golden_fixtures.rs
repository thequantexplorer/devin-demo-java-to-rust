//! Runs every case in fixtures/ through the library and compares with expected.txt.
//! scripts/golden-check.sh does the same through the `golden` binary.

use std::fs;
use std::path::Path;

#[test]
fn every_fixture_matches_expected_output() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
    let mut cases: Vec<_> = fs::read_dir(&fixtures)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_dir())
        .collect();
    cases.sort();
    assert_eq!(
        cases.len(),
        6,
        "expected 6 fixtures in {}",
        fixtures.display()
    );
    for case in cases {
        let input = fs::read_to_string(case.join("input.txt")).unwrap();
        let expected = fs::read_to_string(case.join("expected.txt")).unwrap();
        let actual = acme_billing::golden_cli::run(&input).unwrap();
        assert!(
            actual == expected,
            "{} differs from expected.txt",
            case.display()
        );
    }
}

#[test]
fn unknown_line_is_an_error() {
    let err = acme_billing::golden_cli::run("print\nrefund 1\n").unwrap_err();
    assert_eq!(err.to_string(), "unknown line: refund 1");
}
