
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

pub mod db;

use std::{
    boxed::Box,
    path::Path,
    collections::HashMap
};
use futures::future::{Future, result};
use mongodb::coll::{Collection, options::{FindOptions}};
use actix_web::{
    server::{HttpServer}, http, Error, Json,
    App, HttpRequest, HttpResponse, Result,
    fs::NamedFile, http::Method, State
};
use db::{
    get_coll, create_post as create_post_db, get_posts as get_posts_db,
    models::{NewPost, Post, Comment},
    utils::get_timestamp
};



struct AppState {
    posts: Collection
}

#[derive(Deserialize, Serialize)]
struct RetData<T> {
    code: u8,
    msg: Option<String>,
    data: Option<T>
}


type RequestWithState = HttpRequest<AppState>;


fn index(req: RequestWithState) -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("static/index.html"))?)
}

fn get_posts(req: RequestWithState) -> Result<Json<RetData<Vec<Post>>>> {
    let posts_coll = &req.state().posts;
    let posts = get_posts_db(Some(0), Some(10), posts_coll);
    println!("{:?}", posts);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: Some(posts)
    };
    Ok(Json(ret))
}

fn create_post(state: State<AppState>, info: Json<NewPost>) -> Result<Json<RetData<Option<String>>>> {
    let post = NewPost {
        title: info.title.clone(),
        tags: info.tags.clone(),
        content: info.content.clone(),
        cover: info.cover.clone()
    };
    create_post_db(&post, &state.posts);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: None
    };
    Ok(Json(ret))
}

fn create_app() -> App<AppState> {
    let app_state = AppState {
        posts: get_coll("posts")
    };
    App::with_state(app_state)
        .resource("/", |r| {
            r.method(http::Method::GET).f(index);
        })
        .resource("/api/posts", |r| {
            r.method(http::Method::GET).f(get_posts);
            r.method(http::Method::POST).with2(create_post);
        })
}

fn main() {
    HttpServer::new(create_app)
        .bind("127.0.0.1:8087").expect("Can not bind to 127.0.0.1:8087")
        .threads(4)
        .run();
}
