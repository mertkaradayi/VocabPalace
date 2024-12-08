use dialoguer::Select;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // Path to the library database
    let library_db_path = "/Users/imertkaradayi/Library/Containers/com.apple.iBooksX/Data/Documents/BKLibrary/BKLibrary-1-091020131601.sqlite";

    // Path to the highlights database
    let highlights_db_path = "/Users/imertkaradayi/Library/Containers/com.apple.iBooksX/Data/Documents/AEAnnotation/AEAnnotation_v10312011_1727_local.sqlite";

    // Connect to the library database
    let conn = Connection::open(library_db_path)?;

    // Attach the highlights database
    conn.execute(
        &format!("ATTACH '{}' AS highlights;", highlights_db_path),
        [],
    )?;

    // Query to get books ordered by the last highlight date
    let mut stmt = conn.prepare(
        "
        SELECT 
            lib.ZASSETID AS BookID, 
            lib.ZTITLE AS Title, 
            MAX(anno.ZANNOTATIONCREATIONDATE) AS LastHighlightDate
        FROM 
            ZBKLIBRARYASSET lib
        JOIN 
            highlights.ZAEANNOTATION anno
        ON 
            lib.ZASSETID = anno.ZANNOTATIONASSETID
        WHERE 
            lib.ZTITLE IS NOT NULL
        GROUP BY 
            lib.ZASSETID, lib.ZTITLE
        ORDER BY 
            LastHighlightDate DESC;
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
