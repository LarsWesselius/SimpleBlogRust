mod index;
mod db;
mod models;
mod blog;
mod add_post;

use std::{sync::{Mutex, Arc}, env};

use tera::Tera;
use warp::Filter;
use log::*;

use index::IndexHandler;
use db::Db;
use blog::BlogHandler;
use add_post::AddPostHandler;

#[tokio::main]
async fn main() {
    setup_logging();

    env::var("POST_PASSWORD").expect("POST_PASSWORD environment variable not set. Please set it.");

    info!("Setting up Tera templating engine..");

    let mut tera = Tera::new("templates/**/*.html").expect("Failed to initialize templating engine!");
    tera.autoescape_on(vec![".html"]);
    let tera_arc = Arc::new(tera);

    info!("Setting up SQLite..");

    let db = Db::new();
    db.bootstrap().await.unwrap();
    let db_arc = Arc::new(Mutex::new(db));

    info!("Setting up routes..");

    let routes = 
        warp::get().and(warp::path("static")).and(warp::fs::dir("static"))
        .or(IndexHandler::mapping(db_arc.clone(), tera_arc.clone()))
        .or(AddPostHandler::mapping(db_arc.clone(), tera_arc.clone()))
        .or(BlogHandler::mapping(db_arc.clone(), tera_arc.clone()));

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

fn setup_logging(){
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .filter(Some("simpleblogrust"), log::LevelFilter::Trace)
        .write_style(env_logger::WriteStyle::Always)
        .init();
}
