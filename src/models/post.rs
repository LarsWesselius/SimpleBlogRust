use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: Option<u32>,
    pub title: String,
    pub content: String,
    pub publish_time: u64,
    pub author: Option<String>,
    pub image_url: Option<String>
}