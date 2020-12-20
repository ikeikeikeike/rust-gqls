use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::schemas::root::Context;

#[derive(Default, Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
}

#[juniper::object(Context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }
}
