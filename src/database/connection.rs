use rusqlite::{Connection, Result};
use crate::config::paths::get_ibooks_paths;

pub fn setup_database_connection() -> Result<Connection> {
    let paths = get_ibooks_paths()
        .expect("Could not find iBooks databases");
    
    let conn = Connection::open(&paths.library_path)?;
    conn.execute(
        &format!("ATTACH '{}' AS highlights;", paths.highlights_path.to_str().unwrap()),
        [],
    )?;
    
    Ok(conn)
} 