use rmcp::{ErrorData, ServerHandler, ServiceExt, model::*};
use std::env;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

mod documents;
mod mcp;
mod typst;

use mcp::{prompts, resources, tools};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish(),
    )
    .expect("Failed to set tracing subscriber");

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
    use rmcp::transport::streamable_http_server::{
        StreamableHttpService, session::local::LocalSessionManager,
    };
    use std::net::SocketAddr;

    // Get port from environment or use default
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!(
        "Starting MCP server with Streamable HTTP transport on {}",
        addr
    );

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
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
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
                 documents like resumes and CVs.\n\n\
                 RECOMMENDED WORKFLOW FOR AI AGENTS:\n\
                 1. Use 'get_resume_best_practices' tool to understand writing guidelines and best practices\n\
                 2. Use 'get_resume_schema' tool to see the exact JSON structure required\n\
                 3. Gather information from the user and construct the resume JSON\n\
                 4. Use 'validate_resume' tool to check the JSON structure\n\
                 5. Use 'generate_resume' tool to create the final PDF\n\n\
                 Following this workflow ensures high-quality, properly structured documents.\n\n\
                 ALTERNATIVE: Advanced agents can also use:\n\
                 - PROMPT 'resume-best-practices' for comprehensive guidance\n\
                 - RESOURCE 'docgen://schemas/resume' for schema definition"
                    .to_string(),
            ),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        Ok(ListResourcesResult {
            resources: resources::list_resources(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParam,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ReadResourceResult, ErrorData> {
        match resources::read_resource(&request.uri) {
            Some(contents) => Ok(ReadResourceResult {
                contents: vec![contents],
            }),
            None => Err(ErrorData::resource_not_found(
                format!("Resource not found: {}", request.uri),
                None,
            )),
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ListPromptsResult, ErrorData> {
        Ok(ListPromptsResult {
            prompts: prompts::list_prompts(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParam,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<GetPromptResult, ErrorData> {
        match prompts::get_prompt(&request.name) {
            Some(result) => Ok(result),
            None => Err(ErrorData::resource_not_found(
                format!("Prompt not found: {}", request.name),
                None,
            )),
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult {
            tools: tools::list_tools(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        // Convert Map<String, Value> to Value::Object
        let arguments = serde_json::Value::Object(request.arguments.unwrap_or_default());

        match tools::call_tool(&request.name, arguments) {
            Ok(result) => Ok(CallToolResult::structured(result)),
            Err(e) => Ok(CallToolResult::structured_error(serde_json::json!({
                "error": e
            }))),
        }
    }
}
