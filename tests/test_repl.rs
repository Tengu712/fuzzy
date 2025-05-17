use assert_cmd::Command;

fn run(input: &'static str, output: &'static str) {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(output);
}

#[test]
fn test_nil() {
    run("\n#exit\n", ">> ()\n>> ");
}

#[test]
fn test_float() {
    run("1.2f32\n#exit", ">> 1.2 (f32)\n>> ");
}

#[test]
fn test_add_type_missmatch() {
    run(
        "1.2f32 + 123\n#exit",
        ">> error: type missmatched argument passed to 'add:+'.\n>> ",
    );
}

#[test]
fn test_expression() {
    run("2 * 3 + 4\n#exit", ">> 14 (i32)\n>> ");
}

#[test]
fn test_expression_ordered() {
    run("(2 * 3) + 4\n#exit", ">> 10 (i32)\n>> ");
}

#[test]
fn test_make_variable() {
    run(
        "12 => twelve.\ntwelve\n#exit",
        ">> ()\n>> twelve => 12 (i32)\n>> ",
    );
}

#[test]
fn test_make_symbol_variable() {
    run(
        "pi => foo.\n3.14f32 -> pi.\nfoo\n#exit",
        ">> ()\n>> ()\n>> foo => pi -> 3.14 (f32)\n>> ",
    );
}
