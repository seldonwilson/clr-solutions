#[test]
fn runs() {
    assert_cmd::Command::cargo_bin("hello")
        .unwrap()
        .assert()
        .success()
        .stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    assert_cmd::Command::cargo_bin("true")
        .unwrap()
        .assert()
        .success();
}

#[test]
fn false_not_ok() {
    assert_cmd::Command::cargo_bin("false")
        .unwrap()
        .assert()
        .failure();
}