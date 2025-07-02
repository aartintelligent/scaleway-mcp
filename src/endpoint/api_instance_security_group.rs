use crate::endpoint::Endpoint;
use crate::models::*;
use reqwest::Method;
use rmcp::schemars;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ListInstanceSecurityGroupsRequest {
    pub zone: String,
    pub name: Option<String>,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub tags: Option<String>,
    pub project_default: Option<bool>,
    pub per_page: Option<u32>,
    pub page: Option<u32>,
}

impl Endpoint for ListInstanceSecurityGroupsRequest {
    type Response = ListInstanceSecurityGroupsResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("/instance/v1/zones/{}/images", self.zone)
    }

    fn query(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(name) = &self.name {
            params.push(("name".into(), name.clone()));
        }
        if let Some(org) = &self.organization {
            params.push(("organization".into(), org.clone()));
        }
        if let Some(project) = &self.project {
            params.push(("project".into(), project.clone()));
        }
        if let Some(tags) = &self.tags {
            params.push(("tags".into(), tags.clone()));
        }
        if let Some(public) = self.project_default {
            params.push(("project_default".into(), public.to_string()));
        }
        if let Some(per_page) = self.per_page {
            params.push(("per_page".into(), per_page.to_string()));
        }
        if let Some(page) = self.page {
            params.push(("page".into(), page.to_string()));
        }

        params
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ListInstanceSecurityGroupsResponse {
    pub security_groups: Vec<InstanceSecurityGroup>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct CreateInstanceSecurityGroupRequest {
    #[serde(skip_serializing)]
    pub zone: String,
    pub name: String,
    pub project: Option<String>,
    pub tags: Option<Vec<String>>,
    pub project_default: Option<bool>,
    pub stateful: Option<bool>,
    pub inbound_default_policy: InstanceSecurityGroupPolicyType,
    pub outbound_default_policy: InstanceSecurityGroupPolicyType,
    pub enable_default_security: Option<bool>,
}

impl Endpoint for CreateInstanceSecurityGroupRequest {
    type Response = CreateInstanceSecurityGroupResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("/instance/v1/zones/{}/security_groups", self.zone)
    }

    fn body(&self) -> Option<serde_json::Value> {
        Some(json!(self))
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct CreateInstanceSecurityGroupResponse {
    pub security_group: InstanceSecurityGroup,
}
