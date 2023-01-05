use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::util::{jar::RequestJar, Error};

use super::Group;

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRequest {
    pub group_ids: Vec<usize>,
}

/// Batch management of group relationships (enemies and allies)
pub mod relationships {

    use crate::util::{jar::RequestJar, Error};

    use super::{BatchRequest, RelationshipType};

    /// Declines a batch of group relationships for a group.
    #[async_recursion::async_recursion]
    #[allow(unused_assignments)] // It appears to be a bug in the checker, this kinda fixes it.
    pub async fn decline(
        jar: &mut RequestJar,
        group_id: usize,
        group_ids: Vec<usize>,
        relationship_type: RelationshipType,
    ) -> Result<(), Box<Error>> {
        if relationship_type == RelationshipType::All {
            decline(jar, group_id, group_ids.clone(), RelationshipType::Enemy).await?;
            decline(jar, group_id, group_ids.clone(), RelationshipType::Ally).await?;
            return Ok(());
        }

        let url = format!(
            "https://groups.roblox.com/v1/groups/{}/relationships/{}/requests",
            group_id,
            relationship_type.get_relationship_type_string()
        );
        let request = BatchRequest { group_ids };
        let response = jar.delete_json::<(), BatchRequest>(&url, request).await?;
        Ok(response)
    }

    /// Accepts a batch of group relationships for a group.
    #[async_recursion::async_recursion]
    #[allow(unused_assignments)] // It appears to be a bug in the checker, this kinda fixes it.
    pub async fn accept(
        jar: &mut RequestJar,
        group_id: usize,
        group_ids: Vec<usize>,
        relationship_type: RelationshipType,
    ) -> Result<(), Box<Error>> {
        if relationship_type == RelationshipType::All {
            accept(jar, group_id, group_ids.clone(), RelationshipType::Enemy).await?;
            accept(jar, group_id, group_ids.clone(), RelationshipType::Ally).await?;
            return Ok(());
        }

        let url = format!(
            "https://groups.roblox.com/v1/groups/{}/relationships/{}/requests",
            group_id,
            relationship_type.get_relationship_type_string()
        );
        let request = BatchRequest { group_ids };
        let response = jar.post_json::<(), BatchRequest>(&url, request).await?;
        Ok(response)
    }
}

/// Management of 1 group relationship at a time  (enemies and allies)
pub mod relationship {

    use crate::util::{jar::RequestJar, Error};

    use super::RelationshipType;

    /// Declines a group relationships for a group.
    #[async_recursion::async_recursion]
    #[allow(unused_assignments)] // It appears to be a bug in the checker, this kinda fixes it.
    pub async fn decline(
        jar: &mut RequestJar,
        group_id: usize,
        relation_group_id: usize,
        relationship_type: RelationshipType,
    ) -> Result<(), Box<Error>> {
        let url = format!(
            "https://groups.roblox.com/v1/groups/{}/relationships/{}/requests/{}",
            group_id,
            relationship_type.get_relationship_type_string(),
            relation_group_id
        );

        jar.delete(&url, true, "".to_string()).await?;

        Ok(())
    }

    /// Accepts a group relationships for a group.
    #[async_recursion::async_recursion]
    #[allow(unused_assignments)] // It appears to be a bug in the checker, this kinda fixes it.
    pub async fn accept(
        jar: &mut RequestJar,
        group_id: usize,
        relation_group_id: usize,
        relationship_type: RelationshipType,
    ) -> Result<(), Box<Error>> {
        let url = format!(
            "https://groups.roblox.com/v1/groups/{}/relationships/{}/requests/{}",
            group_id,
            relationship_type.get_relationship_type_string(),
            relation_group_id
        );

        jar.post(&url, true, "".to_string()).await?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Display)]
pub enum RelationshipType {
    #[serde(rename = "Enemies")]
    Enemy,
    #[serde(rename = "Allies")]
    Ally,
    All,
}

