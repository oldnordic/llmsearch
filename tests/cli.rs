use assert_cmd::Command;

#[test]
fn test_cli_search_returns_json() {
    // Test that basic search returns valid JSON
    Command::cargo_bin("llmsearch")
        .arg("-p")
        .arg("llmsearch")
        .arg("--json")
        .assert()
        .success()
        .stdout(predicates::str::is_json());
}

#[test]
fn test_cli_with_nonexistent_directory() {
    Command::cargo_bin("llmsearch")
        .arg("-r")
        .arg("/nonexistent/path/xyz123")
        .arg("-p")
        .arg("test")
        .assert()
        .failure()
        .stderr(predicates::str::contains("does not exist"));
}

#[test]
fn test_cli_with_empty_pattern() {
    Command::cargo_bin("llmsearch")
        .arg("-p")
        .arg("")
        .assert()
        .failure()
        .stderr(predicates::str::contains("cannot be empty"));
}

#[test]
fn test_cli_with_limit_zero() {
    Command::cargo_bin("llmsearch")
        .arg("-p")
        .arg("test")
        .arg("-l")
        .arg("0")
        .assert()
        .failure()
        .stderr(predicates::str::contains("must be at least 1"));
}

#[test]
fn test_cli_no_matches_returns_empty_json() {
    // Create a temp directory with a file
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("no_match_test.txt");
    std::fs::write(&test_file, "no matches here").unwrap();

    Command::cargo_bin("llmsearch")
        .arg("-r")
        .arg(temp_dir.to_str().unwrap())
        .arg("-p")
        .arg("unlikelypatternxyz123")
        .arg("--json")
        .assert()
        .success()
        .stdout(predicates::str::is_json())
        .stdout(predicates::str::contains(r#""match_count":0"#));

    std::fs::remove_file(&test_file).unwrap();
}
