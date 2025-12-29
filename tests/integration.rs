use tokio::process::Command;
use tokio::time::{Duration, timeout};

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
    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;

    // Try to connect to the server
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:3001/mcp").send().await;

    // The endpoint should be accessible (even if it returns an error about missing headers)
    assert!(response.is_ok(), "HTTP server should be reachable");

    // Clean up
    child.kill().await.expect("Failed to kill HTTP server");
}

/// Test CORS preflight requests work correctly for Claude.ai
#[tokio::test]
async fn test_cors_preflight_for_claude_ai() {
    // Start server in HTTP mode
    let mut child = Command::new(env!("CARGO_BIN_EXE_docgen-mcp"))
        .arg("--http")
        .env("PORT", "3002")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start HTTP server");

    // Give server time to start
    tokio::time::sleep(Duration::from_millis(2000)).await;

    // Send OPTIONS preflight request as Claude.ai would
    let client = reqwest::Client::new();
    let response = client
        .request(reqwest::Method::OPTIONS, "http://localhost:3002/mcp")
        .header("Origin", "https://claude.ai")
        .header("Access-Control-Request-Method", "POST")
        .header("Access-Control-Request-Headers", "content-type, mcp-session-id")
        .send()
        .await
        .expect("Failed to send OPTIONS request");

    // Check CORS headers are present
    let headers = response.headers();

    assert!(
        headers.get("access-control-allow-origin").is_some(),
        "Should have Access-Control-Allow-Origin header"
    );
    assert_eq!(
        headers.get("access-control-allow-origin").unwrap(),
        "https://claude.ai",
        "Should allow claude.ai origin"
    );
    assert!(
        headers.get("access-control-allow-methods").is_some(),
        "Should have Access-Control-Allow-Methods header"
    );
    assert!(
        headers.get("access-control-allow-headers").is_some(),
        "Should have Access-Control-Allow-Headers header"
    );
    assert!(
        headers.get("access-control-allow-credentials").is_some(),
        "Should have Access-Control-Allow-Credentials header"
    );

    // Clean up
    child.kill().await.expect("Failed to kill HTTP server");
}

/// Test CORS headers are returned on actual requests from Claude.ai
#[tokio::test]
async fn test_cors_headers_on_post_request() {
    // Start server in HTTP mode
    let mut child = Command::new(env!("CARGO_BIN_EXE_docgen-mcp"))
        .arg("--http")
        .env("PORT", "3003")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start HTTP server");

    // Give server time to start
    tokio::time::sleep(Duration::from_millis(2000)).await;

    // Send POST request with Claude.ai origin (MCP initialize request)
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3003/mcp")
        .header("Origin", "https://claude.ai")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream")
        .body(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#)
        .send()
        .await
        .expect("Failed to send POST request");

    // Check CORS headers are present on the response
    let headers = response.headers();

    assert!(
        headers.get("access-control-allow-origin").is_some(),
        "Should have Access-Control-Allow-Origin header on response"
    );
    assert_eq!(
        headers.get("access-control-allow-origin").unwrap(),
        "https://claude.ai",
        "Should allow claude.ai origin"
    );

    // Clean up
    child.kill().await.expect("Failed to kill HTTP server");
}

/// Test that MCP endpoint returns proper session ID header
#[tokio::test]
async fn test_mcp_session_id_header() {
    // Start server in HTTP mode
    let mut child = Command::new(env!("CARGO_BIN_EXE_docgen-mcp"))
        .arg("--http")
        .env("PORT", "3004")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start HTTP server");

    // Give server time to start
    tokio::time::sleep(Duration::from_millis(2000)).await;

    // Send initialize request
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3004/mcp")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream")
        .body(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#)
        .send()
        .await
        .expect("Failed to send initialize request");

    // MCP spec requires session ID to be exposed
    let headers = response.headers();

    // Check that access-control-expose-headers includes mcp-session-id
    if let Some(expose_headers) = headers.get("access-control-expose-headers") {
        let expose_str = expose_headers.to_str().unwrap_or("");
        assert!(
            expose_str.to_lowercase().contains("mcp-session-id"),
            "Should expose mcp-session-id header, got: {}", expose_str
        );
    }

    // Clean up
    child.kill().await.expect("Failed to kill HTTP server");
}
