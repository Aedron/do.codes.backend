use actix::*;
use actix_web::*;


#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct NewPost {
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Post {
    pub created: i64,
    pub edited: Option<i64>,
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
    pub comments: Option<Vec<Comment>>
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Comment {
    pub created: i64,
    pub username: Option<String>,
    pub content: String
}


#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct PostResponse {
    pub total: u32,
    pub count: u32,
    pub skip: u32,
    pub posts: Vec<Post>
}
