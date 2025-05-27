use assert_cmd::Command;

fn run(path: &'static str, output: &'static str) {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args([path])
        .assert()
        .success()
        .stdout(output);
}

fn run_wrong(path: &'static str) {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args([path])
        .assert()
        .failure();
}

#[test]
fn test_empty() {
    run("tests/scripts/empty.fuz", "");
}

#[test]
fn test_hello_world() {
    run("tests/scripts/hello-world.fuz", "Hello, world!\n");
}

#[test]
fn test_variable() {
    run("tests/scripts/variable.fuz", "1225\n25\n72\n");
}

#[test]
fn test_implicit_comma() {
    run("tests/scripts/implicit-comma.fuz", "300\n");
}

#[test]
fn test_symbol_value() {
    run("tests/scripts/symbol-value.fuz", "1\n3\n3\n3\n3\n");
}

#[test]
fn test_lazy_block() {
    run("tests/scripts/lazy-block.fuz", "3\nHello, world!\n");
}

#[test]
fn test_array() {
    run(
        "tests/scripts/array.fuz",
        "[1 2.3 hello]\n5\n[1 5 9 2 5]\n3\n2\n3\n[1 bar 3]\n[1 bar baz 3]\n1\n3\n",
    );
}

#[test]
fn test_type_missmatched_addition() {
    run_wrong("tests/scripts/wrong-add.fuz");
}

#[test]
fn test_undefined_value_of_symbol_subject() {
    run_wrong("tests/scripts/wrong-symbol-value-subject.fuz");
}

#[test]
fn test_undefined_value_of_symbol_verb() {
    run_wrong("tests/scripts/wrong-symbol-value-verb.fuz");
}

#[test]
fn test_undefined_value_of_symbol_object() {
    run_wrong("tests/scripts/wrong-symbol-value-object.fuz");
}
