use rspc::Type;
use serde::{Deserialize, Serialize};

use crate::util::{jar::RequestJar, Error};

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupConfigMetadata {
    pub group_configuration: GroupConfigurationMetadata,
    pub recurring_payouts_configuration: GroupRecurringPayoutsConfigurationMetadata,
    pub role_configuration: GroupRoleConfigurationMetadata,
    pub group_name_change_configuration: GroupNameChangeConfigurationMetadata,
    pub is_premium_payouts_enabled: bool,
    pub is_default_emblem_policy_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupConfigurationMetadata {
    pub name_max_length: i64,
    pub description_max_length: i64,
    pub icon_max_file_size_mb: i64,
    pub cost: i64,
    pub is_using_two_step_webview_component: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupRecurringPayoutsConfigurationMetadata {
    pub max_payout_partners: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupRoleConfigurationMetadata {
    pub name_max_length: i64,
    pub description_max_length: i64,
    pub limit: i64,
    pub cost: i64,
    pub min_rank: i64,
    pub max_rank: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupNameChangeConfigurationMetadata {
    pub cost: i64,
    pub cooldown_in_days: i64,
    pub ownership_cooldown_in_days: i64,
}

/// Gets the metadata for group configuration
pub async fn config_metadata(jar: &RequestJar) -> Result<GroupConfigMetadata, Box<Error>> {
    let url = "https://groups.roblox.com/v1/groups/configuration/metadata";
    let response = jar.get_json::<GroupConfigMetadata>(&url).await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupMetadata {
    pub group_limit: i64,
    pub current_group_count: i64,
    pub group_status_max_length: i64,
    pub group_post_max_length: i64,
    pub is_group_wall_notifications_enabled: bool,
    pub group_wall_notifications_subscribe_interval_in_milliseconds: i64,
    pub are_profile_groups_hidden: bool,
    pub is_group_details_policy_enabled: bool,
    pub show_previous_group_names: bool,
}

/// Gets the metadata for groups
pub async fn metadata(jar: &RequestJar) -> Result<GroupMetadata, Box<Error>> {
    let url = "https://groups.roblox.com/v1/groups/metadata";
    let response = jar.get_json::<GroupMetadata>(&url).await?;
    Ok(response)
}
