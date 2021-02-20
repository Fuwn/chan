use rusqlite::{params, Connection, Result, Error as SQLError};

use crate::structures::*;

/// Create a new thread in the SQLite database.
pub fn new_thread(mut thread: Thread) -> Result<()> {
	// Open SQLite database file.
	let db = Connection::open("chan.db")?;

	// If the name field in the POST request is empty, user's name should be "anonymous".
	if thread.name == "" {
		thread.name = "Anonymous".to_owned();
	}

	// Add thread entry to database.
	db.execute(
		"INSERT INTO threads (board, name, comment) VALUES (?1, ?2, ?3)",
		params![thread.board, thread.name, thread.comment]
	)?;

	Ok(())
}

/// Get all threads from the SQLite database.
pub fn _get_all_threads() -> Result<Vec<Thread>, SQLError> {
	// Open SQLite database file.
	let db = Connection::open("chan.db")?;

	// Get all entries from the thread table.
	let mut statement = db.prepare("SELECT board, name, comment FROM threads")?;
	statement.query_map(params![], |row| {
		Ok(Thread {
			board: row.get(0)?,
			name: row.get(1)?,
			comment: row.get(2)?
		})
	})?.collect()
}

/// Get all boards from the SQLite database.
pub fn get_boards() -> Result<Vec<Board>, SQLError> {
	// Open SQLite database file.
	let db = Connection::open("chan.db")?;

	// Get all entries from the thread table.
	let mut statement = db.prepare("SELECT tag, name, nsfw, disabled FROM boards")?;
	statement.query_map(params![], |row| {
		Ok(Board {
			tag: row.get(0)?,
			name: row.get(1)?,
			nsfw: row.get(2)?,
			disabled: row.get(3)?
		})
	})?.collect()
}

/// Get all threads from a specific board within the SQLite database.
pub fn get_threads(board: String) -> Result<Vec<Thread>, SQLError> {
	// Open SQLite database file.
	let db = Connection::open("chan.db")?;

	// Get all entries from the thread table.
	let mut statement = db.prepare("SELECT board, name, comment FROM threads WHERE board = (?)")?;
	statement.query_map(params![board], |row| {
		Ok(Thread {
			board: row.get(0)?,
			name: row.get(1)?,
			comment: row.get(2)?
		})
	})?.collect()
}
