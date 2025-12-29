use std::env;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use rmcp::{
    ServerHandler,
    ServiceExt,
    model::*,
};

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
    use axum::Router;
    use std::net::SocketAddr;
    use rmcp::transport::streamable_http_server::{
        StreamableHttpService,
        session::local::LocalSessionManager,
    };

    // Get port from environment or use default
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Starting MCP server with Streamable HTTP transport on {}", addr);

    // Create the streamable HTTP service
    let service = StreamableHttpService::new(
        || Ok(DocgenServer::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    // Create axum router with MCP endpoint
    let app = Router::new().nest_service("/mcp", service);

    info!("MCP server listening on {} (endpoint: /mcp)", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap();
        })
        .await?;

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
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ServerCapabilities::builder().build(),
            server_info: Implementation {
                name: "docgen-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                title: Some("Document Generation MCP Server".to_string()),
                website_url: None,
                icons: None,
            },
            instructions: Some(
                "A Model Context Protocol server for programmatic document generation, \
                 powered by Typst. Use this server to generate professionally typeset \
                 documents like resumes and CVs.".to_string()
            ),
        }
    }
}
