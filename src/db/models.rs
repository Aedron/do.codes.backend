
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct NewPost {
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
    pub cover: String
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Post {
    pub id: Option<String>,
    pub created: i64,
    pub edited: Option<i64>,
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
    pub cover: String,
    pub comments: CommentResponse
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

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct CommentResponse {
    pub total: i32,
    pub data: Option<Vec<Comment>>
}
