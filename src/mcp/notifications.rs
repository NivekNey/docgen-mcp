//! MCP list change notification helpers
//!
//! This module provides utilities for sending MCP list change notifications to clients.
//! These notifications inform clients when the available tools, resources, or prompts
//! have changed, allowing them to re-fetch the updated lists.
//!
//! ## Overview
//!
//! The Model Context Protocol (MCP) supports notifications that inform clients when
//! server capabilities change. This is particularly useful for:
//!
//! - **Dynamic tool registration**: When tools are added/removed at runtime
//! - **Resource updates**: When new resources become available or existing ones are removed
//! - **Prompt changes**: When prompt templates are modified or new ones added
//!
//! ## Capabilities
//!
//! This server declares support for list change notifications via the `listChanged` capability
//! for tools, resources, and prompts. See [`crate::main::DocgenServer::get_info`] for the
//! capability declaration.
//!
//! ## Notification Types
//!
//! MCP defines three notification types:
//!
//! - `notifications/tools/list_changed` - Sent when tools are added, removed, or modified
//! - `notifications/resources/list_changed` - Sent when resources are added, removed, or modified
//! - `notifications/prompts/list_changed` - Sent when prompts are added, removed, or modified
//!
//! ## Usage
//!
//! To send notifications, you need access to the peer context from request handlers.
//! The context provides a `peer` field that allows sending notifications.
//!
//! ### Example: Sending a tool list changed notification
//!
//! ```rust,ignore
//! use rmcp::model::{ServerNotification, ToolListChangedNotification};
//! use rmcp::service::RequestContext;
//!
//! async fn some_handler(
//!     context: RequestContext<rmcp::RoleServer>,
//! ) -> Result<(), rmcp::ErrorData> {
//!     // ... make changes to tools ...
//!
//!     // Notify clients that the tool list has changed
//!     context.peer
//!         .send_notification(ServerNotification::from(ToolListChangedNotification::default()))
//!         .await
//!         .map_err(|e| rmcp::ErrorData::internal_error(format!("Failed to send notification: {}", e), None))?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Example: Sending all notification types
//!
//! ```rust,ignore
//! use rmcp::model::{ServerNotification, ToolListChangedNotification, ResourceListChangedNotification, PromptListChangedNotification};
//! use rmcp::service::RequestContext;
//!
//! async fn notify_all_lists_changed(
//!     context: RequestContext<rmcp::RoleServer>,
//! ) -> Result<(), Box<dyn std::error::Error>> {
//!     // Notify about tools
//!     context.peer
//!         .send_notification(ServerNotification::from(ToolListChangedNotification::default()))
//!         .await?;
//!
//!     // Notify about resources
//!     context.peer
//!         .send_notification(ServerNotification::from(ResourceListChangedNotification::default()))
//!         .await?;
//!
//!     // Notify about prompts
//!     context.peer
//!         .send_notification(ServerNotification::from(PromptListChangedNotification::default()))
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## When to Send Notifications
//!
//! Send notifications in these scenarios:
//!
//! 1. **After dynamic tool registration**: If your server adds new tools at runtime
//! 2. **After resource updates**: When new documents or schemas become available
//! 3. **After configuration changes**: When server capabilities are modified
//! 4. **After hot-reload**: If your server supports reloading configurations without restart
//!
//! ## Client Behavior
//!
//! When clients receive a list change notification, they should:
//!
//! 1. Re-query the server using the appropriate list endpoint:
//!    - `tools/list` for tool changes
//!    - `resources/list` for resource changes
//!    - `prompts/list` for prompt changes
//! 2. Update their internal cache with the new list
//! 3. Optionally notify the user about new capabilities
//!
//! ## Implementation Notes
//!
//! Currently, this server has static tools, resources, and prompts that don't change
//! at runtime. However, the notification infrastructure is in place for future enhancements:
//!
//! - **Future: Hot-reload support** - Reload document templates without restart
//! - **Future: Plugin system** - Dynamically load document type plugins
//! - **Future: User-defined templates** - Allow runtime template registration
//!
//! ## MCP Specification
//!
//! For more details, see the official MCP specification:
//! - [Tools specification](https://modelcontextprotocol.io/specification/2024-11-05/server/tools)
//! - [Resources specification](https://modelcontextprotocol.io/specification/2024-11-05/server/resources)
//! - [Prompts specification](https://modelcontextprotocol.io/specification/2024-11-05/server/prompts)

use rmcp::model::{
    PromptListChangedNotification, ResourceListChangedNotification, ServerNotification,
    ToolListChangedNotification,
};
use rmcp::service::{Peer, RoleServer};

