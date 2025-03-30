use assert_cmd::Command;

#[test]
fn test_blank_nil() {
    let c = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin("\n#exit\n")
        .assert()
        .stdout(">> Nil\n>> ");
}
