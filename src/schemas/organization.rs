use chrono::prelude::*;
use serde::{Deserialize, Serialize};

// use crate::schema::organizations;
use crate::schemas::root::Context;

// Default, Insertable
#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct Organization {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[juniper::object(Context = Context)]
impl Organization {
    fn id(&self) -> i32 {
        self.id
    }
    fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
    fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
