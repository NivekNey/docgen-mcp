use std::env;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use rmcp::{ServerHandler, ServiceExt};

mod mcp;
mod documents;
mod typst;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish()
    ).expect("Failed to set tracing subscriber");

    info!("Starting docgen-mcp server");

    // Check if HTTP mode is requested via --http flag or PORT environment variable
    let args: Vec<String> = env::args().collect();
    let http_mode = args.contains(&"--http".to_string()) || env::var("PORT").is_ok();

    if http_mode {
        run_http_server().await?;
    } else {
        run_stdio_server().await?;
    }

    Ok(())
}

async fn run_stdio_server() -> Result<(), Box<dyn std::error::Error>> {
    use rmcp::transport::async_rw::AsyncRwTransport;
    use tokio::io::{stdin, stdout};

    info!("Starting MCP server with stdio transport (Claude Desktop mode)");

    // Create the server handler
    let server = DocgenServer::new();

    // Create stdio transport
    let transport = AsyncRwTransport::new(stdin(), stdout());

    // Run the server
    server.serve(transport).await?;

    Ok(())
}

async fn run_http_server() -> Result<(), Box<dyn std::error::Error>> {
    use axum::{routing::get, Router};
    use std::net::SocketAddr;

    // Get port from environment or use default
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Starting MCP server with HTTP/SSE transport on {}", addr);

    // Create a basic HTTP router (MCP SSE endpoints will be added here)
    let app = Router::new()
        .route("/", get(|| async { "docgen-mcp server" }));

    info!("HTTP server listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// The main server handler
struct DocgenServer;

impl DocgenServer {
    fn new() -> Self {
        Self
    }
}

impl ServerHandler for DocgenServer {
    // Empty implementation for now - will be filled in later milestones
    // The trait provides default implementations for all methods
}
