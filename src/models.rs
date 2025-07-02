use rmcp::schemars;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceServer {
    pub id: String,
    pub name: String,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub allowed_actions: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub commercial_type: Option<String>,
    pub creation_date: Option<String>,
    pub dynamic_ip_required: Option<bool>,
    pub routed_ip_enabled: Option<bool>,
    pub enable_ipv6: Option<bool>,
    pub hostname: Option<String>,
    pub image: Option<InstanceImage>,
    pub protected: Option<bool>,
    pub private_ip: Option<String>,
    pub public_ip: Option<InstanceServerPublicIp>,
    pub public_ips: Option<Vec<InstanceServerPublicIp>>,
    pub mac_address: Option<String>,
    pub modification_date: Option<String>,
    pub state: Option<String>,
    pub location: Option<InstanceServerLocation>,
    pub ipv6: Option<InstanceServerIpv6>,
    pub boot_type: Option<String>,
    pub volumes: Option<HashMap<String, InstanceServerVolume>>,
    pub security_group: Option<InstanceServerSecurityGroup>,
    pub maintenances: Option<Vec<InstanceServerMaintenance>>,
    pub state_detail: Option<String>,
    pub arch: Option<String>,
    pub placement_group: Option<InstanceServerPlacementGroup>,
    pub private_nics: Option<Vec<InstanceServerPrivateNic>>,
    pub zone: Option<String>,
    pub admin_password_encryption_ssh_key_id: Option<String>,
    pub admin_password_encrypted_value: Option<String>,
    pub filesystems: Option<Vec<InstanceServerFilesystem>>,
    pub end_of_service: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceServerAction {
    pub name: InstanceServerActionType,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstanceServerActionType {
    Poweron,
    Backup,
    StopInPlace,
    Poweroff,
    Terminate,
    Reboot,
    EnableRoutedIp,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceServerActionTask {
    pub id: String,
    pub description: Option<String>,
    pub progress: Option<u32>,
    pub started_at: Option<String>,
    pub terminated_at: Option<String>,
    pub status: String,
    pub zone: Option<String>,
    pub href_from: Option<String>,
    pub href_result: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceImage {
    pub id: String,
    pub name: Option<String>,
    pub arch: Option<InstanceImageArchType>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
    pub default_bootscript: Option<InstanceImageBootScript>,
    pub extra_volumes: Option<HashMap<String, InstanceImageExtraVolume>>,
    pub from_server: Option<String>,
    pub organization: Option<String>,
    pub public: Option<bool>,
    pub root_volume: Option<InstanceImageRootVolume>,
    pub state: Option<InstanceImageStateType>,
    pub project: Option<String>,
    pub tags: Option<Vec<String>>,
    pub zone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstanceImageArchType {
    UnknownArch,
    #[serde(rename = "x86_64")]
    X8664,
    Arm,
    Arm64,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstanceImageStateType {
    Available,
    Creating,
    Error,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceImageBootScript {
    pub architecture: Option<String>,
    pub bootcmdargs: Option<String>,
    pub default: Option<bool>,
    pub dtb: Option<String>,
    pub id: String,
    pub initrd: Option<String>,
    pub kernel: Option<String>,
    pub organization: Option<String>,
    pub public: Option<bool>,
    pub title: Option<String>,
    pub project: Option<String>,
    pub zone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceImageExtraVolume {
    pub id: String,
    pub name: Option<String>,
    pub export_uri: Option<String>,
    pub size: Option<u64>,
    pub volume_type: Option<String>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub tags: Option<Vec<String>>,
    pub server: Option<InstanceImageExtraVolumeServer>,
    pub state: Option<InstanceImageExtraVolumeStateType>,
    pub zone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstanceImageExtraVolumeStateType {
    Available,
    Snapshotting,
    Fetching,
    Resizing,
    Saving,
    Hotsyncing,
    Error,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceImageExtraVolumeServer {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceImageRootVolume {
    pub id: String,
    pub name: Option<String>,
    pub size: Option<u64>,
    pub volume_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceServerPublicIp {
    pub id: Option<String>,
    pub address: Option<String>,
    pub gateway: Option<String>,
    pub netmask: Option<String>,
    pub family: Option<String>,
    pub dynamic: Option<bool>,
    pub provisioning_mode: Option<String>,
    pub tags: Option<Vec<String>>,
    pub ipam_id: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceServerLocation {
    pub cluster_id: Option<String>,
    pub hypervisor_id: Option<String>,
    pub node_id: Option<String>,
    pub platform_id: Option<String>,
    pub zone_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceServerIpv6 {
    pub address: Option<String>,
    pub gateway: Option<String>,
    pub netmask: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceServerVolume {
    pub id: String,
    pub name: Option<String>,
    pub export_uri: Option<String>,
    pub organization: Option<String>,
    pub server: Option<InstanceImageExtraVolumeServer>,
    pub size: Option<u64>,
    pub volume_type: Option<String>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
    pub state: Option<String>,
    pub project: Option<String>,
    pub boot: Option<bool>,
    pub zone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceServerSecurityGroup {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceServerMaintenance {
    pub reason: Option<String>,
    pub start_date: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceServerPlacementGroup {
    pub id: Option<String>,
    pub name: Option<String>,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub tags: Option<Vec<String>>,
    pub policy_mode: Option<String>,
    pub policy_type: Option<String>,
    pub policy_respected: Option<bool>,
    pub zone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceServerPrivateNic {
    pub id: Option<String>,
    pub server_id: Option<String>,
    pub private_network_id: Option<String>,
    pub mac_address: Option<String>,
    pub state: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceServerFilesystem {
    pub filesystem_id: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceSecurityGroup {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub enable_default_security: Option<bool>,
    pub inbound_default_policy: Option<InstanceSecurityGroupPolicyType>,
    pub outbound_default_policy: Option<InstanceSecurityGroupPolicyType>,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub tags: Option<Vec<String>>,
    pub organization_default: Option<bool>,
    pub project_default: Option<bool>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
    pub servers: Option<Vec<InstanceSecurityGroupServer>>,
    pub stateful: Option<bool>,
    pub state: Option<String>,
    pub zone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstanceSecurityGroupPolicyType {
    UnknownPolicy,
    Accept,
    Drop,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceSecurityGroupServer {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceSecurityGroupRule {
    pub id: String,
    pub protocol: Option<String>,
    pub direction: Option<String>,
    pub action: Option<String>,
    pub ip_range: Option<String>,
    pub dest_port_from: Option<u32>,
    pub dest_port_to: Option<u32>,
    pub position: Option<u32>,
    pub editable: Option<bool>,
    pub zone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceProductServer {
    pub monthly_price: Option<f64>,
    pub hourly_price: Option<f64>,
    pub alt_names: Option<Vec<String>>,
    pub per_volume_constraint: Option<InstanceProductServerPerVolumeConstraint>,
    pub volumes_constraint: Option<InstanceProductServerPerVolumeConstraint>,
    pub ncpus: Option<u32>,
    pub gpu: Option<u32>,
    pub ram: Option<u32>,
    pub gpu_info: Option<InstanceProductServerGpuInfo>,
    pub arch: Option<String>,
    pub network: Option<InstanceProductServerNetwork>,
    pub capabilities: Option<InstanceProductServerCapabilities>,
    pub max_additional_volumes: Option<u32>,
    pub max_volume_size: Option<u32>,
    pub block_bandwidth: Option<u32>,
    pub scratch_storage_max_size: Option<u32>,
    pub ipv6_support: Option<bool>,
    pub end_of_service: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceProductServerPerVolumeConstraint {
    pub min_size: Option<u32>,
    pub max_size: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceProductServerGpuInfo {
    pub gpu_manufacturer: Option<String>,
    pub gpu_name: Option<String>,
    pub gpu_memory: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceProductServerNetwork {
    pub interfaces: Option<Vec<InstanceProductServerNetworkInterface>>,
    pub sum_internal_bandwidth: Option<u32>,
    pub sum_internet_bandwidth: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceProductServerNetworkInterface {
    pub internal_bandwidth: Option<u32>,
    pub internet_bandwidth: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceProductServerCapabilities {
    pub block_storage: Option<bool>,
    pub boot_types: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceProductServerAvailability {
    pub availability: String,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceProductVolume {
    pub display_name: Option<String>,
    pub capabilities: Option<InstanceProductVolumeCapabilities>,
    pub constraints: Option<InstanceProductVolumeConstraints>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceProductVolumeCapabilities {
    pub snapshot: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceProductVolumeConstraints {
    pub min: Option<u32>,
    pub max: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceIp {
    pub id: String,
    pub address: String,
    pub reverse: Option<String>,
    pub server: Option<InstanceIpServer>,
    pub organization: Option<String>,
    pub tags: Option<Vec<String>>,
    pub project: Option<String>,
    pub r#type: Option<String>,
    pub state: Option<String>,
    pub prefix: Option<String>,
    pub ipam_id: Option<String>,
    pub zone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceIpServer {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceVolume {
    pub id: String,
    pub name: String,
    pub export_uri: Option<String>,
    pub size: u64,
    pub volume_type: InstanceVolumeType,
    pub creation_date: String,
    pub modification_date: String,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub tags: Option<Vec<String>>,
    pub server: Option<InstanceVolumeServer>,
    pub state: Option<String>,
    pub zone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstanceVolumeType {
    UnknownVolumeType,
    LSsd,
    BSsd,
    Unified,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceVolumeServer {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstanceSnapshot {
    pub id: String,
    pub name: String,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub tags: Option<Vec<String>>,
    pub volume_type: String,
    pub size: u64,
    pub state: String,
    pub base_volume: Option<SnapshotBaseVolume>,
    pub creation_date: String,
    pub modification_date: String,
    pub zone: String,
    pub error_reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct SnapshotBaseVolume {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstancePlacementGroup {
    pub id: String,
    pub name: String,
    pub organization: String,
    pub project: String,
    pub tags: Vec<String>,
    pub policy_mode: String,
    pub policy_type: String,
    pub policy_respected: bool,
    pub zone: String,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InstancePlacementGroupServer {
    pub id: String,
    pub name: String,
    pub policy_respected: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoscalingInstanceGroup {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub tags: Vec<String>,
    pub instance_template_id: String,
    pub capacity: AutoscalingInstanceGroupCapacity,
    pub loadbalancer: Option<AutoscalingInstanceGroupLoadBalancer>,
    pub private_network_id: Option<String>,
    pub error_messages: Option<Vec<String>>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoscalingInstanceGroupCapacity {
    pub max_replicas: u32,
    pub min_replicas: u32,
    pub cooldown_delay: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoscalingInstanceGroupLoadBalancer {
    pub id: String,
    pub backend_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoscalingInstanceEvent {
    pub id: String,
    pub source: String,
    pub level: String,
    pub name: String,
    pub created_at: String,
    pub details: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoscalingInstanceTemplate {
    pub id: String,
    pub commercial_type: String,
    pub image_id: String,
    pub volumes: serde_json::Value,
    pub tags: Vec<String>,
    pub security_group_id: String,
    pub placement_group_id: String,
    pub public_ips_v4_count: u32,
    pub public_ips_v6_count: u32,
    pub project_id: String,
    pub name: String,
    pub private_network_ids: Vec<String>,
    pub status: String,
    pub cloud_init: String,
    pub created_at: String,
    pub updated_at: String,
}
