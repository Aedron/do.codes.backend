
use bson::{Bson, ordered::{OrderedDocument}};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::{Collection, options::{FindOptions}};
use dotenv::dotenv;
use std::env;
use std::option::Option;

pub mod models;
use self::models::{Post, Comment};



fn get_coll(coll_name: &str) -> Collection {
    dotenv().ok();
    let db_address = env::var("DB_ADDRESS").unwrap();
    let db_port = env::var("DB_PORT").unwrap();
    let db_name = env::var("DB_NAME").unwrap();

    let client = Client::connect(
        &db_address,
        db_port.parse().unwrap()
    ).expect("Failed to initialize client.");
    let coll = client.db(&db_name).collection(coll_name);

    let mut cursor = coll.find(None, None).unwrap();
    for result in cursor {
        if let Ok(item) = result {
            if let Some(&Bson::String(ref title)) = item.get("title") {
                println!("title: {}", title);
            }
        }
    }

    coll
}

fn find_from_coll(coll: Collection, doc: OrderedDocument, options: Option<FindOptions>) {
    let mut cursor = coll
        .find(Some(doc), options)
        .ok().expect("Failed to execute find.");

    let item = cursor.next();
    match item {
        Some(Ok(doc)) => match doc.get("title") {
            Some(&Bson::String(ref title)) => println!("{}", title),
            _ => panic!("Expected title to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned no results!"),
    }
}


pub fn get_all_posts() {
    let collection = get_coll("posts");
    let doc = doc! {
        "title": "Jaws",
        "array": [ 1, 2, 3 ],
    };
    find_from_coll(collection, doc.clone(), None);
}

pub fn new_post(post: &Post) {
    let collection = get_coll("posts");
    let doc = doc! {
        "created": post.created,
        "title": &post.title
    };

    collection.insert_one(doc.clone(), None)
        .ok().expect("Failed to insert document.");
}
