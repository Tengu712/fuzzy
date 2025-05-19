use assert_cmd::Command;

fn run(path: &'static str, output: &'static str) {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args([path])
        .assert()
        .success()
        .stdout(output);
}

#[test]
fn test_empty() {
    run("tests/scripts/empty.fuz", "");
}
