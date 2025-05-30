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
fn test_expression() {
    run("2 * 3 + 4\n#exit", ">> 14 (i32)\n>> ");
}

#[test]
fn test_expression_ordered() {
    run("(2 * 3) + 4\n#exit", ">> 10 (i32)\n>> ");
}

#[test]
fn test_expression_ordered_with_comma() {
    run("2 * 3, + 4\n#exit", ">> 10 (i32)\n>> ");
}

#[test]
fn test_define_variable() {
    run("12 => 'twelve.\ntwelve\n#exit", ">> ()\n>> 12 (i32)\n>> ");
}

#[test]
fn test_define_symbol_variable() {
    run("'a -> 'b.\nb\n#exit", ">> ()\n>> a (symbol)\n>> ");
}

#[test]
fn test_define_defined_symbol_variable() {
    run(
        "'a -> 'b.\n24 -> 'a.\nb\n#exit",
        ">> ()\n>> ()\n>> a <- 24 (i32)\n>> ",
    );
}

#[test]
fn test_define_variable_upper_scope() {
    run(
        "(1 => 'a)\na\n#exit",
        ">> ()\n>> error: undefined variable a found.\n>> ",
    );
}

#[test]
fn test_redefine_variable_same_scope() {
    run(
        "1 -> 'a.\n2 => 'a.\na\n#exit",
        ">> ()\n>> ()\n>> 2 (i32)\n>> ",
    );
}

#[test]
fn test_redefine_variable_upper_scope() {
    run("1 -> 'a. (2 => 'a)\na\n#exit", ">> ()\n>> 2 (i32)\n>> ");
}

#[test]
fn test_restrict_redefine_variable_same_scope() {
    run(
        "1 => 'a.\n2 -> 'a.\n#exit",
        ">> ()\n>> error: cannot redefine variable a.\n>> ",
    );
}

#[test]
fn test_restrict_redefine_variable_upper_scope() {
    run(
        "1 => 'a. (2 -> 'a)\n#exit",
        ">> error: cannot redefine variable a.\n>> ",
    );
}

#[test]
fn test_print() {
    run("1 !\n2 !!\n#exit\n", ">> 11 (i32)\n>> 2\n2 (i32)\n>> ")
}
