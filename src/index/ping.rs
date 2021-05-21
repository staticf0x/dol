#[get("/ping")]
pub fn ping() -> &'static str {
    "pong"
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, World!"
}
