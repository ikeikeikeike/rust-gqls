use serde::{Deserialize, Serialize};

// Default, Insertable
#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct UsersOrganizations {
    pub user_id: i32,
    pub organization_id: i32,
}
