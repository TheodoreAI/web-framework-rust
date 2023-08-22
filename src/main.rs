#[macro_use]
extern crate rocket;
use async_std::sync::{Arc, Mutex};

use rocket::serde::json::json;

use rocket::form::Form;
use rocket_dyn_templates::{context, Template};

use lazy_static::lazy_static;

use serde_json;

// refactors the function above to be imported from a module called blogs.rs
// the following file will contain the functions that deal with the blogs
mod blog_helpers;
use blog_helpers::get_all_blogs;
use blog_helpers::markdown_to_html_strings;
use blog_helpers::reading_json_file;

// write an enum to
// global shared data
lazy_static! {
    static ref SHARED_DATA: Arc<Mutex<Vec<String>>> = create_and_work_shared_data();
}
// initialize the vector with some names
fn create_and_work_shared_data() -> Arc<Mutex<Vec<String>>> {
    // create an empty vector inside a Mutex and wrap it in an Arc for shared ownership
    Arc::new(Mutex::new(Vec::new()))
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! { title_page: "Dashboard", app_name: "Rust Website", data_table: [1, 2, 3, 4, 5], description: "The following is a project that I am working on to learn Rust. I am using the Rocket framework to build a website. I am also using the Rocket rocket_dyn_templates to render Handlebar templates. The goal is to program a set of Arcade games of names from A-Z."},
    )
}

#[get("/home")]
fn home() -> Template {

    Template::render(
        "home",
        context! { field_one: "Welcome", field_two: "Home", users: reading_json_file("users.json")},
    )
}

#[get("/about")]
fn about() -> Template {
    Template::render(
        "about",
        context! { field_one: "About", field_two: "Us", about_us: reading_json_file("about.json")},
    )
}

// refactor the blogs function to accept a search query search?name= from a form and filter the blogs based on the search query
// if the search query is empty, then all the blogs will be displayed
#[get("/blogs?<search>")]
fn blogs(search: Option<String>) -> Template {
    // reads the markdown file and converts it to html
    let html_output = markdown_to_html_strings(&get_all_blogs());
    // converts the html into an array of json objects that can be used in the template in a for loop
    let html_output = serde_json::to_value(&html_output).unwrap();
    // if the search query is empty, then all the blogs will be displayed
    if search.is_none() {
        return Template::render(
            "blogs",
            context! { title: "Blogs", field_two: "Blog posts", blogs: html_output, is_blogs_page: true},
        );
    }

    let mut query_not_found = true;
    let mut found_msg = "Search query found";

    let search = search.unwrap();
    let mut filtered_blogs = Vec::new();
    for blog in html_output.as_array().unwrap() {
        let blog = blog.as_str().unwrap();
        if blog.contains(&search) {
            filtered_blogs.push(blog.to_string());
            query_not_found = false;
            found_msg = "Search query found";
            println!("FOUND IT");
        } 
    }
    let filtered_blogs = serde_json::to_value(&filtered_blogs).unwrap();
    Template::render(
        "blogs",
        context! { title: "Blogs", field_two: "Blog posts", blogs: filtered_blogs, query_not_found: query_not_found, found_msg: found_msg, is_blogs_page: true},
    )
}

#[post("/search", data = "<search>")]
fn search(search: Form<Search>) -> Template {
    // Check if the name field is empty
    if search.name.is_empty() {
        return Template::render("blogs", context! { error_msg: "Name cannot be empty." });
    }
    return Template::render(
        "blogs",
        context! { success_msg: "Name found.", name: &search.name },
    );
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
    return Template::render(
        "home",
        json!({ "success_msg": "Name added.", "data": data_json }),
    );
}

#[derive(FromForm)]
struct Search {
    name: String,
}

#[derive(FromForm)]
struct Add {
    name: String,
}

#[catch(404)]
fn not_found() -> Template {
    Template::render("404", context! {})
}

#[catch(500)]
fn internal_error() -> Template {
    Template::render("500", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(
            rocket::Config::figment()
                .merge(("port", 8080))
        )
        .attach(Template::fairing())
        .mount("/", routes![index, home, about, search, add_data, blogs])
        .register("/", catchers![not_found, internal_error])
}
