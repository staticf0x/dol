use rocket_contrib::json::Json;
use crate::{TestDbConn, models::User};
use crate::schema::users::dsl::*;
use diesel::{QueryDsl, RunQueryDsl};

#[get("/users")]
pub fn list_users(conn: TestDbConn) -> Json<Vec<User>> {
    let result = users.load::<User>(&*conn).unwrap();

    Json(result)
}

#[get("/users/<user_id>")]
pub fn get_user(conn: TestDbConn, user_id: i32) -> Option<Json<User>> {
    let result = users.find(user_id).first::<User>(&*conn);

    match result {
        Ok(user) => Some(Json(user)),
        Err(_e) => None
    }
}
