use rspc::Type;
use serde::{Deserialize, Serialize};

use crate::{
    users::MinimalGroupUser,
    util::{jar::RequestJar, responses::DataWrapper, Error},
};

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct PayoutRestrictions {
    pub can_use_recurring_payout: bool,
    pub can_use_one_time_payout: bool,
}

/// Retrieves the payout restrictions for a group.
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 9: You don't have permission to view this group's payouts.
pub async fn get_payout_restrictions(
    jar: &RequestJar,
    group_id: i64,
) -> Result<PayoutRestrictions, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/payout-restrictions",
        group_id
    );
    let response = jar.get_json::<PayoutRestrictions>(&url).await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct RecurringPayout {
    pub user: MinimalGroupUser,
    pub percentage: f64,
}

/// Retrieves a list of recurring payouts for a group.
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 9: You don't have permission to view this group's payouts.
pub async fn get_recurring_payouts(
    jar: &RequestJar,
    group_id: i64,
) -> Result<Vec<RecurringPayout>, Box<Error>> {
    let url = format!("https://groups.roblox.com/v1/groups/{}/payouts", group_id);
    let response = jar
        .get_json::<DataWrapper<Vec<RecurringPayout>>>(&url)
        .await?;
    Ok(response.data)
}

// We dont have a way to create recurring payouts or one time payouts. We likely wont either.
