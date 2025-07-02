use crate::endpoint::Endpoint;
use crate::models::*;
use reqwest::Method;
use rmcp::schemars;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ListInstanceServersRequest {
    pub zone: String,
    pub name: Option<String>,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub state: Option<String>,
    pub servers: Option<String>,
    pub tags: Option<String>,
    pub with_ip: Option<String>,
    pub without_ip: Option<bool>,
    pub private_network: Option<String>,
    pub private_networks: Option<String>,
    pub private_nic_mac_address: Option<String>,
    pub commercial_type: Option<String>,
    pub order: Option<String>,
    pub per_page: Option<u32>,
    pub page: Option<u32>,
}

impl Endpoint for ListInstanceServersRequest {
    type Response = ListInstanceServersResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("/instance/v1/zones/{}/servers", self.zone)
    }

    fn query(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(name) = &self.name {
            params.push(("name".to_string(), name.clone()));
        }
        if let Some(org) = &self.organization {
            params.push(("organization".to_string(), org.clone()));
        }
        if let Some(proj) = &self.project {
            params.push(("project".to_string(), proj.clone()));
        }
        if let Some(state) = &self.state {
            params.push(("state".to_string(), state.clone()));
        }
        if let Some(servers) = &self.servers {
            params.push(("servers".to_string(), servers.clone()));
        }
        if let Some(tags) = &self.tags {
            params.push(("tags".to_string(), tags.clone()));
        }
        if let Some(with_ip) = &self.with_ip {
            params.push(("with_ip".to_string(), with_ip.clone()));
        }
        if let Some(without_ip) = self.without_ip {
            params.push(("without_ip".to_string(), without_ip.to_string()));
        }
        if let Some(private_network) = &self.private_network {
            params.push(("private_network".to_string(), private_network.clone()));
        }
        if let Some(private_networks) = &self.private_networks {
            params.push(("private_networks".to_string(), private_networks.clone()));
        }
        if let Some(private_nic_mac_address) = &self.private_nic_mac_address {
            params.push((
                "private_nic_mac_address".to_string(),
                private_nic_mac_address.clone(),
            ));
        }
        if let Some(commercial_type) = &self.commercial_type {
            params.push(("commercial_type".to_string(), commercial_type.clone()));
        }
        if let Some(order) = &self.order {
            params.push(("order".to_string(), order.clone()));
        }
        if let Some(per_page) = self.per_page {
            params.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(page) = self.page {
            params.push(("page".to_string(), page.to_string()));
        }

        params
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ListInstanceServersResponse {
    pub servers: Vec<InstanceServer>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct CreateInstanceServerRequest {
    #[serde(skip_serializing)]
    pub zone: String,
    pub name: String,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub tags: Option<Vec<String>>,
    pub image: String,
    pub volumes: Option<HashMap<String, InstanceVolume>>,
    pub boot_type: Option<String>,
    pub public_ip: Option<String>,
    pub public_ips: Option<Vec<String>>,
    pub dynamic_ip_required: Option<bool>,
    pub routed_ip_enabled: Option<bool>,
    pub enable_ipv6: Option<bool>,
    pub security_group: Option<String>,
    pub placement_group: Option<String>,
    pub admin_password_encryption_ssh_key_id: Option<String>,
    pub protected: Option<bool>,
    pub commercial_type: String,
}

impl Endpoint for CreateInstanceServerRequest {
    type Response = CreateInstanceServerResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("/instance/v1/zones/{}/servers", self.zone)
    }

    fn body(&self) -> Option<serde_json::Value> {
        Some(json!(self))
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct CreateInstanceServerResponse {
    pub server: Vec<InstanceServer>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct GetInstanceServerRequest {
    pub zone: String,
    pub server_id: String,
}

impl Endpoint for GetInstanceServerRequest {
    type Response = GetInstanceServerResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "/instance/v1/zones/{}/servers/{}",
            self.zone, self.server_id
        )
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct GetInstanceServerResponse {
    pub server: InstanceServer,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct UpdateInstanceServerRequest {
    #[serde(skip_serializing)]
    pub zone: String,
    #[serde(skip_serializing)]
    pub server_id: String,
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub volumes: Option<HashMap<String, InstanceVolume>>,
    pub boot_type: Option<String>,
    pub public_ips: Option<Vec<String>>,
    pub dynamic_ip_required: Option<bool>,
    pub routed_ip_enabled: Option<bool>,
    pub enable_ipv6: Option<bool>,
    pub private_nics: Option<Vec<InstanceServerPrivateNic>>,
    pub protected: Option<bool>,
    pub security_group: Option<String>,
    pub placement_group: Option<String>,
    pub admin_password_encryption_ssh_key_id: Option<String>,
    pub commercial_type: Option<String>,
}

impl Endpoint for UpdateInstanceServerRequest {
    type Response = InstanceServer;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "/instance/v1/zones/{}/servers/{}",
            self.zone, self.server_id
        )
    }

    fn body(&self) -> Option<serde_json::Value> {
        Some(json!(self))
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct UpdateInstanceServerResponse {
    pub server: InstanceServer,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct DeleteInstanceServerRequest {
    pub zone: String,
    pub server_id: String,
}

impl Endpoint for DeleteInstanceServerRequest {
    type Response = ();

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "/instance/v1/zones/{}/servers/{}",
            self.zone, self.server_id
        )
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ListInstanceServerActionsRequest {
    pub zone: String,
    pub server_id: String,
}

impl Endpoint for ListInstanceServerActionsRequest {
    type Response = ListInstanceServerActionsResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "/instance/v1/zones/{}/servers/{}/action",
            self.zone, self.server_id
        )
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ListInstanceServerActionsResponse {
    pub actions: Vec<InstanceServerAction>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct PerformInstanceServerActionRequest {
    #[serde(skip_serializing)]
    pub zone: String,
    #[serde(skip_serializing)]
    pub server_id: String,
    pub name: Option<String>,
    pub action: Option<InstanceServerActionType>,
    pub volumes: Option<HashMap<String, InstanceVolumeType>>,
    pub disable_ipv6: Option<bool>,
}

impl Endpoint for PerformInstanceServerActionRequest {
    type Response = PerformInstanceServerActionResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "/instance/v1/zones/{}/servers/{}/action",
            self.zone, self.server_id
        )
    }

    fn body(&self) -> Option<serde_json::Value> {
        Some(json!(self))
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct PerformInstanceServerActionResponse {
    pub task: InstanceServerActionTask,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct AttachVolumeInstanceServerRequest {
    #[serde(skip_serializing)]
    pub zone: String,
    #[serde(skip_serializing)]
    pub server_id: String,
    pub filesystem_id: String,
}

impl Endpoint for AttachVolumeInstanceServerRequest {
    type Response = AttachVolumeInstanceServerResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "/instance/v1/zones/{}/servers/{}/attach-volume",
            self.zone, self.server_id
        )
    }

    fn body(&self) -> Option<serde_json::Value> {
        Some(json!(self))
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct AttachVolumeInstanceServerResponse {
    pub server: InstanceServer,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct GetInstanceServerCompatibilityTypeRequest {
    pub zone: String,
    pub server_id: String,
}

impl Endpoint for GetInstanceServerCompatibilityTypeRequest {
    type Response = GetInstanceServerCompatibilityTypeResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "/instance/v1/zones/{}/servers/{}/compatible-types",
            self.zone, self.server_id
        )
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct GetInstanceServerCompatibilityTypeResponse {
    pub compatible_types: Vec<String>,
}
