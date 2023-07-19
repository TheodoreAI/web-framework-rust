#[macro_use] extern crate rocket;


#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/home")]
fn home() -> &'static str {
    "Welcome to my home page made with Rust!"
}

#[get("/hello/<name>")]
fn hello_name(name: &str) -> String {
    format!("Hello, {}!", name)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, home, hello_name])
}