use assert_cmd::Command;

#[test]
fn test_nil() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin("\n#exit\n")
        .assert()
        .success()
        .stdout(">> Nil\n>> ");
}

#[test]
fn test_float() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin("1.2f32\n#exit\n")
        .assert()
        .success()
        .stdout(">> F32(1.2)\n>> ");
}
