
pub struct Post {
    pub created: u64,
//    pub edited: Option<u64>,
    pub title: String,
//    pub tags: Vec<String>,
//    pub comments: Option<Vec<Comment>>
}

pub struct Comment {
    pub created: u64,
    pub title: String
}
