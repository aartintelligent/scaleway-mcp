use reqwest::Method;

pub trait Endpoint {
    type Response: serde::de::DeserializeOwned;

    fn method(&self) -> Method;

    fn path(&self) -> String;

    fn query(&self) -> Vec<(String, String)> {
        Vec::new()
    }

    fn body(&self) -> Option<serde_json::Value> {
        None
    }
}

pub mod api_instance_image;
pub mod api_instance_security_group;
pub mod api_instance_server;
