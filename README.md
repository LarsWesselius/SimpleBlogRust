# SimpleBlogRust
SimpleBlog is a small, simple project to show how you'd implement an extremely basic blog system in different programming languages. The repository you're looking at is Rust based. This is also used as a personal exercise as I'm still learning Rust.

## Requirements
The project is intended to be **extremely simple**. The following features should be supported:
- No login required but you can post should you know the password
- The blog page (under 'Rust' in the top navigation bar) should show the latest post or whichever id is provided in the URL
- There should be a block on the blog page that shows previous posts, so you can navigate straight to older posts

Because the project merely poses as an example, several things would have to be addressed if you'd like to continue this project on your own.
- SQLite is being used for simplicity but a more robust solution would probably be better
- Password based posting would have to be replaced by some way for the user to log in, maybe some roles, etc.
- Refactoring, cleanup and perhaps better separation

## Dependencies
- tokio for async runtime
- warp as the web server, probably the most important component
- tera for template rendering
- log, env_logger for logging
- rusqlite for the SQLite database handling
- serde because the `Post` struct needs to be accessible in a tera template and for that it needs to be serializable
- pulldown-cmark for the Markdown rendering

## How does the project work?
As is obvious, it all starts out in `main.rs`. Tera is set up, the database is set up and bootstrapped if necessary. Then warp is set up, with a static file route under `static`. Wasn't able to figure out how to get it working on the root so you don't need /static. Then I've attempted to implement some separation of concern by splitting the respective pages out into separate modules that all have a `mapping()` function. The database and tera instances are passed to each of them. If you look at each of the page handlers it'll become quite clear what is being done.

**Please provide comments if there are things that could be improved. Still learning Rust so any feedback is welcome**
