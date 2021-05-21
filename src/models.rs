use serde::Serialize;
use diesel::*;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String
}
