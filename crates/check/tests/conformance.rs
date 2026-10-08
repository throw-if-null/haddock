//! Run the bats suite of the repository against the binary of this crate. The suite is the
//! conformance oracle: a fixture whose output differs from its `.expected` file fails here,
//! and `cargo mutants` runs this test for each mutant. The suite needs bats-core on PATH.

use std::path::Path;
use std::process::Command;

#[test]
fn bats_suite_passes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    // Cargo builds the binary of the package before its integration tests, and sets
    // CARGO_BIN_EXE_<name> to its path at compile time.
    let output = Command::new(root.join("tests/run"))
        .env("CHECK", env!("CARGO_BIN_EXE_doc-style-check"))
        .output()
        .expect("tests/run starts");
    assert!(
        output.status.success(),
        "tests/run failed:\n{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
