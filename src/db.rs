use rusqlite::{Connection, params};
use log::*;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::models::post::Post;

pub struct Db {}

impl Db {
    pub fn new() -> Db {
        Db { }
    }

    pub fn get_connection(&self) -> Connection {
        Connection::open("sqlite.db").expect("Could not open/create SQLite db!")
    }

    pub async fn bootstrap(&self) -> Result<(), Box<dyn std::error::Error>> {
        let con = self.get_connection();

        let result: rusqlite::Result<String> = con.query_row("SELECT name FROM sqlite_master WHERE type='table' AND name = 'posts'", [], |row| row.get(0));

        if let Err(_) = result {
            info!("Database empty, creating posts table..");
            con.execute(r#"CREATE TABLE posts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title NVARCHAR(255) NOT NULL,
                publish_time INT NOT NULL,
                content TEXT NULL,
                author NVARCHAR(255) NULL,
                image_url NVARCHAR(255) NULL
                );"#, [])?;
        }

        let post_count: u32 = con.query_row("SELECT COUNT(*) FROM posts", [], |row| row.get(0))?;
        if post_count == 0 {
            info!("No existing post! Adding one..");

            con.execute("INSERT INTO posts (title, content, publish_time, author, image_url) VALUES (?1, ?2, ?3, ?4, ?5)", params![
                "Simple Blog — Programming example in different languages",
                "Just a placeholder for a *proper* post.",
                SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
                "Someone",
                "https://picsum.photos/seed/simple/900/400"
            ])?;
        }

        Ok(())
    }

    pub fn get_post(&self, id: u32) -> Result<Option<Post>, Box<dyn std::error::Error>> {
        let con = self.get_connection();

        let mut stmt = con.prepare("SELECT id, title, content, publish_time, author, image_url FROM posts WHERE id = ?1")?;

        let post = stmt.query_row(params![id], |row| {
            Ok(Post {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                publish_time: row.get(3)?,
                author: row.get(4)?,
                image_url: row.get(5)?
            })
        })?;

        Ok(Some(post))
    }

    pub fn get_posts(&self, count: u32) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
        let con = self.get_connection();

        let mut stmt = con.prepare("SELECT id, title, content, publish_time, author, image_url FROM posts ORDER BY publish_time DESC LIMIT ?1")?;

        let post = stmt.query_map(params![count], |row| {
            Ok(Post {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                publish_time: row.get(3)?,
                author: row.get(4)?,
                image_url: row.get(5)?
            })
        })?;

        Ok(post.map(|r| r.unwrap()).collect())
    }

    pub fn add_post(&self, post: &Post) -> Result<(), Box<dyn std::error::Error>> {
        let con = self.get_connection();
        
        con.execute("INSERT INTO posts (title, content, author, image_url, publish_time) VALUES(?1, ?2, ?3, ?4, ?5)", params![
            post.title,
            post.content,
            post.author,
            post.image_url,
            post.publish_time
        ])?;

        Ok(())
    }
}