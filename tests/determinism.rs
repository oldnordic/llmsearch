use assert_cmd::Command;
use serde_json::Value;

#[test]
fn test_determinism_same_inputs_same_outputs() {
    let temp_dir = std::env::temp_dir();
    let test_dir = temp_dir.join("llmsearch_determinism_test");
    std::fs::create_dir_all(&test_dir).unwrap();

    // Create consistent test file
    let test_file = test_dir.join("test.txt");
    let content = "line one\nline two\nline three\nline one\nline two";
    std::fs::write(&test_file, content).unwrap();

    // Run same search twice and compare outputs
    let output1 = Command::cargo_bin("llmsearch")
        .unwrap()
        .arg("-r")
        .arg(test_dir.to_str().unwrap())
        .arg("-p")
        .arg("line two")
        .arg("--json")
        .output()
        .unwrap()
        .stdout;

    let output2 = Command::cargo_bin("llmsearch")
        .unwrap()
        .arg("-r")
        .arg(test_dir.to_str().unwrap())
        .arg("-p")
        .arg("line two")
        .arg("--json")
        .output()
        .unwrap()
        .stdout;

    // Parse JSON to compare deterministic fields (excluding random UUIDs)
    let json1: Value = serde_json::from_slice(&output1).unwrap();
    let json2: Value = serde_json::from_slice(&output2).unwrap();

    // Pattern should match
    assert_eq!(json1["pattern"], json2["pattern"]);

    // Match count should be identical
    assert_eq!(json1["match_count"], json2["match_count"]);

    // Extract matches and compare deterministic fields
    let matches1 = json1["matches"].as_array().unwrap();
    let matches2 = json2["matches"].as_array().unwrap();

    assert_eq!(matches1.len(), matches2.len());

    for (m1, m2) in matches1.iter().zip(matches2.iter()) {
        // Compare all fields except match_id (which is random)
        assert_eq!(m1["file"], m2["file"]);
        assert_eq!(m1["byte_start"], m2["byte_start"]);
        assert_eq!(m1["byte_end"], m2["byte_end"]);
        assert_eq!(m1["matched_text"], m2["matched_text"]);
        assert_eq!(m1["line_number"], m2["line_number"]);
        assert_eq!(m1["column_number"], m2["column_number"]);
    }

    // Cleanup
    std::fs::remove_file(&test_file).unwrap();
    std::fs::remove_dir(&test_dir).unwrap();
}

#[test]
fn test_determinism_sorting_consistency() {
    let temp_dir = std::env::temp_dir();
    let test_dir = temp_dir.join("llmsearch_sorting_test");
    std::fs::create_dir_all(&test_dir).unwrap();

    // Create multiple files with matches in different order
    let file_b = test_dir.join("b.txt");
    let file_a = test_dir.join("a.txt");
    std::fs::write(&file_b, "match in b\nmatch in b").unwrap();
    std::fs::write(&file_a, "match in a\nmatch in a").unwrap();

    let json_output = Command::cargo_bin("llmsearch")
        .unwrap()
        .arg("-r")
        .arg(test_dir.to_str().unwrap())
        .arg("-p")
        .arg("match")
        .arg("--json")
        .output()
        .unwrap()
        .stdout;

    let json_str = String::from_utf8(json_output).unwrap();
    let output: Value = serde_json::from_str(&json_str).unwrap();
    let matches = output["matches"].as_array().unwrap();

    // Should have 4 matches total, sorted by file path then byte offset
    assert_eq!(matches.len(), 4);

    // First two should be from a.txt (alphabetically before b.txt)
    assert!(matches[0]["file"].as_str().unwrap().ends_with("a.txt"));
    assert!(matches[1]["file"].as_str().unwrap().ends_with("a.txt"));

    // Last two should be from b.txt
    assert!(matches[2]["file"].as_str().unwrap().ends_with("b.txt"));
    assert!(matches[3]["file"].as_str().unwrap().ends_with("b.txt"));

    // Within same file, byte offsets should be ascending
    assert!(matches[0]["byte_start"].as_u64().unwrap() < matches[1]["byte_start"].as_u64().unwrap());
    assert!(matches[2]["byte_start"].as_u64().unwrap() < matches[3]["byte_start"].as_u64().unwrap());

    // Cleanup
    std::fs::remove_file(&file_b).unwrap();
    std::fs::remove_file(&file_a).unwrap();
    std::fs::remove_dir(&test_dir).unwrap();
}

