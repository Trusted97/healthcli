use assert_cmd::Command;

#[test]
fn test_list_without_check_command() {
    let mut cmd = Command::cargo_bin("HealthCLI").unwrap();
    cmd.arg("list")
        .assert()
        .success()
        .stdout(predicates::str::contains("No health check registered."));
}

#[test]
fn test_register_command() {
    let mut cmd = Command::cargo_bin("HealthCLI").unwrap();
    cmd.args(["register", "-n", "test_check", "--check-type", "url"])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "Health check test_check of type url registered successfully",
        ));
}

#[test]
fn test_check_command_all() {
    let mut cmd = Command::cargo_bin("HealthCLI").unwrap();
    cmd.arg("check")
        .assert()
        .success()
        .stdout(predicates::str::contains("Running all health checks..."));
}

#[test]
fn test_check_command_specific() {
    let mut cmd = Command::cargo_bin("HealthCLI").unwrap();

    cmd.args(["register", "--name", "test_check", "--check-type", "url"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("HealthCLI").unwrap();

    cmd.args(["check", "--name", "test_check"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Running check: test_check"));
}

#[test]
fn test_check_command_remove() {
    let mut cmd = Command::cargo_bin("HealthCLI").unwrap();

    cmd.args(["register", "-n", "test_check", "--check-type", "url"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("HealthCLI").unwrap();

    cmd.args(["check", "--name", "test_check", "--remove"])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "Check test_check has been removed",
        ));
}
