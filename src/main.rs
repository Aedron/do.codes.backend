#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use(bson, doc)]

extern crate bson;
extern crate mongodb;
extern crate rocket;

mod db;
use db::{ DB, DBConfig };


#[get("/")]
fn index() -> &'static str {
    let config = DBConfig {
        address: String::from("localhost"),
        host: 27017,
        db: String::from("blog"),
        collection: String::from("posts")
    };
    let db = DB::connect(config);
    "Hello, world!"
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}
