#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EhNamespaceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EhNamespace>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EhNamespace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<eh_namespace::Properties>,
}
pub mod eh_namespace {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "provisioningState", skip_serializing)]
        pub provisioning_state: Option<String>,
        #[serde(rename = "createdAt", skip_serializing)]
        pub created_at: Option<String>,
        #[serde(rename = "updatedAt", skip_serializing)]
        pub updated_at: Option<String>,
        #[serde(rename = "serviceBusEndpoint", skip_serializing)]
        pub service_bus_endpoint: Option<String>,
        #[serde(rename = "clusterArmId", default, skip_serializing_if = "Option::is_none")]
        pub cluster_arm_id: Option<String>,
        #[serde(rename = "metricId", skip_serializing)]
        pub metric_id: Option<String>,
        #[serde(rename = "isAutoInflateEnabled", default, skip_serializing_if = "Option::is_none")]
        pub is_auto_inflate_enabled: Option<bool>,
        #[serde(rename = "maximumThroughputUnits", default, skip_serializing_if = "Option::is_none")]
        pub maximum_throughput_units: Option<i32>,
        #[serde(rename = "kafkaEnabled", default, skip_serializing_if = "Option::is_none")]
        pub kafka_enabled: Option<bool>,
        #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
        pub zone_redundant: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encryption: Option<Encryption>,
        #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
        pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Basic,
        Standard,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Basic,
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Encryption {
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub key_vault_properties: Vec<KeyVaultProperties>,
    #[serde(rename = "keySource", default, skip_serializing_if = "Option::is_none")]
    pub key_source: Option<encryption::KeySource>,
    #[serde(rename = "requireInfrastructureEncryption", default, skip_serializing_if = "Option::is_none")]
    pub require_infrastructure_encryption: Option<bool>,
}
pub mod encryption {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeySource {
        #[serde(rename = "Microsoft.KeyVault")]
        MicrosoftKeyVault,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultProperties {
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "keyVaultUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
    #[serde(rename = "keyVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<UserAssignedIdentityProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<ConnectionState>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_endpoint_connection_properties::ProvisioningState>,
}
pub mod private_endpoint_connection_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<connection_state::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
pub mod connection_state {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        Approved,
        Rejected,
        Disconnected,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourcesListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAssignedIdentityProperties {
    #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing)]
        pub provider: Option<String>,
        #[serde(skip_serializing)]
        pub resource: Option<String>,
        #[serde(skip_serializing)]
        pub operation: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaptureDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<capture_description::Encoding>,
    #[serde(rename = "intervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i32>,
    #[serde(rename = "sizeLimitInBytes", default, skip_serializing_if = "Option::is_none")]
    pub size_limit_in_bytes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[serde(rename = "skipEmptyArchives", default, skip_serializing_if = "Option::is_none")]
    pub skip_empty_archives: Option<bool>,
}
pub mod capture_description {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Encoding {
        Avro,
        AvroDeflate,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Eventhub {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<eventhub::Properties>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
pub mod eventhub {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "partitionIds", skip_serializing)]
        pub partition_ids: Vec<String>,
        #[serde(rename = "createdAt", skip_serializing)]
        pub created_at: Option<String>,
        #[serde(rename = "updatedAt", skip_serializing)]
        pub updated_at: Option<String>,
        #[serde(rename = "messageRetentionInDays", default, skip_serializing_if = "Option::is_none")]
        pub message_retention_in_days: Option<i64>,
        #[serde(rename = "partitionCount", default, skip_serializing_if = "Option::is_none")]
        pub partition_count: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[serde(rename = "captureDescription", default, skip_serializing_if = "Option::is_none")]
        pub capture_description: Option<CaptureDescription>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            Active,
            Disabled,
            Restoring,
            SendDisabled,
            ReceiveDisabled,
            Creating,
            Deleting,
            Renaming,
            Unknown,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Eventhub>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Destination {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<destination::Properties>,
}
pub mod destination {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
        pub storage_account_resource_id: Option<String>,
        #[serde(rename = "blobContainer", default, skip_serializing_if = "Option::is_none")]
        pub blob_container: Option<String>,
        #[serde(rename = "archiveNameFormat", default, skip_serializing_if = "Option::is_none")]
        pub archive_name_format: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UnavailableReason {
    None,
    InvalidName,
    SubscriptionIsDisabled,
    NameInUse,
    NameInLockdown,
    TooManyNamespaceInCurrentSubscription,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameter {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityResult {
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<UnavailableReason>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmDisasterRecovery {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<arm_disaster_recovery::Properties>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
pub mod arm_disaster_recovery {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "provisioningState", skip_serializing)]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[serde(rename = "partnerNamespace", default, skip_serializing_if = "Option::is_none")]
        pub partner_namespace: Option<String>,
        #[serde(rename = "alternateName", default, skip_serializing_if = "Option::is_none")]
        pub alternate_name: Option<String>,
        #[serde(skip_serializing)]
        pub role: Option<properties::Role>,
        #[serde(rename = "pendingReplicationOperationsCount", skip_serializing)]
        pub pending_replication_operations_count: Option<i64>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Accepted,
            Succeeded,
            Failed,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Role {
            Primary,
            PrimaryNotReplicating,
            Secondary,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmDisasterRecoveryListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ArmDisasterRecovery>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subnet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NwRuleSetIpRules {
    #[serde(rename = "ipMask", default, skip_serializing_if = "Option::is_none")]
    pub ip_mask: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<nw_rule_set_ip_rules::Action>,
}
pub mod nw_rule_set_ip_rules {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Allow,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NwRuleSetVirtualNetworkRules {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<Subnet>,
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRuleSet {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<network_rule_set::Properties>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
pub mod network_rule_set {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "trustedServiceAccessEnabled", default, skip_serializing_if = "Option::is_none")]
        pub trusted_service_access_enabled: Option<bool>,
        #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
        pub default_action: Option<properties::DefaultAction>,
        #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
        pub virtual_network_rules: Vec<NwRuleSetVirtualNetworkRules>,
        #[serde(rename = "ipRules", default, skip_serializing_if = "Vec::is_empty")]
        pub ip_rules: Vec<NwRuleSetIpRules>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum DefaultAction {
            Allow,
            Deny,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationRuleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AuthorizationRule>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationRule {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<authorization_rule::Properties>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
pub mod authorization_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        pub rights: Vec<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessKeys {
    #[serde(rename = "primaryConnectionString", skip_serializing)]
    pub primary_connection_string: Option<String>,
    #[serde(rename = "secondaryConnectionString", skip_serializing)]
    pub secondary_connection_string: Option<String>,
    #[serde(rename = "aliasPrimaryConnectionString", skip_serializing)]
    pub alias_primary_connection_string: Option<String>,
    #[serde(rename = "aliasSecondaryConnectionString", skip_serializing)]
    pub alias_secondary_connection_string: Option<String>,
    #[serde(rename = "primaryKey", skip_serializing)]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", skip_serializing)]
    pub secondary_key: Option<String>,
    #[serde(rename = "keyName", skip_serializing)]
    pub key_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateAccessKeyParameters {
    #[serde(rename = "keyType")]
    pub key_type: regenerate_access_key_parameters::KeyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
pub mod regenerate_access_key_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        PrimaryKey,
        SecondaryKey,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsumerGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<consumer_group::Properties>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
pub mod consumer_group {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "createdAt", skip_serializing)]
        pub created_at: Option<String>,
        #[serde(rename = "updatedAt", skip_serializing)]
        pub updated_at: Option<String>,
        #[serde(rename = "userMetadata", default, skip_serializing_if = "Option::is_none")]
        pub user_metadata: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsumerGroupListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConsumerGroup>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}