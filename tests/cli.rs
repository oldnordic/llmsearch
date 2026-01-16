use assert_cmd::Command;
use predicates::prelude::*;

fn cargo_llmsearch() -> Command {
    Command::cargo_bin("llmsearch")
        .expect("Failed to find llmsearch binary")
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
        .stdout(predicate::str::starts_with("{"));
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
        .stdout(predicate::str::contains(r#""match_count":0"#));

    std::fs::remove_file(&test_file).unwrap();
}

#[test]
fn test_cli_glob_filters_results() {
    let temp_dir = std::env::temp_dir();

    // Create test files
    let rust_file = temp_dir.join("test.rs");
    let txt_file = temp_dir.join("test.txt");
    std::fs::write(&rust_file, "fn main() {}").unwrap();
    std::fs::write(&txt_file, "some text").unwrap();

    let json_output = cargo_llmsearch()
        .arg("-r")
        .arg(temp_dir.to_str().unwrap())
        .arg("-p")
        .arg("main")
        .arg("-g")
        .arg("*.rs")
        .arg("--json")
        .output()
        .unwrap()
        .stdout;

    // Convert bytes to string
    let json_str = std::str::from_utf8(&json_output).unwrap();

    // Parse JSON and verify only .rs file found
    let output: serde_json::Value = serde_json::from_str(json_str).unwrap();
    let matches = &output["matches"];

    // Filter to only matches in our test files (temp dir may have other files)
    let test_matches: Vec<_> = matches.as_array().unwrap()
        .iter()
        .filter(|m| m["file"].as_str().unwrap().ends_with("test.rs") ||
                    m["file"].as_str().unwrap().ends_with("test.txt"))
        .collect();

    // Should have exactly 1 match from test.rs (not test.txt due to glob filter)
    assert_eq!(test_matches.len(), 1);
    let file_path = test_matches[0]["file"].as_str().unwrap();
    assert!(file_path.ends_with("test.rs"));

    // Cleanup
    std::fs::remove_file(&rust_file).unwrap();
    std::fs::remove_file(&txt_file).unwrap();
}
