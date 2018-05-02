use actix::*;
use actix_web::*;
use chrono::{DateTime, Utc};


#[derive(Clone,Debug,Serialize)]
pub struct Post {
    pub created: DateTime<Utc>,
    pub edited: Option<DateTime<Utc>>,
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
    pub comments: Option<Vec<Comment>>
}

#[derive(Clone,Debug,Serialize)]
pub struct Comment {
    pub created: u64,
    pub title: String
}
