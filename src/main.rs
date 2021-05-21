#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;

mod index;
mod users;
mod models;
mod schema;

#[database("test_db")]
pub struct TestDbConn(diesel::SqliteConnection);

fn main() {
    rocket::ignite()
        .attach(TestDbConn::fairing())
        .mount("/", routes![index::ping::ping, index::ping::hello, users::list_users, users::get_user])
        .launch();
}
