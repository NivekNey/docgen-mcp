use tokio::process::Command;
use tokio::time::{timeout, Duration};

/// Test that the stdio server starts and runs without crashing
#[tokio::test]
async fn test_stdio_server_starts() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_docgen-mcp"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start stdio server");

    // Give the server a moment to start up
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify the process is still running (hasn't crashed)
    match child.try_wait() {
        Ok(Some(status)) => {
            panic!("Server exited prematurely with status: {}", status);
        }
        Ok(None) => {
            // Process is still running, which is what we want
        }
        Err(e) => {
            panic!("Error checking server status: {}", e);
        }
    }

    // Clean up
    child.kill().await.expect("Failed to kill stdio server");
}

/// Test that the server properly shuts down when killed
#[tokio::test]
async fn test_stdio_server_shutdown() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_docgen-mcp"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start stdio server");

    // Give the server a moment to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Kill the server
    child.kill().await.expect("Failed to kill stdio server");

    // Wait for the process to exit
    let result = timeout(Duration::from_secs(5), child.wait()).await;

    match result {
        Ok(Ok(_)) => {
            // Server exited successfully
        }
        Ok(Err(e)) => {
            panic!("Error waiting for server to exit: {}", e);
        }
        Err(_) => {
            panic!("Server did not exit within timeout");
        }
    }
}

#[tokio::test]
async fn test_http_server_starts() {
    // Start server in HTTP mode
    let mut child = Command::new(env!("CARGO_BIN_EXE_docgen-mcp"))
        .arg("--http")
        .env("PORT", "3001")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start HTTP server");

    // Give server time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Try to connect to the server
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3001/mcp")
        .send()
        .await;

    // The endpoint should be accessible (even if it returns an error about missing headers)
    assert!(response.is_ok(), "HTTP server should be reachable");

    // Clean up
    child.kill().await.expect("Failed to kill HTTP server");
}
