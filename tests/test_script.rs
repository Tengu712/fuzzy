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
fn test_type_missmatched_addition() {
    run_wrong("tests/scripts/wrong-add.fuz");
}
