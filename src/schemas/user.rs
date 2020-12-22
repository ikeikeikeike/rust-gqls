use chrono::prelude::*;
use diesel::prelude::*;
use juniper::FieldResult;
use serde::{Deserialize, Serialize};

// use crate::schema::users;
use crate::schema::blogs::dsl as blogs_dsl;
use crate::schema::organizations::dsl as orgs_dsl;
use crate::schemas::root::Context;

use super::blog::Blog;
// use super::organization::Organization;

// Default, Insertable
// #[table_name = "users"]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub who: Option<String>,
    pub identifier: String,
    pub role: String,
    pub email: String,
    // pub password: String,
    pub date_joined: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>,
    pub is_company_receive_unread: bool,
    pub is_company_receive_information: bool,
    pub is_person_receive_information: bool,
    pub is_authed: bool,
    pub is_published: bool,
    pub is_pro: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[juniper::object(Context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }
    fn who(&self) -> Option<&str> {
        self.who.as_deref()
    }
    fn identifier(&self) -> &str {
        self.identifier.as_str()
    }
    fn role(&self) -> &str {
        self.role.as_str()
    }
    fn email(&self) -> &str {
        self.email.as_str()
    }
    fn date_joined(&self) -> NaiveDateTime {
        self.date_joined
    }
    fn last_login(&self) -> Option<NaiveDateTime> {
        self.last_login
    }
    fn is_company_receive_unread(&self) -> bool {
        self.is_company_receive_unread
    }
    fn is_company_receive_information(&self) -> bool {
        self.is_company_receive_information
    }
    fn is_person_receive_information(&self) -> bool {
        self.is_person_receive_information
    }
    fn is_authed(&self) -> bool {
        self.is_authed
    }
    fn is_published(&self) -> bool {
        self.is_published
    }
    fn is_pro(&self) -> bool {
        self.is_pro
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
    // association
    //
    fn blogs(&self, context: &Context, limit: i32, offset: i32) -> FieldResult<Vec<Blog>> {
        let mut conn = context.dbpool.get().unwrap();

        let query = blogs_dsl::blogs
            .filter(blogs_dsl::user_id.eq(self.id))
            .limit(limit.into())
            .offset(offset.into());

        let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&query);
        println!("DEBUG: {:?}", debug);

        query.load::<Blog>(&conn).map_err(|e| e.into())
    }
    // fn organizations(&self, context: &Context, limit: i32, offset: i32) -> FieldResult<Vec<Organization>> {
    //     let mut conn = context.dbpool.get().unwrap();
    //
    //     let query = orgs_dsl::organizations
    //         .filter(orgs_dsl::user_id.eq(self.id))
    //         .limit(limit.into())
    //         .offset(offset.into());
    //
    //     let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&query);
    //     println!("DEBUG: {:?}", debug);
    //
    //     query.load::<Organization>(&conn).map_err(|e| e.into())
    // }
}
