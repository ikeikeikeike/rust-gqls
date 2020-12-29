use chrono::prelude::*;
use diesel::prelude::*;
use juniper::FieldResult;
use serde::{Deserialize, Serialize};

use crate::schema::users::dsl as users_dsl;
use crate::schema::users_organizations::dsl as users_orgs_dsl;
use crate::schemas::root::Context;

use super::through::UsersOrganizations;
use super::user::User;

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
    fn users(&self, context: &Context, limit: i32, offset: i32) -> FieldResult<Vec<User>> {
        let mut conn = context.dbpool.get().unwrap();

        let query = users_dsl::users
            .inner_join(users_orgs_dsl::users_organizations)
            .filter(users_orgs_dsl::organization_id.eq(self.id))
            .limit(limit.into())
            .offset(offset.into());

        let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&query);
        log::debug!("QUERY: {:?}", debug);

        query
            .load::<(User, UsersOrganizations)>(&conn)
            .map(|e| e.into_iter().map(|(user, _)| user).collect::<Vec<User>>())
            .map_err(|e| e.into())
    }
}
