use dialoguer::Select;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // Path to the library database
    let db_path = "/Users/imertkaradayi/Library/Containers/com.apple.iBooksX/Data/Documents/BKLibrary/BKLibrary-1-091020131601.sqlite";

    // Connect to the database
    let conn = Connection::open(db_path)?;

    // Query to get book IDs and titles
    let mut stmt = conn.prepare(
        "
        SELECT ZASSETID AS BookID, ZTITLE AS Title 
        FROM ZBKLIBRARYASSET 
        WHERE ZTITLE IS NOT NULL 
        ORDER BY ZTITLE;
    ",
    )?;

    // Collect results into a vector
    let books: Vec<(String, String)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .filter_map(|res| res.ok())
        .collect();

    // Display book titles to the user and let them select one
    let options: Vec<String> = books
        .iter()
        .map(|(_, title)| title.clone())
        .collect();
    let selection = Select::new()
        .with_prompt("Select a book to view highlights")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    // Print the selected book's title
    println!("You selected: {}", options[selection]);

    Ok(())
}
