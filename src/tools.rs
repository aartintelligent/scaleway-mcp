use crate::client::ScalewayClient;
use crate::endpoint::api_instance_image::*;
use crate::endpoint::api_instance_security_group::*;
use crate::endpoint::api_instance_server::*;
use rmcp::model::{CallToolResult, Content, ServerCapabilities, ServerInfo};
use rmcp::{Error, ServerHandler, tool};

#[derive(Debug, Clone)]
pub struct ScalewayTools {
    client: ScalewayClient,
}

impl ScalewayTools {
    pub fn new(client: ScalewayClient) -> Self {
        Self { client }
    }
}

#[tool(tool_box)]
impl ScalewayTools {
    #[tool(description = "List instance servers")]
    async fn list_instance_servers(
        &self,
        #[tool(aggr)] request: ListInstanceServersRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }

    #[tool(description = "Create instance server")]
    async fn create_instance_server(
        &self,
        #[tool(aggr)] request: CreateInstanceServerRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }

    #[tool(description = "Get instance server")]
    async fn get_instance_server(
        &self,
        #[tool(aggr)] request: GetInstanceServerRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }

    #[tool(description = "Update instance server")]
    async fn update_instance_server(
        &self,
        #[tool(aggr)] request: UpdateInstanceServerRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }

    #[tool(description = "Delete instance server")]
    async fn delete_instance_server(
        &self,
        #[tool(aggr)] request: DeleteInstanceServerRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }

    #[tool(description = "List instance server actions")]
    async fn list_instance_server_actions(
        &self,
        #[tool(aggr)] request: ListInstanceServerActionsRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }

    #[tool(description = "Perform instance server action")]
    async fn perform_instance_server_action(
        &self,
        #[tool(aggr)] request: PerformInstanceServerActionRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }

    #[tool(description = "Attache volume instance server")]
    async fn attach_volume_instance_server(
        &self,
        #[tool(aggr)] request: AttachVolumeInstanceServerRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }

    #[tool(description = "Get instance server compatibility type")]
    async fn get_instance_server_compatibility_type(
        &self,
        #[tool(aggr)] request: GetInstanceServerCompatibilityTypeRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }

    #[tool(description = "List instance images")]
    async fn list_instance_images(
        &self,
        #[tool(aggr)] request: ListInstanceImagesRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }

    #[tool(description = "Create instance image")]
    async fn create_instance_image(
        &self,
        #[tool(aggr)] request: CreateInstanceImageRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }

    #[tool(description = "Update instance image")]
    async fn update_instance_image(
        &self,
        #[tool(aggr)] request: UpdateInstanceImageRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }

    #[tool(description = "Get instance image")]
    async fn get_instance_image(
        &self,
        #[tool(aggr)] request: GetInstanceImageRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }

    #[tool(description = "Delete instance image")]
    async fn delete_instance_image(
        &self,
        #[tool(aggr)] request: DeleteInstanceImageRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }

    #[tool(description = "List instance security groups")]
    async fn list_instance_secutiry_groups(
        &self,
        #[tool(aggr)] request: ListInstanceSecurityGroupsRequest,
    ) -> Result<CallToolResult, Error> {
        let response = self.client.execute(request).await;
        match response {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(e.into()),
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for ScalewayTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Scaleway mcp".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