#[test]
fn test_execution_id_unique_per_run() {
    let temp_dir = std::env::temp_dir();
    let test_dir = temp_dir.join("llmsearch_exec_id_test");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("test.txt");
    std::fs::write(&test_file, "test content").unwrap();

    let output1 = Command::cargo_bin("llmsearch")
        .unwrap()
        .arg("-r")
        .arg(test_dir.to_str().unwrap())
        .arg("-p")
        .arg("test")
        .arg("--json")
        .output()
        .unwrap()
        .stdout;

    let output2 = Command::cargo_bin("llmsearch")
        .unwrap()
        .arg("-r")
        .arg(test_dir.to_str().unwrap())
        .arg("-p")
        .arg("test")
        .arg("--json")
        .output()
        .unwrap()
        .stdout;

    let json1: Value = serde_json::from_slice(&output1).unwrap();
    let json2: Value = serde_json::from_slice(&output2).unwrap();

    let exec_id1 = json1["execution_id"].as_str().unwrap();
    let exec_id2 = json2["execution_id"].as_str().unwrap();

    // Execution IDs should be unique (UUID v4)
    assert_ne!(exec_id1, exec_id2, "Each run should have unique execution_id");

    // Both should be valid UUIDs
    assert!(uuid::Uuid::parse_str(exec_id1).is_ok());
    assert!(uuid::Uuid::parse_str(exec_id2).is_ok());

    // Cleanup
    std::fs::remove_file(&test_file).unwrap();
    std::fs::remove_dir(&test_dir).unwrap();
}

#[test]
fn test_limit_returns_first_n_sorted() {
    let temp_dir = std::env::temp_dir();
    let test_dir = temp_dir.join("llmsearch_limit_test");
    std::fs::create_dir_all(&test_dir).unwrap();

    // Create files with many matches
    let test_file = test_dir.join("test.txt");
    let content = "match\nmatch\nmatch\nmatch\nmatch\nmatch\nmatch\nmatch\nmatch\nmatch";
    std::fs::write(&test_file, content).unwrap();

    // Request limit of 5
    let json_output = Command::cargo_bin("llmsearch")
        .unwrap()
        .arg("-r")
        .arg(test_dir.to_str().unwrap())
        .arg("-p")
        .arg("match")
        .arg("-l")
        .arg("5")
        .arg("--json")
        .output()
        .unwrap()
        .stdout;

    let json_str = String::from_utf8(json_output).unwrap();
    let output: Value = serde_json::from_str(&json_str).unwrap();

    // Should have exactly 5 matches
    assert_eq!(output["match_count"].as_u64().unwrap(), 5);
    assert_eq!(output["matches"].as_array().unwrap().len(), 5);

    // All matches should be from same file (only one file exists)
    for m in output["matches"].as_array().unwrap() {
        assert!(m["file"].as_str().unwrap().ends_with("test.txt"));
    }

    // Byte offsets should be ascending (deterministic ordering)
    let mut prev_byte = 0;
    for m in output["matches"].as_array().unwrap() {
        let byte = m["byte_start"].as_u64().unwrap();
        assert!(byte >= prev_byte, "Byte offsets should be ascending");
        prev_byte = byte;
    }

    // Cleanup
    std::fs::remove_file(&test_file).unwrap();
    std::fs::remove_dir(&test_dir).unwrap();
}
