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

#[test]
fn test_add_type_missmatch() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin("1.2f32 + 123\n#exit\n")
        .assert()
        .failure();
}

#[test]
fn test_expression() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin("2 * 3 + 4\n#exit\n")
        .assert()
        .success()
        .stdout(">> I32(14)\n>> ");
}

#[test]
fn test_expression_ordered() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin("(2 * 3) + 4\n#exit\n")
        .assert()
        .success()
        .stdout(">> I32(10)\n>> ");
}
