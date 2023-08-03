// the following file will contain the functions that deal with the blogs
use serde_json;
use std::fs;
// exports the function to be used in the main.rs file

pub fn reading_json_file(path_name: &str) -> serde_json::Value {
    // Read the JSON data from the file
    let data = fs::read_to_string(path_name).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    res
}

// rewrites markdown_to_html function to accept a vector of paths and returns a vector of html strings to be used in the template
pub fn markdown_to_html_strings(vector_paths: &Vec<String>) -> Vec<String> {
    let mut html_output = Vec::new();
    for path in vector_paths {
        let data = fs::read_to_string(path).expect("Unable to read file");
        let parser = pulldown_cmark::Parser::new(&data);
        let mut html_string = String::new();
        pulldown_cmark::html::push_html(&mut html_string, parser);
        html_output.push(html_string);
    }
    html_output
}

// writing a function that does the following: goes into the blogs folder and reads all the markdown file paths and returns a vector of the paths
pub fn get_all_blogs() -> Vec<String> {
    let mut paths = Vec::new();
    let paths_iterator = fs::read_dir("blogs").unwrap();
    for path in paths_iterator {
        let path = path.unwrap().path();
        let path_string = path.to_str().unwrap().to_string();
        paths.push(path_string);
    }
    paths
}
