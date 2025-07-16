use std::process::{Command, Stdio};
use std::io::Write;

#[test]
fn test_binary_help_flag() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to run binary");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("claude-emotion"));
    assert!(stdout.contains("--install-shell"));
    assert!(stdout.contains("--list-emotions"));
    assert!(output.status.success());
}

#[test]
fn test_binary_version_flag() {
    let output = Command::new("cargo")
        .args(["run", "--", "--version"])
        .output()
        .expect("Failed to run binary");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.1.0"));
    assert!(output.status.success());
}

#[test]
fn test_binary_list_emotions() {
    let output = Command::new("cargo")
        .args(["run", "--", "--list-emotions"])
        .output()
        .expect("Failed to run binary");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("analytical"));
    assert!(stdout.contains("helpful"));
    assert!(stdout.contains("curious"));
    assert!(stdout.contains("uncertain"));
    assert!(stdout.contains("apologetic"));
    assert!(stdout.contains("excited"));
    assert!(output.status.success());
}

#[test]
fn test_stream_processing_with_emotion() {
    let mut child = Command::new("cargo")
        .args(["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");
    
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"[analytical] This is a test response\n").unwrap();
    stdin.flush().unwrap();
    drop(stdin);
    
    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("╭─────────────────────╮"));
    assert!(stdout.contains("Analytical"));
    assert!(stdout.contains("( ･ω･)つ━☆"));
    assert!(stdout.contains("This is a test response"));
    assert!(output.status.success());
}

#[test]
fn test_stream_processing_without_emotion() {
    let mut child = Command::new("cargo")
        .args(["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");
    
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"Regular response without emotion tag\n").unwrap();
    stdin.flush().unwrap();
    drop(stdin);
    
    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert_eq!(stdout, "Regular response without emotion tag\n");
    assert!(output.status.success());
}

#[test]
fn test_stream_processing_invalid_emotion() {
    let mut child = Command::new("cargo")
        .args(["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");
    
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"[invalid] This should pass through\n").unwrap();
    stdin.flush().unwrap();
    drop(stdin);
    
    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert_eq!(stdout, "[invalid] This should pass through\n");
    assert!(output.status.success());
}

#[test]
fn test_stream_processing_multiline() {
    let mut child = Command::new("cargo")
        .args(["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");
    
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"[helpful] First line\nSecond line\nThird line\n").unwrap();
    stdin.flush().unwrap();
    drop(stdin);
    
    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("Helpful"));
    assert!(stdout.contains("First line"));
    assert!(stdout.contains("Second line"));
    assert!(stdout.contains("Third line"));
    assert!(output.status.success());
}

#[test]
fn test_stream_processing_utf8() {
    let mut child = Command::new("cargo")
        .args(["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");
    
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all("[excited] 这是中文测试! 🎉 émojis\n".as_bytes()).unwrap();
    stdin.flush().unwrap();
    drop(stdin);
    
    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("Excited"));
    assert!(stdout.contains("这是中文测试! 🎉 émojis"));
    assert!(output.status.success());
}

#[test]
fn test_all_emotions_work() {
    let emotions = vec![
        "analytical", "helpful", "curious", 
        "uncertain", "apologetic", "excited"
    ];
    
    for emotion in emotions {
        let mut child = Command::new("cargo")
            .args(["run"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start process");
        
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        let input = format!("[{}] Test response\n", emotion);
        stdin.write_all(input.as_bytes()).unwrap();
        stdin.flush().unwrap();
        drop(stdin);
        
        let output = child.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        assert!(stdout.contains("╭─────────────────────╮"), "Box not found for {}", emotion);
        assert!(stdout.contains("Test response"), "Content not found for {}", emotion);
        assert!(output.status.success(), "Process failed for {}", emotion);
    }
}