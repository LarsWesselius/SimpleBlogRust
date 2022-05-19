use std::{sync::{Arc, Mutex}, convert::Infallible, collections::HashMap, env};
use tera::{Tera, Context};
use std::time::{SystemTime, UNIX_EPOCH};
use warp::{Filter, hyper::Uri};
use log::*;

use crate::db::Db;
use crate::models::post::Post;

pub struct AddPostHandler {
}

impl AddPostHandler {
    pub fn mapping(db: Arc<Mutex<Db>>, tera: Arc<Tera>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let tera_copy = tera.clone();
        let db_copy = db.clone();

        warp::post()
            .and(warp::path!("rust" / "post"))
            .and(warp::body::form())
            .and(warp::any().map(move || -> Arc<Mutex<Db>> { db.clone() }))
            .and(warp::any().map(move || -> Arc<Tera> { tera.clone() }))
            .and_then(AddPostHandler::handle_post_add_post)
        .or(
            warp::get()
                .and(warp::path!("rust" / "post"))
                .and(warp::any().map(move || -> Arc<Mutex<Db>> { db_copy.clone() }))
                .and(warp::any().map(move || -> Arc<Tera> { tera_copy.clone() }))
                .and_then(AddPostHandler::handle_get_add_post)
        )
    }

    pub async fn handle_post_add_post(mut form: HashMap<String, String>, db: Arc<Mutex<Db>>, _tera: Arc<Tera>) -> Result<impl warp::Reply, Infallible> {
        if form.remove("password").unwrap() != env::var("POST_PASSWORD").unwrap_or("unset".to_string()) {
            return Ok(warp::redirect::redirect(Uri::from_static("/rust")));
        }

        // Validate input (only those two because they are required)
        if !form.contains_key("title") || !form.contains_key("content") {
            return Ok(warp::redirect::redirect(Uri::from_static("/rust/post")));
        }

        let db_locked = db.lock().unwrap();
        let seconds_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let post = Post {
            author: form.remove("author"),
            id: None,
            title: form.remove("title").unwrap(),
            content: form.remove("content").unwrap(),
            publish_time: seconds_epoch,
            image_url: form.remove("image_url"),
        };
        
        match db_locked.add_post(&post) {
            Err(e) => error!("Could not save post into database! {:#?}", e),
            Ok(_) => ()
        }

        Ok(warp::redirect::redirect(Uri::from_static("/rust")))
    }

    pub async fn handle_get_add_post(_db: Arc<Mutex<Db>>, tera: Arc<Tera>) -> Result<impl warp::Reply, Infallible> {
        let context = Context::new();
        let html = tera.render("add_post.html", &context).expect("Could not render add_post template!");
        Ok(warp::reply::html(html))
    }
}