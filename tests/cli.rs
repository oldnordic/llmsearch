use assert_cmd::Command;
use predicates::prelude::*;

fn cargo_llmsearch() -> Command {
    Command::from_std(
        assert_cmd::cargo::cargo_bin("llmsearch")
            .expect("Failed to get llmsearch binary path")
    )
}

#[test]
fn test_cli_search_returns_json() {
    // Test that basic search returns valid JSON
    cargo_llmsearch()
        .arg("-p")
        .arg("llmsearch")
        .arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::is_json());
}

#[test]
fn test_cli_with_nonexistent_directory() {
    cargo_llmsearch()
        .arg("-r")
        .arg("/nonexistent/path/xyz123")
        .arg("-p")
        .arg("test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

#[test]
fn test_cli_with_empty_pattern() {
    cargo_llmsearch()
        .arg("-p")
        .arg("")
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be empty"));
}

#[test]
fn test_cli_with_limit_zero() {
    cargo_llmsearch()
        .arg("-p")
        .arg("test")
        .arg("-l")
        .arg("0")
        .assert()
        .failure()
        .stderr(predicate::str::contains("must be at least 1"));
}

#[test]
fn test_cli_no_matches_returns_empty_json() {
    // Create a temp directory with a file
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("no_match_test.txt");
    std::fs::write(&test_file, "no matches here").unwrap();

    cargo_llmsearch()
        .arg("-r")
        .arg(temp_dir.to_str().unwrap())
        .arg("-p")
        .arg("unlikelypatternxyz123")
        .arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::is_json())
        .stdout(predicate::str::contains(r#""match_count":0"#));

    std::fs::remove_file(&test_file).unwrap();
}
