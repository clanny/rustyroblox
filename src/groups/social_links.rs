use rspc::Type;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::util::{jar::RequestJar, responses::DataWrapper, Error};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Display, Type)]
pub enum SocialLinkType {
    #[serde(rename = "Facebook")]
    Facebook,
    #[serde(rename = "Twitter")]
    Twitter,
    #[serde(rename = "YouTube")]
    YouTube,
    #[serde(rename = "Twitch")]
    Twitch,
    /// Does this even exist anymore? lol
    #[serde(rename = "GooglePlus")]
    GooglePlus,
    #[serde(rename = "Discord")]
    Discord,
    #[serde(rename = "RobloxGroup")]
    RobloxGroup,
    #[serde(rename = "Amazon")]
    Amazon,
    #[serde(rename = "Guilded")]
    Guilded,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct SocialLink {
    /// Only present when retrieving social links. Do not provide when adding a social link.
    pub id: Option<i64>,
    #[serde(rename = "type")] // Rust doesn't like "type" as a field name
    pub link_type: SocialLinkType,
    pub url: String,
    pub title: String,
}

/// Gets a group's social links.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 11: Social links cannot be processed as this time.
/// - 13: Only users who are over thirteen years of age may view social links.
pub async fn social_links(jar: &RequestJar, group_id: i64) -> Result<Vec<SocialLink>, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/social-links",
        group_id
    );
    let response = jar.get_json::<DataWrapper<Vec<SocialLink>>>(&url).await?;
    Ok(response.data)
}

/// Adds a social link to a group.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 2: You do not have permission to configure this social link.
/// - 3: The social link title is too long.
/// - 4: The social link title cannot be empty.
/// - 5: The social link url cannot be empty.
/// - 6: The social link url was improperly formatted.
/// - 7: The request was null.
/// - 8: The requested group or social link was not found.
/// - 9: The social link type is invalid.
/// - 11: Social links cannot be processed as this time.
/// - 12: The social link title was moderated.
/// - 16: A social link with this type already exists on this group.
pub async fn add_social_link(
    jar: &RequestJar,
    group_id: i64,
    social_link: SocialLink,
) -> Result<(), Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/social-links",
        group_id
    );

    jar.post_json(&url, &social_link).await?;
    Ok(())
}

/// Deletes a social link from a group.
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 2: You do not have permission to configure this social link.
/// - 10: The social link is not for a group.
/// - 11: Social links cannot be processed as this time.
/// - 13: Only users who are over thirteen years of age may edit social links.
/// - 15: The social link id doesn't match the group id.
pub async fn delete_social_link(
    jar: &RequestJar,
    group_id: i64,
    social_link_id: i64,
) -> Result<(), Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/social-links/{}",
        group_id, social_link_id
    );

    jar.delete(&url, "".to_string()).await?;
    Ok(())
}

/// Updates a group's social link.
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 2: You do not have permission to configure this social link.
/// - 3: The social link title is too long.
/// - 4: The social link title cannot be empty.
/// - 5: The social link url cannot be empty.
/// - 6: The social link url was improperly formatted.
/// - 7: The request was null.
/// - 8: The requested group or social link was not found.
/// - 9: The social link type is invalid.
/// - 10: The social link is not for a group.
/// - 11: Social links cannot be processed as this time.
/// - 12: The social link title was moderated.
/// - 16: A social link with this type already exists on this group.
pub async fn update_social_link(
    jar: &RequestJar,
    group_id: i64,
    social_link: SocialLink,
) -> Result<(), Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/social-links/{}",
        group_id,
        social_link.id.unwrap()
    );

    jar.patch_json(&url, &social_link).await?;
    Ok(())
}
