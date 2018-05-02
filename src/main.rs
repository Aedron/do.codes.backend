
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate rocket;
extern crate dotenv;

use rocket::response::content;

mod db;
use db::models::{Post, Comment};

#[get("/")]
fn index() -> &'static str {
    let post = Post {
        created: 0,
        title: String::from("test")
    };
    db::new_post(&post);
    "Hello, world!"
}

//#[post("/posts", format="application/json", data="<message>")]
//fn new_post() -> content::Json<&'static str> {
//
//}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
