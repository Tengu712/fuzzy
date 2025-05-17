use assert_cmd::Command;

#[test]
fn test_nil() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin("\n#exit\n")
        .assert()
        .success()
        .stdout(">> ()\n>> ");
}

#[test]
fn test_float() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin("1.2f32\n#exit\n")
        .assert()
        .success()
        .stdout(">> 1.2 (f32)\n>> ");
}

#[test]
fn test_add_type_missmatch() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin("1.2f32 + 123\n#exit\n")
        .assert()
        .success()
        .stdout(">> error: type missmatched argument passed to 'add:+'.\n>> ");
}

#[test]
fn test_expression() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin("2 * 3 + 4\n#exit\n")
        .assert()
        .success()
        .stdout(">> 14 (i32)\n>> ");
}

#[test]
fn test_expression_ordered() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin("(2 * 3) + 4\n#exit\n")
        .assert()
        .success()
        .stdout(">> 10 (i32)\n>> ");
}

#[test]
fn test_make_variable() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin("12 => twelve.\ntwelve\n#exit\n")
        .assert()
        .success()
        .stdout(">> ()\n>> twelve => 12 (i32)\n>> ");
}
