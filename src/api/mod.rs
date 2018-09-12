
use actix_web::{Json, Result, State, HttpRequest};
use db::{
    create_post as create_post_db,
    get_posts as get_posts_db,
    get_post as get_post_db,
    models::{NewPost, Post},
};


pub mod models;

use self::models::{
    RetData
};


pub fn get_post(req: &HttpRequest) -> Result<Json<RetData<Post>>> {
    let id = req.match_info().get("id").unwrap();
    let post = get_post_db(id);
    println!("Id: {:?}\nPost: {:?}", id, post);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: post
    };
    Ok(Json(ret))
}

pub fn get_posts(req: &HttpRequest) -> Result<Json<RetData<Vec<Post>>>> {
    let posts = get_posts_db(Some(0), Some(10));
    println!("{:?}", posts);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: Some(posts),
    };
    Ok(Json(ret))
}

pub fn create_post(info: Json<NewPost>) -> Result<Json<RetData<Option<String>>>> {
    let post = NewPost {
        title: info.title.clone(),
        tags: info.tags.clone(),
        content: info.content.clone(),
        cover: info.cover.clone(),
    };
    create_post_db(&post);
    let ret = RetData {
        code: 0,
        msg: Some(String::from("success")),
        data: None,
    };
    Ok(Json(ret))
}
