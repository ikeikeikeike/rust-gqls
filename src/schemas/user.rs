use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Default, Debug, Clone, Serialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
}

#[juniper::object(Context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }
}
