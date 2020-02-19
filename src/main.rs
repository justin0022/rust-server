#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate reqwest;
use reqwest::Error;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}", name)
}

struct Breweries {
    id: i32,
    name: String,
    brewery_type: String,
    street: String,
    city: String,
    state: String,
    postal_code: String,
    country: String,
    longitude: String,
    latitude: String,
    phone: String,
    website_url: String,
    updated_at: String,
    tag_list: Vec<String>
}

#[get("/breweries?by_city&<city_name>")]
fn breweries(city_name: String) -> Json<Vec<Breweries>> {
    let request_url = format!("https://api.openbrewerydb.org/breweries?by_city={city_name}", city_name=city_name);
    println!("{}", request_url);

    let mut response = reqwest::get(&request_url);
    let breweries_in_city: Vec<Breweries> = response.json();
    Json(breweries_in_city)
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello]).launch();
}