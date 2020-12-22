use chrono::prelude::*;
use diesel::prelude::*;
use juniper::FieldResult;
use serde::{Deserialize, Serialize};

use crate::schema::blogs;
use crate::schema::users::dsl as users_dsl;
use crate::schemas::root::Context;

use super::user::User;

// Default, Insertable, Associations
// #[belongs_to(User, foreign_key="user_id")]
// #[table_name = "blogs"]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Associations, Queryable)]
#[belongs_to(User)]
pub struct Blog {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub summary: String,
    pub status: String,
    pub publish_from: Option<NaiveDateTime>,
    pub publish_until: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[juniper::object(Context = Context)]
impl Blog {
    fn id(&self) -> i32 {
        self.id
    }
    fn user_id(&self) -> i32 {
        self.user_id
    }
    fn title(&self) -> &str {
        self.title.as_str()
    }
    fn content(&self) -> &str {
        self.content.as_str()
    }
    fn summary(&self) -> &str {
        self.summary.as_str()
    }
    fn status(&self) -> &str {
        self.status.as_str()
    }
    fn publish_from(&self) -> Option<NaiveDateTime> {
        self.publish_from
    }
    fn publish_until(&self) -> Option<NaiveDateTime> {
        self.publish_until
    }
    fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
    fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
    // association
    //
    fn user(&self, context: &Context) -> FieldResult<User> {
        let mut conn = context.dbpool.get().unwrap();

        let query = users_dsl::users
            .filter(users_dsl::id.eq(self.user_id))
            .limit(1);

        let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&query);
        println!("DEBUG: {:?}", debug);

        query.get_result::<User>(&conn).map_err(|e| e.into())
    }
}
