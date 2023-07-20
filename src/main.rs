#[macro_use] extern crate rocket;
use async_std::sync::{Arc, Mutex};

use rocket::serde::{json::json};

use rocket_dyn_templates::{Template, context};
use rocket::form::Form;

use lazy_static::lazy_static;

// global shared data
lazy_static! {
    static ref SHARED_DATA: Arc<Mutex<Vec<String>>> = create_and_work_shared_data();
}
// initialize the vector with some names
fn create_and_work_shared_data() -> Arc<Mutex<Vec<String>>>{
    // create an empty vector inside a Mutex and wrap it in an Arc for shared ownership
    Arc::new(Mutex::new(Vec::new()))
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { field: "Mateo", another_field: "Rust" })
}

#[get("/home")]
fn home() -> Template {
    Template::render("home", context! { field_one: "Welcome", field_two: "Home"})
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", context! { field_one: "About", field_two: "Us"})
}

#[get("/signup-form")]
fn signup_form() -> Template {
    Template::render("signup", context! { field_one: "Sign", field_two: "Up"})
}


#[post("/search", data = "<search>")]
fn search(search: Form<Search>) -> Template {
    // Check if the name field is empty
    if search.name.is_empty() {
        return Template::render("index", context! { error_msg: "Name cannot be empty." });
    }
    return Template::render("index", context!{ success_msg: "Name found.", name: &search.name })
}


#[post("/add", data = "<add>")]
async fn add_data(add: Form<Add>) -> Template {
    // check if the value is empty
    if add.name.is_empty() {
        return Template::render("home", context! { error_msg: "Name cannot be empty." });
    }
    // Clone Arc for each task
    let shared_data_clone = Arc::clone(&SHARED_DATA);

    // Spawn a new asynchronous task to work with the data
    async_std::task::spawn(async move {
        // Lock the Mutex to get exclusive access to the vector
        let mut data = shared_data_clone.lock().await;

        // Update the shared_data with the modified vector
        data.push(add.name.to_string());
    })
    .await; // Await the completion of the task before proceeding

    let data_json = json!(&*SHARED_DATA.lock().await);
    // return the template with the success message and the data_locked
    println!("Vector content: {:?}", data_json);
    return Template::render("home", json!({ "success_msg": "Name added.", "data": data_json }));
}

#[post("/signingup", data = "<signup>")]
fn signup(signup: Form<Signup>) -> Template {
    // Check if the name field is empty
    if signup.username.is_empty() {
        return Template::render("signup", context! { error_msg: "Name cannot be empty." });
    }
    println!("Username: {}", signup.username);
    return Template::render("login", context!{ success_msg: "Account created!", name: &signup.username })
}


#[derive(FromForm)]
struct Search {
    name: String,
}

#[derive(FromForm)]
struct Signup {
    username: String,
    password: String,
}

#[derive(FromForm)]
struct Add {
    name: String,
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
    rocket::build().attach(Template::fairing()).mount("/", routes![index, home, about, search, add_data, signup, signup_form]).register("/", catchers![not_found, internal_error])
}