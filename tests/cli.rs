use assert_cmd::Command;

#[test]
fn test_empty() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["empty.fuz"])
        .assert()
        .stdout("");
}

#[test]
fn test_12() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["12.fuz"])
        .assert()
        .stdout("");
}