/// Sends a notification that the tool list has changed
///
/// This informs clients that they should re-fetch the tool list using `tools/list`.
///
/// # Arguments
///
/// * `peer` - The peer connection to send the notification to
///
/// # Returns
///
/// Returns `Ok(())` if the notification was sent successfully, or an error otherwise.
///
/// # Example
///
/// ```rust,ignore
/// use rmcp::service::RequestContext;
/// use crate::mcp::notifications::notify_tools_changed;
///
/// async fn add_new_tool(context: RequestContext<rmcp::RoleServer>) -> Result<(), rmcp::ErrorData> {
///     // ... add tool logic ...
///
///     notify_tools_changed(&context.peer).await
///         .map_err(|e| rmcp::ErrorData::internal_error(format!("Failed to notify: {}", e), None))?;
///
///     Ok(())
/// }
/// ```
pub async fn notify_tools_changed(peer: &Peer<RoleServer>) -> Result<(), rmcp::service::ServiceError> {
    peer.send_notification(ServerNotification::from(
        ToolListChangedNotification::default(),
    ))
    .await
}

/// Sends a notification that the resource list has changed
///
/// This informs clients that they should re-fetch the resource list using `resources/list`.
///
/// # Arguments
///
/// * `peer` - The peer connection to send the notification to
///
/// # Returns
///
/// Returns `Ok(())` if the notification was sent successfully, or an error otherwise.
///
/// # Example
///
/// ```rust,ignore
/// use rmcp::service::RequestContext;
/// use crate::mcp::notifications::notify_resources_changed;
///
/// async fn add_new_schema(context: RequestContext<rmcp::RoleServer>) -> Result<(), rmcp::ErrorData> {
///     // ... add schema logic ...
///
///     notify_resources_changed(&context.peer).await
///         .map_err(|e| rmcp::ErrorData::internal_error(format!("Failed to notify: {}", e), None))?;
///
///     Ok(())
/// }
/// ```
pub async fn notify_resources_changed(
    peer: &Peer<RoleServer>,
) -> Result<(), rmcp::service::ServiceError> {
    peer.send_notification(ServerNotification::from(
        ResourceListChangedNotification::default(),
    ))
    .await
}

/// Sends a notification that the prompt list has changed
///
/// This informs clients that they should re-fetch the prompt list using `prompts/list`.
///
/// # Arguments
///
/// * `peer` - The peer connection to send the notification to
///
/// # Returns
///
/// Returns `Ok(())` if the notification was sent successfully, or an error otherwise.
///
/// # Example
///
/// ```rust,ignore
/// use rmcp::service::RequestContext;
/// use crate::mcp::notifications::notify_prompts_changed;
///
/// async fn update_best_practices(context: RequestContext<rmcp::RoleServer>) -> Result<(), rmcp::ErrorData> {
///     // ... update prompt logic ...
///
///     notify_prompts_changed(&context.peer).await
///         .map_err(|e| rmcp::ErrorData::internal_error(format!("Failed to notify: {}", e), None))?;
///
///     Ok(())
/// }
/// ```
pub async fn notify_prompts_changed(
    peer: &Peer<RoleServer>,
) -> Result<(), rmcp::service::ServiceError> {
    peer.send_notification(ServerNotification::from(
        PromptListChangedNotification::default(),
    ))
    .await
}

/// Sends notifications for all list types (tools, resources, and prompts)
///
/// This is a convenience function that sends all three notification types.
/// Use this when making changes that affect multiple list types.
///
/// # Arguments
///
/// * `peer` - The peer connection to send the notifications to
///
/// # Returns
///
/// Returns `Ok(())` if all notifications were sent successfully, or an error otherwise.
/// If any notification fails, the function returns immediately with the error.
///
/// # Example
///
/// ```rust,ignore
/// use rmcp::service::RequestContext;
/// use crate::mcp::notifications::notify_all_lists_changed;
///
/// async fn reload_configuration(context: RequestContext<rmcp::RoleServer>) -> Result<(), rmcp::ErrorData> {
///     // ... reload config, update tools, resources, and prompts ...
///
///     notify_all_lists_changed(&context.peer).await
///         .map_err(|e| rmcp::ErrorData::internal_error(format!("Failed to notify: {}", e), None))?;
///
///     Ok(())
/// }
/// ```
pub async fn notify_all_lists_changed(
    peer: &Peer<RoleServer>,
) -> Result<(), rmcp::service::ServiceError> {
    notify_tools_changed(peer).await?;
    notify_resources_changed(peer).await?;
    notify_prompts_changed(peer).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These functions require a live peer connection to test properly.
    // In a real test environment, you would set up a mock MCP server/client pair.
    // For now, we just verify the module compiles and exports the correct functions.

    #[test]
    fn test_module_exports() {
        // This test ensures the module compiles and the functions are accessible
        // Actual testing would require integration tests with a running MCP connection
    }
}
