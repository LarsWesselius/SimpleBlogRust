use std::{sync::{Arc, Mutex}, convert::Infallible};

use tera::{Tera, Context};
use warp::Filter;

use crate::db::Db;

pub struct IndexHandler {
}

impl IndexHandler {
    pub fn mapping(db: Arc<Mutex<Db>>, tera: Arc<Tera>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path::end()
            .and(warp::any().map(move || -> Arc<Mutex<Db>> { db.clone() }))
            .and(warp::any().map(move || -> Arc<Tera> { tera.clone() }))
            .and_then(IndexHandler::handle_index)
    }

    pub async fn handle_index(_db: Arc<Mutex<Db>>, tera: Arc<Tera>) -> Result<impl warp::Reply, Infallible> {
        let context = Context::new();

        let html = tera.render("index.html", &context).expect("Could not render index html template!");
        Ok(warp::reply::html(html))
    }
}