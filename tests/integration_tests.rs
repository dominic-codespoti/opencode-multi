use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("opencode-multi").unwrap();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "Multi-profile manager for OpenCode",
    ));
}

#[test]
fn test_create_list_show_remove_workflow() {
    // Use a temporary directory for testing
    let _temp_dir = TempDir::new().unwrap();
    let profile_name = "integration-test-profile";

    // Create profile
    let mut cmd = Command::cargo_bin("opencode-multi").unwrap();
    cmd.arg("create").arg(profile_name);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created profile"));

    // List profiles (should show our new profile)
    let mut cmd = Command::cargo_bin("opencode-multi").unwrap();
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(profile_name));

    // Show profile details
    let mut cmd = Command::cargo_bin("opencode-multi").unwrap();
    cmd.arg("show").arg(profile_name);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Profile:"))
        .stdout(predicate::str::contains("needs-auth"));

    // Clean up - remove profile
    let mut cmd = Command::cargo_bin("opencode-multi").unwrap();
    cmd.arg("remove").arg(profile_name).arg("--yes");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Removed profile"));
}

#[test]
fn test_create_duplicate_profile_fails() {
    let profile_name = "duplicate-test-profile";

    // Create profile
    let mut cmd = Command::cargo_bin("opencode-multi").unwrap();
    cmd.arg("create").arg(profile_name);
    cmd.assert().success();

    // Try to create again - should fail
    let mut cmd = Command::cargo_bin("opencode-multi").unwrap();
    cmd.arg("create").arg(profile_name);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));

    // Clean up
    let mut cmd = Command::cargo_bin("opencode-multi").unwrap();
    cmd.arg("remove").arg(profile_name).arg("--yes");
    cmd.assert().success();
}

#[test]
fn test_invalid_profile_name() {
    let mut cmd = Command::cargo_bin("opencode-multi").unwrap();
    cmd.arg("create").arg("invalid name with spaces");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("can only contain alphanumeric"));
}

#[test]
fn test_doctor_command() {
    let mut cmd = Command::cargo_bin("opencode-multi").unwrap();
    cmd.arg("doctor");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("OpenCode-Multi Doctor"));
}
