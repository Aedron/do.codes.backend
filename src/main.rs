
#[macro_use(bson, doc)]
extern crate bson;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate mongodb;
extern crate dotenv;
extern crate futures;
extern crate actix_web;
extern crate actix;

mod db;

use std::{
    boxed::Box,
    path::Path,
    collections::HashMap
};
use chrono::prelude::*;
use futures::future::{Future, result};
use mongodb::coll::{Collection, options::{FindOptions}};
use db::{
    get_coll,
    models::{Post, Comment}
};
use actix_web::{
    server::{HttpServer}, http, Error, Json,
    App, HttpRequest, HttpResponse, Result,
    fs::NamedFile, http::Method
};



struct AppState {
    posts: Collection
}

struct RetData<T> {
    code: u8,
    msg: RetMsg,
    data: Option<T>
}

enum RetMsg {
    Success,
    Error
}


type RequestWithState = HttpRequest<AppState>;


fn index(req: RequestWithState) -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("static/index.html"))?)
}

fn get_posts(req: RequestWithState) -> Result<Json<Vec<Post>>> {
    let posts_coll = req.state().posts;
    Ok(Json(posts))
}

fn create_post(req: RequestWithState) -> Result<Json<RetData<None>>> {

}

fn create_app() -> App<AppState> {
    let app_state = AppState {
        posts: get_coll("posts")
    };
    App::with_state(app_state)
        .resource("/", |r| r.method(http::Method::GET).f(index))
        .resource("/api/posts", |r| r.method(http::Method::GET).f(get_posts))
}

fn main() {
    HttpServer::new(create_app)
        .bind("127.0.0.1:8088").expect("Can not bind to 127.0.0.1:0")
        .threads(4)
        .run();
}
