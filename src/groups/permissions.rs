use rspc::Type;
use serde::{Deserialize, Serialize};

use crate::util::{jar::RequestJar, responses::DataWrapper, Error};

use super::GroupRole;

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupPermissions {
    pub group_posts_permissions: GroupPostPermissions,
    pub group_membership_permissions: GroupMembershipPermissions,
    pub group_management_permissions: GroupManagementPermissions,
    pub group_economy_permissions: GroupEconomyPermissions,
    pub group_open_cloud_permissions: GroupOpenCloudPermissions,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupPostPermissions {
    pub view_wall: bool,
    pub post_to_wall: bool,
    pub delete_from_wall: bool,
    pub view_status: bool,
    pub post_to_status: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembershipPermissions {
    pub change_rank: bool,
    pub invite_members: bool,
    pub remove_members: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupManagementPermissions {
    pub manage_relationships: bool,
    pub manage_clan: bool,
    pub view_audit_logs: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupEconomyPermissions {
    pub spend_group_funds: bool,
    pub advertise_group: bool,
    pub create_items: bool,
    pub manage_items: bool,
    pub add_group_places: bool,
    pub manage_group_games: bool,
    pub view_group_payouts: bool,
    pub view_analytics: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupOpenCloudPermissions {
    pub use_cloud_authentication: bool,
    pub administer_cloud_authentication: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct RolePermissions {
    pub group_id: i64,
    pub role: GroupRole,
    pub permissions: GroupPermissions,
}

/// Gets the permissions for a specific role in a group
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 2: The roleset is invalid or does not exist.
/// - 3: You are not authorized to view/edit permissions for this role.
pub async fn role_permissions(
    jar: &RequestJar,
    group_id: i64,
    role_id: i64,
) -> Result<RolePermissions, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/roles/{}/permissions",
        group_id, role_id
    );

    let response = jar.get_json(&url).await?;

    Ok(response)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRolePermissionsRequestPermissions {
    pub delete_from_wall: Option<bool>,
    pub post_to_wall: Option<bool>,
    pub invite_members: Option<bool>,
    pub post_to_status: Option<bool>,
    pub remove_members: Option<bool>,
    pub view_status: Option<bool>,
    pub view_wall: Option<bool>,
    pub change_rank: Option<bool>,
    pub advertise_group: Option<bool>,
    pub manage_relationships: Option<bool>,
    pub add_group_places: Option<bool>,
    pub view_audit_logs: Option<bool>,
    pub create_items: Option<bool>,
    pub manage_items: Option<bool>,
    pub spend_group_funds: Option<bool>,
    pub manage_clan: Option<bool>,
    pub manage_group_games: Option<bool>,
    pub use_cloud_authentication: Option<bool>,
    pub administer_cloud_authentication: Option<bool>,
    pub view_analytics: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRolePermissionsRequest {
    pub permissions: UpdateRolePermissionsRequestPermissions,
}

/// Sets the permissions for a specific role in a group
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 2: The roleset is invalid or does not exist.
/// - 3: You are not authorized to view/edit permissions for this role.
/// - 4: This role's permissions can not be modified.
pub async fn update_role_permissions(
    jar: &RequestJar,
    group_id: i64,
    role_id: i64,
    permissions: UpdateRolePermissionsRequestPermissions,
) -> Result<(), Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/roles/{}/permissions",
        group_id, role_id
    );

    jar.patch_json(&url, &(UpdateRolePermissionsRequest { permissions }))
        .await?;

    Ok(())
}

/// Gets the permissions for the group's guest role
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 2: The roleset is invalid or does not exist.
/// - 3: You are not authorized to view/edit permissions for this role.
pub async fn guest_permissions(
    jar: &RequestJar,
    group_id: i64,
) -> Result<RolePermissions, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/roles/guest/permissions",
        group_id,
    );

    let response = jar.get_json(&url).await?;

    Ok(response)
}

/// Gets the permissions for all the group's roles
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
///
/// *Note: None were provided in the documentation*
pub async fn permissions(
    jar: &RequestJar,
    group_id: i64,
) -> Result<Vec<RolePermissions>, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/roles/permissions",
        group_id,
    );

    let response = jar
        .get_json::<DataWrapper<Vec<RolePermissions>>>(&url)
        .await?;

    Ok(response.data)
}
