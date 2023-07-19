#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { field: "Mateo", anotherField: "Rust" })
}

#[catch(404)]
fn not_found() -> Template {
    Template::render("404", context!{})
}

#[catch(500)]
fn internal_error() -> Template {
    Template::render("500", context!{})
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Template::fairing()).mount("/", routes![index]).register("/", catchers![not_found, internal_error])
}