use rmcp::{ErrorData, ServerHandler, ServiceExt, model::*};
use std::env;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

mod documents;
mod mcp;
mod storage;
mod typst;

use mcp::{prompts, resources, tools};
use storage::FileStorage;

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

    // Create the server handler (no file storage or base URL for stdio mode)
    let server = DocgenServer::new(None, None);

    // Create stdio transport
    let transport = AsyncRwTransport::new(stdin(), stdout());

    // Run the server
    server.serve(transport).await?;

    Ok(())
}

async fn run_http_server() -> Result<(), Box<dyn std::error::Error>> {
    use axum::{
        Router,
        extract::{Path, State},
        http::{StatusCode, header},
        response::{IntoResponse, Response},
    };
    use rmcp::transport::streamable_http_server::{
        StreamableHttpService, session::local::LocalSessionManager,
    };
    use std::net::SocketAddr;
    use uuid::Uuid;

    // Get port from environment or use default
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // Determine base URL for download links
    // Use BASE_URL env var if set (for production), otherwise construct from port
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| {
        format!("http://localhost:{}", port)
    });

    info!(
        "Starting MCP server with Streamable HTTP transport on {}",
        addr
    );
    info!("Download URL base: {}", base_url);

    // Create file storage and start cleanup task
    let file_storage = FileStorage::new();
    file_storage.clone().start_cleanup_task();

    // Create the streamable HTTP service with storage
    let storage_clone = file_storage.clone();
    let base_url_clone = base_url.clone();
    let service = StreamableHttpService::new(
        move || Ok(DocgenServer::new(Some(storage_clone.clone()), Some(base_url_clone.clone()))),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    // File download handler
    async fn download_file(
        State(storage): State<FileStorage>,
        Path(file_id): Path<String>,
    ) -> Response {
        // Parse UUID
        let id = match Uuid::parse_str(&file_id) {
            Ok(id) => id,
            Err(_) => {
                return (StatusCode::BAD_REQUEST, "Invalid file ID").into_response();
            }
        };

        // Retrieve file
        match storage.retrieve(&id).await {
            Some(file) => {
                // Return the PDF with appropriate headers
                (
                    StatusCode::OK,
                    [
                        (header::CONTENT_TYPE, "application/pdf"),
                        (
                            header::CONTENT_DISPOSITION,
                            &format!("inline; filename=\"{}\"", file.filename),
                        ),
                        (header::CACHE_CONTROL, "no-store, must-revalidate"),
                    ],
                    file.data,
                )
                    .into_response()
            }
            None => (StatusCode::NOT_FOUND, "File not found or expired").into_response(),
        }
    }

    // Create axum router with MCP endpoint and file downloads
    let app = Router::new()
        .nest_service("/mcp", service)
        .route("/files/{id}", axum::routing::get(download_file))
        .with_state(file_storage);

    info!("MCP server listening on {} (endpoint: /mcp)", addr);
    info!("File download endpoint: /files/:id");

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
struct DocgenServer {
    /// Optional file storage for HTTP mode
    file_storage: Option<FileStorage>,
    /// Base URL for HTTP mode (for generating download links)
    base_url: Option<String>,
}

impl DocgenServer {
    fn new(file_storage: Option<FileStorage>, base_url: Option<String>) -> Self {
        Self {
            file_storage,
            base_url,
        }
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

        // Create tool context based on transport mode
        let tool_context = if let (Some(storage), Some(base_url)) = (&self.file_storage, &self.base_url) {
            tools::ToolContext::http(storage.clone(), base_url.clone())
        } else {
            tools::ToolContext::stdio()
        };

        match tools::call_tool(&request.name, arguments, &tool_context).await {
            Ok(result) => Ok(CallToolResult::structured(result)),
            Err(e) => Ok(CallToolResult::structured_error(serde_json::json!({
                "error": e
            }))),
        }
    }
}
