use chrono::prelude::*;
use serde::{Deserialize, Serialize};

// use crate::schema::organizations;
use crate::schemas::root::Context;

// Default, Insertable
#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct Organization {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub identifier: String,
    pub role: String,
    pub is_authed: bool,
    pub is_published: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[juniper::object(Context = Context)]
impl Organization {
    fn id(&self) -> i32 {
        self.id
    }
    fn parent_id(&self) -> Option<i32> {
        self.parent_id
    }
    fn identifier(&self) -> &str {
        self.identifier.as_str()
    }
    fn role(&self) -> &str {
        self.role.as_str()
    }
    fn is_authed(&self) -> bool {
        self.is_authed
    }
    fn is_published(&self) -> bool {
        self.is_published
    }
    fn is_deleted(&self) -> bool {
        self.is_deleted
    }
    fn deleted_at(&self) -> Option<NaiveDateTime> {
        self.deleted_at
    }
    fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
    fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
