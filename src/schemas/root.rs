use diesel::prelude::*;
use juniper::{
    // graphql_value, FieldError,
    FieldResult,
    RootNode,
};

// use actix_web::web;

use crate::db::Pool;
use crate::schema::{
    blogs::dsl as blogs_dsl, organizations::dsl as organizations_dsl, users::dsl as users_dsl,
};

use super::blog::Blog;
use super::organization::Organization;
use super::user::User;

pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all blogs")]
    fn blogs(context: &Context, limit: i32, offset: i32) -> FieldResult<Vec<Blog>> {
        let mut conn = context.dbpool.get().unwrap();

        let query = blogs_dsl::blogs.limit(limit.into()).offset(offset.into());

        let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&query);
        println!("DEBUG: {:?}", debug);

        query.load::<Blog>(&conn).map_err(|e| e.into())
    }

    #[graphql(description = "Get Single blog reference by blog ID")]
    fn blog(context: &Context, id: i32) -> FieldResult<Blog> {
        let mut conn = context.dbpool.get().unwrap();

        let query = blogs_dsl::blogs.filter(blogs_dsl::id.eq(id)).limit(1);

        let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&query);
        println!("DEBUG: {:?}", debug);

        query.get_result::<Blog>(&conn).map_err(|e| e.into())
    }

    #[graphql(description = "List of all users")]
    fn users(context: &Context, limit: i32, offset: i32) -> FieldResult<Vec<User>> {
        let mut conn = context.dbpool.get().unwrap();

        let query = users_dsl::users.limit(limit.into()).offset(offset.into());

        let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&query);
        println!("DEBUG: {:?}", debug);

        query.load::<User>(&conn).map_err(|e| e.into())
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn user(context: &Context, id: i32) -> FieldResult<User> {
        let mut conn = context.dbpool.get().unwrap();

        let query = users_dsl::users.filter(users_dsl::id.eq(id)).limit(1);

        let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&query);
        println!("DEBUG: {:?}", debug);

        query.get_result::<User>(&conn).map_err(|e| e.into())
    }

    #[graphql(description = "List of all organizations")]
    fn organizations(context: &Context, limit: i32, offset: i32) -> FieldResult<Vec<Organization>> {
        let mut conn = context.dbpool.get().unwrap();

        let query = organizations_dsl::organizations
            .limit(limit.into())
            .offset(offset.into());

        let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&query);
        println!("DEBUG: {:?}", debug);

        query.load::<Organization>(&conn).map_err(|e| e.into())
    }

    #[graphql(description = "Get Single organization reference by organization ID")]
    fn organization(context: &Context, id: i32) -> FieldResult<Organization> {
        let mut conn = context.dbpool.get().unwrap();

        let query = organizations_dsl::organizations
            .filter(organizations_dsl::id.eq(id))
            .limit(1);

        let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&query);
        println!("DEBUG: {:?}", debug);

        query
            .get_result::<Organization>(&conn)
            .map_err(|e| e.into())
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
