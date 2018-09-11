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
extern crate listenfd;

pub mod db;

use listenfd::ListenFd;
use mongodb::coll::{Collection};
use actix_web::{
    server, http, Json,
    App, HttpRequest, Result,
    fs, State,
};
use db::{
    get_coll,
    create_post as create_post_db,
    get_posts as get_posts_db,
    get_post as get_post_db,
    models::{NewPost, Post},
};


struct AppState {
    posts_collection: Collection
}

#[derive(Deserialize, Serialize)]
struct RetData<T> {
    code: u8,
    msg: Option<String>,
    data: Option<T>,
}


type RequestWithState = HttpRequest<AppState>;


fn get_post(req: &RequestWithState) -> Result<Json<RetData<Post>>> {
    let posts_coll = &req.state().posts_collection;
    let id = req.match_info().get("id").unwrap();
    let post = get_post_db(id, posts_coll);
    println!("Id: {:?}\nPost: {:?}", id, post);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: post
    };
    Ok(Json(ret))
}

fn get_posts(req: &RequestWithState) -> Result<Json<RetData<Vec<Post>>>> {
    let posts_coll = &req.state().posts_collection;
    let posts = get_posts_db(Some(0), Some(10), posts_coll);
    println!("{:?}", posts);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: Some(posts),
    };
    Ok(Json(ret))
}

fn create_post(state: State<AppState>, info: Json<NewPost>) -> Result<Json<RetData<Option<String>>>> {
    let post = NewPost {
        title: info.title.clone(),
        tags: info.tags.clone(),
        content: info.content.clone(),
        cover: info.cover.clone(),
    };
    create_post_db(&post, &state.posts_collection);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: None,
    };
    Ok(Json(ret))
}


fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        let app_state = AppState {
            posts_collection: get_coll("posts")
        };

        vec![
            App::with_state(app_state)
                .prefix("/api")
                .resource("/posts", |r| {
                    r.method(http::Method::GET).f(get_posts);
                    r.method(http::Method::POST).with(create_post);
                })
                .resource("/post/{id}", |r| {
                    r.method(http::Method::GET).f(get_post);
                })
                .boxed(),
            App::new()
                .handler("/",
                    fs::StaticFiles::new("./static").ok().unwrap().index_file("index.html"))
                .boxed()
        ]
    });

    server
        .bind("127.0.0.1:8087")
        .expect("Can not bind to 127.0.0.1:8087")
        .run();

//    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
//        server.listen(l)
//    } else {
//        server.bind("127.0.0.1:8087").unwrap();
//    };

//    server.run();
}
