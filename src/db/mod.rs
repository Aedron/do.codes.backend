use bson::{
    Bson, ordered::OrderedDocument,
    to_bson, UtcDateTime,
};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::{
    coll::{
        Collection,
        options::{FindOptions, CursorType},
    }
};
use dotenv::dotenv;
use std::env;
use std::option::Option;
use std::iter::FromIterator;

pub mod models;
pub mod utils;

use self::models::{NewPost, Post, Comment, PostResponse};
use self::utils::get_timestamp;


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

pub fn get_posts(count: Option<i32>, skip: Option<i64>, collection: &Collection) -> Vec<Post> {
    let total = collection.count(None, None);
    let mut posts = collection
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
                tags, comments
            };
            result.push(i);
        }
    }
    result
}

pub fn create_post(post: &NewPost, collection: &Collection) {
    let doc = doc! {
        "created": get_timestamp(),
        "title": post.title.clone(),
        "tags": to_bson(&post.tags).unwrap(),
        "content": post.content.clone(),
        "cover": post.cover.clone(),
    };

    collection
        .insert_one(doc, None)
        .ok()
        .expect("Failed to insert document.");
}
