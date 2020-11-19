use rusqlite::{params, Connection, Result, Error as SQLError};

/// The format a valid SQLlite thread entry should have.
#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct Thread {
    pub name: String,
    pub comment: String
}

/// Create a new thread in the SQLite database.
pub fn new_thread(mut thread: Thread) -> Result<()> {
    // Open SQLite database file.
    let db = Connection::open("chan.db")?;

    // If the name field in the POST request is empty, user should be "anonymous".
    if thread.name == "" {
        thread.name = "anonymous".to_owned();
    }

    // Add thread entry to database.
    db.execute(
        "INSERT INTO thread (name, comment) VALUES (?1, ?2)",
        params![thread.name, thread.comment]
    )?;

    Ok(())
}

/// Get all threads from the SQLite databse.
pub fn get_threads() -> Result<Vec<Thread>, SQLError> {
    // Open SQLite database file.
    let db = Connection::open("chan.db")?;

    // Get all entries from the thread table.
    let mut statement = db.prepare("SELECT name, comment FROM thread")?;
    let threads = statement.query_map(params![], |row| {
        Ok(Thread {
            name: row.get(0)?,
            comment: row.get(1)?
        })
    })?;
    threads.collect()
}
