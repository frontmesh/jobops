use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub date: String,
    pub tags: Vec<String>,
    pub category: Vec<String>,
    pub slug: String,
    pub image: Option<String>,
}
