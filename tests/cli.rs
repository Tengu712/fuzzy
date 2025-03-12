use assert_cmd::Command;

#[test]
fn test_empty() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("tests/empty.fuz")
        .assert()
        .success()
        .stdout("");
}

#[test]
fn test_12() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("tests/12.fuz")
        .assert()
        .success()
        .stdout("");
}