impl RelationshipType {
    pub fn get_relationship_type_string(&self) -> String {
        match self {
            RelationshipType::Enemy => "Enemies",
            RelationshipType::Ally => "Allies",
            RelationshipType::All => "All",
        }
        .to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupRelationships {
    pub group_id: usize,
    pub relationship_type: RelationshipType,
    pub total_group_count: usize,
    #[serde(rename = "relatedGroups")]
    pub groups: Vec<Group>,
}

/// Retrieves a list of groups that are enemies, allies or both of the specified group.
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 4: Group relationship type or request type is invalid.
/// - 8: Invalid or missing pagination parameters
#[async_recursion::async_recursion]
pub async fn relationships(
    jar: &mut RequestJar,
    group_id: usize,
    relationship_type: RelationshipType,
) -> Result<GroupRelationships, Box<Error>> {
    if relationship_type == RelationshipType::All {
        let enemies = relationships(jar, group_id, RelationshipType::Enemy).await?;
        let allies = relationships(jar, group_id, RelationshipType::Ally).await?;
        let mut groups = enemies.groups;
        groups.extend(allies.groups);
        return Ok(GroupRelationships {
            group_id,
            relationship_type,
            total_group_count: enemies.total_group_count + allies.total_group_count,
            groups,
        });
    }

    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/relationships/{}?model.startRowIndex=0&model.maxRows=1000",
        group_id, relationship_type.get_relationship_type_string()
    );
    let response = jar.get_json::<GroupRelationships>(&url).await?;
    Ok(response)
}

/// Retrieves a list of groups that are enemies of the specified group.
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 4: Group relationship type or request type is invalid.
/// - 8: Invalid or missing pagination parameters
pub async fn enemies(
    jar: &mut RequestJar,
    group_id: usize,
) -> Result<GroupRelationships, Box<Error>> {
    relationships(jar, group_id, RelationshipType::Enemy).await
}

/// Retrieves a list of groups that are allies of the specified group.
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 4: Group relationship type or request type is invalid.
/// - 8: Invalid or missing pagination parameters
pub async fn allies(
    jar: &mut RequestJar,
    group_id: usize,
) -> Result<GroupRelationships, Box<Error>> {
    relationships(jar, group_id, RelationshipType::Ally).await
}

/// Retrieves a list of enemy, ally or both relationship requests for the specified group.
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 4: Group relationship type or request type is invalid.
/// - 5: You don't have permission to manage this group's relationships.
/// - 8: Invalid or missing pagination parameters
#[async_recursion::async_recursion]
pub async fn relationship_requests(
    jar: &mut RequestJar,
    group_id: usize,
    relationship_type: RelationshipType,
) -> Result<GroupRelationships, Box<Error>> {
    if relationship_type == RelationshipType::All {
        let enemies = relationship_requests(jar, group_id, RelationshipType::Enemy).await?;
        let allies = relationship_requests(jar, group_id, RelationshipType::Ally).await?;
        let mut groups = enemies.groups;
        groups.extend(allies.groups);
        return Ok(GroupRelationships {
            group_id,
            relationship_type,
            total_group_count: enemies.total_group_count + allies.total_group_count,
            groups,
        });
    }

    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/relationships/{}?model.startRowIndex=0&model.maxRows=1000",
        group_id, relationship_type.get_relationship_type_string()
    );
    let response = jar.get_json::<GroupRelationships>(&url).await?;
    Ok(response)
}

/// Retrieves a list of groups that are enemies of the specified group.
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 4: Group relationship type or request type is invalid.
/// - 5: You don't have permission to manage this group's relationships.
/// - 8: Invalid or missing pagination parameters
pub async fn enemy_requests(
    jar: &mut RequestJar,
    group_id: usize,
) -> Result<GroupRelationships, Box<Error>> {
    relationship_requests(jar, group_id, RelationshipType::Enemy).await
}

/// Retrieves a list of groups that are allies of the specified group.
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 4: Group relationship type or request type is invalid.
/// - 5: You don't have permission to manage this group's relationships.
/// - 8: Invalid or missing pagination parameters
pub async fn ally_requests(
    jar: &mut RequestJar,
    group_id: usize,
) -> Result<GroupRelationships, Box<Error>> {
    relationship_requests(jar, group_id, RelationshipType::Ally).await
}
