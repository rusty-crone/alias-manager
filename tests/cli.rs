use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("ram").unwrap();
    cmd.assert().success();
}