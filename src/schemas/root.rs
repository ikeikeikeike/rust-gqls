use juniper::{
    // graphql_value, FieldError,
    FieldResult,
    RootNode,
};

// use actix_web::web;

use diesel::prelude::*;
// use r2d2_mysql::mysql::prelude::*;
// use r2d2_mysql::mysql::{from_row, params, Error as DBError, Row};

use crate::db::Pool;
use crate::schema::users::dsl as users_dsl;

// use super::blog::Blog;
// use super::organization::Organization;
use super::user::User;

pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    // #[graphql(description = "List of all blogs")]
    // fn blogs(context: &Context, limit: i32, offset: i32) -> FieldResult<Vec<Blog>> {
    //     let mut conn = context.dbpool.get().unwrap();
    //     let blogs = conn
    //         .exec_iter(
    //             "select id, user_id, title from blogs limit :limit offset :offset",
    //             params! {"limit" => limit, "offset" => offset},
    //         )
    //         .map(|result| {
    //             result
    //                 .map(|x| x.unwrap())
    //                 .map(|mut row| {
    //                     let (id, user_id, title) = from_row(row);
    //                     Blog { id, user_id, title }
    //                 })
    //                 .collect()
    //         })
    //         .unwrap();
    //     Ok(blogs)
    // }
    //
    // #[graphql(description = "Get Single blog reference by blog ID")]
    // fn blog(context: &Context, id: i32) -> FieldResult<Blog> {
    //     let mut conn = context.dbpool.get().unwrap();
    //
    //     let blog: Result<Option<Row>, DBError> = conn.exec_first(
    //         "SELECT id, user_id, title from blogs WHERE id = :id",
    //         params! {"id" => id},
    //     );
    //
    //     if let Err(err) = blog {
    //         return Err(FieldError::new(
    //             "Blog Not Found",
    //             graphql_value!({ "not_found": "blog not found" }),
    //         ));
    //     }
    //
    //     let (id, user_id, title) = from_row(blog.unwrap().unwrap());
    //     Ok(Blog { id, user_id, title })
    // }

    #[graphql(description = "List of all users")]
    fn users(context: &Context, limit: i32, offset: i32) -> FieldResult<Vec<User>> {
        let mut conn = context.dbpool.get().unwrap();

        // TODO: use web::block to offload blocking Diesel code without blocking server thread
        // let user = web::block(move || users.filter(id.eq(id)).first::<User>(&conn));
        users_dsl::users
            .limit(limit.into())
            .offset(offset.into())
            .load::<User>(&conn)
            .map_err(|e| e.into())
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn user(context: &Context, id: i32) -> FieldResult<User> {
        let mut conn = context.dbpool.get().unwrap();

        // TODO: use web::block to offload blocking Diesel code without blocking server thread
        // let user = web::block(move || users.filter(id.eq(id)).first::<User>(&conn));
        //
        // diesel::print_sql!(users.filter(id.eq(id)).first::<User>(&conn));
        users_dsl::users
            .filter(users_dsl::id.eq(id))
            .first::<User>(&conn)
            .map_err(|e| e.into())
    }

    // #[graphql(description = "List of all organizations")]
    // fn organizations(context: &Context, limit: i32, offset: i32) -> FieldResult<Vec<Organization>> {
    //     let mut conn = context.dbpool.get().unwrap();
    //     let organizations = conn
    //         .exec_iter(
    //             "select id, identifier from organizations limit :limit offset :offset",
    //             params! {"limit" => limit, "offset" => offset},
    //         )
    //         .map(|result| {
    //             result
    //                 .map(|x| x.unwrap())
    //                 .map(|mut row| {
    //                     let (id, identifier) = from_row(row);
    //                     Organization { id, identifier }
    //                 })
    //                 .collect()
    //         })
    //         .unwrap();
    //     Ok(organizations)
    // }
    //
    // #[graphql(description = "Get Single user reference by user ID")]
    // fn organization(context: &Context, id: i32) -> FieldResult<Organization> {
    //     let mut conn = context.dbpool.get().unwrap();
    //     let organization: Result<Option<Row>, DBError> = conn.exec_first(
    //         "SELECT id, identifier from organizations WHERE id = :id",
    //         params! {"id" => id},
    //     );
    //
    //     if let Err(err) = organization {
    //         return Err(FieldError::new(
    //             "Organization Not Found",
    //             graphql_value!({ "not_found": "organization not found" }),
    //         ));
    //     }
    //
    //     let (id, identifier) = from_row(organization.unwrap().unwrap());
    //     Ok(Organization { id, identifier })
    // }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
