//! Example: Using MCP List Change Notifications
//!
//! This example demonstrates how to use MCP list change notifications
//! to inform clients when server capabilities change dynamically.
//!
//! ## Scenario
//!
//! This example shows a hypothetical scenario where:
//! 1. The server starts with a basic set of tools
//! 2. A new document type (cover letter) is added at runtime
//! 3. The server notifies clients about the new tools and resources
//! 4. Clients re-fetch the lists to discover new capabilities
//!
//! ## Running this example
//!
//! This is a documentation example showing the pattern. To adapt this
//! for your use case:
//!
//! 1. Make your ServerHandler stateful (store tools/resources/prompts)
//! 2. Add methods to modify the lists dynamically
//! 3. Send notifications when lists change
//! 4. Store peer references to send notifications to connected clients
//!
//! ## Note
//!
//! The current docgen-mcp implementation has static lists, so this example
//! is primarily educational. However, you could extend the server to support:
//! - Hot-reloading document templates
//! - Dynamic plugin loading
//! - User-defined custom document types

use std::sync::Arc;
use tokio::sync::RwLock;
use rmcp::{ServerHandler, ErrorData, model::*, service::*};
use rmcp::model::{RawResource, AnnotateAble};

/// A stateful server that can dynamically add tools and notify clients
pub struct DynamicDocgenServer {
    /// Current list of tools (stored in shared state)
    tools: Arc<RwLock<Vec<Tool>>>,
    /// Current list of resources (stored in shared state)
    resources: Arc<RwLock<Vec<Resource>>>,
    /// List of connected peer contexts for sending notifications
    peers: Arc<RwLock<Vec<Peer<RoleServer>>>>,
}

impl DynamicDocgenServer {
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(Vec::new())),
            resources: Arc::new(RwLock::new(Vec::new())),
            peers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add a new tool and notify all connected clients
    pub async fn add_tool(&self, tool: Tool) -> Result<(), Box<dyn std::error::Error>> {
        // Add the tool to our list
        {
            let mut tools = self.tools.write().await;
            tools.push(tool);
        }

        // Notify all connected clients
        self.notify_all_clients_tools_changed().await?;

        Ok(())
    }

    /// Add a new resource and notify all connected clients
    pub async fn add_resource(&self, resource: Resource) -> Result<(), Box<dyn std::error::Error>> {
        // Add the resource to our list
        {
            let mut resources = self.resources.write().await;
            resources.push(resource);
        }

        // Notify all connected clients
        self.notify_all_clients_resources_changed().await?;

        Ok(())
    }

    /// Send tool list changed notification to all connected clients
    async fn notify_all_clients_tools_changed(&self) -> Result<(), ServiceError> {
        let peers = self.peers.read().await;
        for peer in peers.iter() {
            peer.send_notification(ServerNotification::from(
                ToolListChangedNotification::default(),
            ))
            .await?;
        }
        Ok(())
    }

    /// Send resource list changed notification to all connected clients
    async fn notify_all_clients_resources_changed(&self) -> Result<(), ServiceError> {
        let peers = self.peers.read().await;
        for peer in peers.iter() {
            peer.send_notification(ServerNotification::from(
                ResourceListChangedNotification::default(),
            ))
            .await?;
        }
        Ok(())
    }

    /// Register a peer when a client connects
    async fn register_peer(&self, peer: Peer<RoleServer>) {
        let mut peers = self.peers.write().await;
        peers.push(peer);
    }
}

impl ServerHandler for DynamicDocgenServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_tool_list_changed()  // Enable notifications
                .enable_resources()
                .enable_resources_list_changed()  // Enable notifications
                .build(),
            server_info: Implementation {
                name: "dynamic-docgen-mcp".to_string(),
                version: "0.1.0".to_string(),
                title: Some("Dynamic Document Generation MCP Server".to_string()),
                website_url: None,
                icons: None,
            },
            instructions: Some(
                "A dynamic MCP server that supports runtime capability updates. \
                 Listen for list_changed notifications to discover new tools and resources."
                    .to_string(),
            ),
        }
    }

    async fn on_initialized(&self, context: NotificationContext<RoleServer>) {
        // Register the peer when a client connects
        self.register_peer(context.peer.clone()).await;
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let tools = self.tools.read().await;
        Ok(ListToolsResult {
            tools: tools.clone(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        let resources = self.resources.read().await;
        Ok(ListResourcesResult {
            resources: resources.clone(),
            next_cursor: None,
            meta: None,
        })
    }
}

/// Example: Adding a new document type at runtime
///
/// This shows how you might extend the server with a new document type
/// and notify clients about the new capabilities.
pub async fn example_add_cover_letter_support(
    server: &DynamicDocgenServer,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new tool for generating cover letters
    let generate_cover_letter_tool = Tool::new(
        "generate_cover_letter",
        "Generates a professionally formatted PDF cover letter from JSON payload",
        Arc::new(serde_json::Map::new()), // Schema would go here
    );

    // Add the tool (this will notify clients)
    server.add_tool(generate_cover_letter_tool).await?;

    // Create a new resource for the cover letter schema
    let mut raw_resource = RawResource::new(
        "docgen://schemas/cover-letter",
        "Cover Letter Schema",
    );
    raw_resource.description = Some("JSON Schema for cover letter documents".to_string());
    raw_resource.mime_type = Some("application/schema+json".to_string());
    let cover_letter_schema = raw_resource.no_annotation();

    // Add the resource (this will notify clients)
    server.add_resource(cover_letter_schema).await?;

    println!("✓ Cover letter support added!");
    println!("✓ Clients have been notified to re-fetch tool and resource lists");

    Ok(())
}

/// Example: Simulating client behavior
///
/// This shows what a client should do when it receives a list_changed notification
pub async fn example_client_behavior() {
    println!("\n=== Client receives notification ===");
    println!("Received: notifications/tools/list_changed");
    println!("Action: Re-fetching tool list...");
    println!("Result: Discovered new tool 'generate_cover_letter'");
    println!();
    println!("Received: notifications/resources/list_changed");
    println!("Action: Re-fetching resource list...");
    println!("Result: Discovered new resource 'docgen://schemas/cover-letter'");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== MCP List Change Notifications Example ===\n");

    // Create a dynamic server
    let server = DynamicDocgenServer::new();

    println!("Server started with basic capabilities");
    println!("Waiting for clients to connect...\n");

    // Simulate adding new capabilities at runtime
    println!("=== Adding new document type: Cover Letter ===");
    example_add_cover_letter_support(&server).await?;

    // Show what the client would see
    example_client_behavior().await;

    println!("\n=== Key Takeaways ===");
    println!("1. Declare listChanged capability in ServerCapabilities");
    println!("2. Store peer references when clients connect (on_initialized)");
    println!("3. Send notifications when lists change");
    println!("4. Clients re-fetch lists when notified");

    Ok(())
}
