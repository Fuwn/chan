use rocket_contrib::templates::Template;
use rocket::response::Redirect;

use crate::db::*;

// GET: Index.
#[get("/")]
pub fn index() -> Template {
    Template::render("index", &())
}

#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to("/")
}

// GET: Make a new thread.
#[get("/post")]
pub fn make_post() -> Template {
    Template::render("post", &())
}

// GET: Check out all the threads.
#[get("/threads")]
pub fn threads() -> Template {
    let context = get_threads().unwrap();
    let threads = serde_json::json!(&context);
    Template::render("threads", threads)
}
