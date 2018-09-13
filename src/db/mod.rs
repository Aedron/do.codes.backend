use bson::{to_bson, oid::ObjectId, Document};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::{coll::{Collection}};
use dotenv::dotenv;
use std::env;
use std::option::Option;

pub mod models;
pub mod utils;

use self::models::{NewPost, Post, Comment};
use self::utils::get_timestamp;

lazy_static! {
    pub static ref POST_COLLECTION: Collection = get_coll("posts");
}

pub fn get_coll(coll_name: &str) -> Collection {
    dotenv().ok();
    let db_address = env::var("DB_ADDRESS").unwrap();
    let db_port = env::var("DB_PORT").unwrap();
    let db_name = env::var("DB_NAME").unwrap();

    let client = Client::connect(
        &db_address,
        db_port.parse().unwrap(),
    ).expect("Failed to initialize client.");
    let coll = client.db(&db_name).collection(coll_name);

    coll
}

pub fn get_posts(count: Option<i32>, skip: Option<i64>) -> Vec<Post> {
    let total = POST_COLLECTION.count(None, None);
    let posts = POST_COLLECTION
        .find(None, None)
        .ok()
        .unwrap();

    let mut result: Vec<Post> = vec![];
    for post in posts {
        if let Ok(doc) = post {
            let tags = doc
                .get_array("tags").unwrap().iter()
                .map(|i| i.as_str().unwrap().to_string()).collect();
            let comments: Option<Vec<Comment>> = match doc.get_array("comments") {
                Ok(c) => Some(
                    c.iter().map(|i| {
                        let c = i.as_document().unwrap();
                        Comment {
                            created: c.get_i64("created").unwrap(),
                            username: match c.get_str("username") {
                                Ok(i) => Some(i.to_string()),
                                _ => None
                            },
                            content: c.get_str("content").unwrap().to_string(),
                        }
                    }).collect()
                ),
                _ => None
            };

            let i = Post {
                id: Some(doc.get_object_id("_id").unwrap().to_hex()),
                created: doc.get_i64("created").unwrap(),
                edited: doc.get_i64("edited").ok(),
                title: doc.get_str("title").unwrap().to_string(),
                content: doc.get_str("content").unwrap().to_string(),
                cover: doc.get_str("cover").unwrap().to_string(),
                tags,
                comments,
            };
            result.push(i);
        }
    }
    result
}

pub fn get_post(id: &str) -> Option<Post> {
    let id = match ObjectId::with_string(id) {
        Ok(i) => Some(doc! { "_id": i }),
        Err(_) => return None
    };
    let post = POST_COLLECTION
        .find_one(id, None)
        .ok()
        .unwrap();
    match post {
        Some(doc) => {
            let tags = doc
                .get_array("tags").unwrap().iter()
                .map(|i| i.as_str().unwrap().to_string()).collect();
            let comments: Vec<Comment> = get_comments(doc);

            return Some(Post {
                id: Some(doc.get_object_id("_id").unwrap().to_hex()),
                created: doc.get_i64("created").unwrap(),
                edited: doc.get_i64("edited").ok(),
                title: doc.get_str("title").unwrap().to_string(),
                content: doc.get_str("content").unwrap().to_string(),
                cover: doc.get_str("cover").unwrap().to_string(),
                tags,
                comments,
            });
        }
        None => None
    }
}

pub fn create_post(post: &NewPost) {
    let doc = doc! {
        "created": get_timestamp(),
        "title": post.title.clone(),
        "tags": to_bson(&post.tags).unwrap(),
        "content": post.content.clone(),
        "cover": post.cover.clone(),
    };

    POST_COLLECTION
        .insert_one(doc, None)
        .ok()
        .expect("Failed to insert document.");
}

fn get_comments(doc: Document) -> Vec<Comment> {
    match doc.get_array("comments") {
        Ok(c) => Some(
            c.iter().map(|i| {
                let c = i.as_document().unwrap();
                Comment {
                    created: c.get_i64("created").unwrap(),
                    username: match c.get_str("username") {
                        Ok(i) => Some(i.to_string()),
                        _ => None
                    },
                    content: c.get_str("content").unwrap().to_string(),
                }
            }).collect()
        ),
        _ => vec![]
    }
}