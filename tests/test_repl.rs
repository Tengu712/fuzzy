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
fn test_empty_block() {
    run("()\n#exit\n", ">> ()\n>> ");
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
fn test_define_variable() {
    run(
        "12 => twelve.\ntwelve\n#exit",
        ">> ()\n>> twelve <= 12 (i32)\n>> ",
    );
}

#[test]
fn test_define_symbol_variable() {
    run(
        "pi => foo.\n3.14f32 -> pi.\nfoo\n#exit",
        ">> ()\n>> ()\n>> foo <= pi <- 3.14 (f32)\n>> ",
    );
}

#[test]
fn test_restrict_define_variable_same_scope() {
    run(
        "1 -> a.\n2 => a.\n3 => a.\n#exit",
        ">> ()\n>> ()\n>> error: cannot redefine variable 'a'.\n>> ",
    )
}

#[test]
fn test_restrict_define_variable_upper_scope() {
    run(
        "(1 -> a. (2 => a. (3 -> a.)))\n#exit",
        ">> error: cannot redefine variable 'a'.\n>> ",
    )
}
