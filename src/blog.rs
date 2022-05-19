use std::{sync::{Arc, Mutex}, convert::Infallible};
use pulldown_cmark::{Parser, Options, html};

use tera::{Tera, Context};
use warp::{Filter, hyper::Uri};
use log::*;
use crate::db::Db;
use crate::models::post::Post;

pub struct BlogHandler {
}

impl BlogHandler {
    pub fn mapping(db: Arc<Mutex<Db>>, tera: Arc<Tera>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let opt = warp::path::param::<u32>()
            .map(Some)
            .or_else(|_| async {
                Ok::<(Option<u32>,), std::convert::Infallible>((None,))
            });

            warp::get()
                .and(warp::path!("rust" / ..))
                .and(opt)
                .and(warp::any().map(move || -> Arc<Mutex<Db>> { db.clone() }))
                .and(warp::any().map(move || -> Arc<Tera> { tera.clone() }))
                .and_then(BlogHandler::handle_blog)
    }

    pub async fn handle_blog(parameter: Option<u32>, db: Arc<Mutex<Db>>, tera: Arc<Tera>) -> Result<Box<dyn warp::reply::Reply>, Infallible> {
        let mut context = Context::new();

        let db_locked = db.lock().unwrap();

        let recent_posts: Vec<Post>;
        if let Ok(posts) = db_locked.get_posts(20) {
            recent_posts = posts;
        } else {
            error!("Could not retrieve posts!");
            return Ok(Box::new(warp::redirect::redirect(Uri::from_static("/rust/post"))));
        }

        if recent_posts.len() == 0 {
            error!("There are no posts yet, this should not happen..anyway, let's send the user to create one");
            return Ok(Box::new(warp::redirect::redirect(Uri::from_static("/rust/post"))));
        }

        let blog_post: Post;
        let blog_post_ref: &Post;
        if let Some(blog_id) = parameter {
            let temp_post = db_locked.get_post(blog_id);
            match temp_post {
                Ok(blog) => {
                    blog_post = blog.unwrap(); // Why assign to blog_post? Because of lifetimes - if we'd assign blog_post_ref immediately, it would complain about the Post being dropped at end of scope
                    blog_post_ref = &blog_post;
                },
                Err(_) => { 
                    warn!("Post not found!");
                    blog_post_ref = recent_posts.get(0).unwrap();
                }
            }
        } else {
            blog_post_ref = recent_posts.get(0).unwrap();
        }

        context.insert("content", &BlogHandler::get_markdown_as_html(&blog_post_ref.content));
        context.insert("post", blog_post_ref);
        context.insert("recent_posts", &recent_posts);

        let html = tera.render("blog.html", &context).expect("Could not render template!");
        Ok(Box::new(warp::reply::html(html)))
    }

    pub fn get_markdown_as_html(content: &String) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(content, options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }
}