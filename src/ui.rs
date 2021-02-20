use rocket_contrib::templates::Template;

use crate::db::*;

/// GET: Index.
#[get("/")]
pub fn index() -> Template {
	Template::render("index", &())
}

#[catch(404)]
pub fn not_found() -> Template {
	Template::render("404", &())
}

/// GET: Make a new thread.
// #[get("/post")]
// pub fn make_post() -> Template {
//     Template::render("post", &())
// }

/// GET: Check out all the threads.
// #[get("/threads")]
// pub fn threads() -> Template {
//     let context = get_all_threads().unwrap();
//     let threads = serde_json::json!(&context);
//     Template::render("threads", threads)
// }

/// GET: Check out all the boards.
#[get("/boards")]
pub fn boards() -> Template {
	let context = get_boards().unwrap();
	let threads = serde_json::json!(&context);
	Template::render("boards", threads)
}

/// GET: Check out all threads within a board.
#[get("/board/<board>")]
pub fn board(board: String) -> Template {
	let boards = serde_json::json!(get_boards().unwrap());

	let mut board_count: i32 = 0;
	for i in boards.as_array().unwrap() {
		if i.get("tag") == Some(&serde_json::json!(board)) {
			board_count += 1;
		}
	}
	if board_count == 0 { return Template::render("404", &()); }

	// Return 404 if the board is not found.
	// This doesn't work, but I kept it because I might try to fix it later.
	// if !boards.as_array().unwrap().contains(&serde_json::json!(board)) {
	//     return Template::render("404", &());
	// }

	let context = get_threads(board).unwrap();
	let threads = serde_json::json!(&context);
	Template::render("threads", threads)
}
