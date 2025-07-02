use crate::endpoint::Endpoint;
use crate::models::*;
use reqwest::Method;
use rmcp::schemars;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ListInstanceImagesRequest {
    pub zone: String,
    pub name: Option<String>,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub tags: Option<String>,
    pub arch: Option<String>,
    pub public: Option<bool>,
    pub per_page: Option<u32>,
    pub page: Option<u32>,
}

impl Endpoint for ListInstanceImagesRequest {
    type Response = ListInstanceImagesResponse;

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
        if let Some(arch) = &self.arch {
            params.push(("arch".into(), arch.clone()));
        }
        if let Some(public) = self.public {
            params.push(("public".into(), public.to_string()));
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
pub struct ListInstanceImagesResponse {
    pub images: Vec<InstanceImage>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct CreateInstanceImageRequest {
    #[serde(skip_serializing)]
    pub zone: String,
    pub name: String,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub tags: Option<Vec<String>>,
    pub arch: InstanceImageArchType,
    pub extra_volumes: Option<HashMap<String, InstanceImageExtraVolume>>,
    pub root_volume: String,
    pub public: Option<bool>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
}

impl Endpoint for CreateInstanceImageRequest {
    type Response = CreateInstanceImageResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("/instance/v1/zones/{}/images", self.zone)
    }

    fn body(&self) -> Option<serde_json::Value> {
        Some(json!(self))
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct CreateInstanceImageResponse {
    pub image: InstanceImage,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct UpdateInstanceImageRequest {
    #[serde(skip_serializing)]
    pub zone: String,
    #[serde(skip_serializing)]
    pub image_id: String,
    pub name: String,
    pub arch: Option<InstanceImageArchType>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
    pub extra_volumes: Option<HashMap<String, InstanceImageExtraVolume>>,
    pub from_server: Option<String>,
    pub organization: Option<String>,
    pub public: Option<bool>,
    pub root_volume: Option<HashMap<String, InstanceVolume>>,
    pub state: Option<InstanceImageStateType>,
    pub project: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Endpoint for UpdateInstanceImageRequest {
    type Response = UpdateInstanceImageResponse;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!("/instance/v1/zones/{}/images/{}", self.zone, self.image_id)
    }

    fn body(&self) -> Option<serde_json::Value> {
        Some(json!(self))
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct UpdateInstanceImageResponse {
    pub image: InstanceImage,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct GetInstanceImageRequest {
    pub zone: String,
    pub image_id: String,
}

impl Endpoint for GetInstanceImageRequest {
    type Response = GetInstanceImageResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("/instance/v1/zones/{}/images/{}", self.zone, self.image_id)
    }

    fn body(&self) -> Option<serde_json::Value> {
        Some(json!(self))
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct GetInstanceImageResponse {
    pub image: InstanceImage,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct DeleteInstanceImageRequest {
    pub zone: String,
    pub image_id: String,
}

impl Endpoint for DeleteInstanceImageRequest {
    type Response = ();

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!("/instance/v1/zones/{}/images/{}", self.zone, self.image_id)
    }
}
