#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "No rockets here!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}