use std::io::{stdout, Write};
use dialoguer::{theme::ColorfulTheme, Select};
use rusqlite::{Connection, Result};
use std::path::PathBuf;

/// Represents a book with its ID and title
struct Book {
    id: String,
    title: String,
}

/// Represents a highlight with its text and creation date
struct Highlight {
    text: String,
    date: String,
}

/// Gets the paths to iBooks databases
fn get_ibooks_paths() -> Option<(PathBuf, PathBuf)> {
    let home = dirs::home_dir()?;
    let container_path = home.join("Library/Containers/com.apple.iBooksX/Data/Documents");
    
    let library_path = container_path
        .join("BKLibrary")
        .join("BKLibrary-1-091020131601.sqlite");
    
    let highlights_path = container_path
        .join("AEAnnotation")
        .join("AEAnnotation_v10312011_1727_local.sqlite");

    Some((library_path, highlights_path))
}

/// Establishes database connections and returns the connection object
fn setup_database_connection() -> Result<Connection> {
    let (library_path, highlights_path) = get_ibooks_paths()
        .expect("Could not find iBooks databases");
    
    let conn = Connection::open(library_path)?;
    conn.execute(
        &format!("ATTACH '{}' AS highlights;", highlights_path.to_str().unwrap()),
        [],
    )?;
    
    Ok(conn)
}

/// Fetches all books with highlights, sorted by last highlight date
fn fetch_books(conn: &Connection) -> Result<Vec<Book>> {
    let mut stmt = conn.prepare("
        SELECT 
            lib.ZASSETID AS BookID, 
            lib.ZTITLE AS Title
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
            MAX(anno.ZANNOTATIONCREATIONDATE) DESC;
    ")?;

    let books = stmt
        .query_map([], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
            })
        })?
        .filter_map(|res| res.ok())
        .collect();

    Ok(books)
}

/// Fetches highlights for a specific book
fn fetch_highlights(conn: &Connection, book_id: &str) -> Result<Vec<Highlight>> {
    let mut stmt = conn.prepare("
        SELECT 
            ZANNOTATIONSELECTEDTEXT AS Highlight,
            datetime(ZANNOTATIONCREATIONDATE + strftime('%s', '2001-01-01'), 'unixepoch') AS CreationDate
        FROM 
            highlights.ZAEANNOTATION
        WHERE 
            ZANNOTATIONASSETID = ?1
            AND ZANNOTATIONSELECTEDTEXT IS NOT NULL
        ORDER BY 
            ZPLLOCATIONRANGESTART;
    ")?;

    let highlights = stmt
        .query_map([book_id], |row| {
            Ok(Highlight {
                text: row.get(0)?,
                date: row.get(1)?,
            })
        })?
        .filter_map(|res| res.ok())
        .collect();

    Ok(highlights)
}

/// Clears the terminal screen
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

/// Displays the book selection menu
fn show_book_selection(books: &[Book]) -> Option<usize> {
    let options: Vec<String> = books
        .iter()
        .map(|book| book.title.clone())
        .collect();

    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a book to view highlights")
        .items(&options)
        .default(0)
        .interact()
        .ok()
}

fn main() -> Result<()> {
    // Set up database connection
    let conn = setup_database_connection()?;
    
    // Fetch all books
    let books = fetch_books(&conn)?;
    
    // Clear screen and show book selection
    clear_screen();
    println!("📚 Available Books (sorted by last highlight date):\n");
    
    if let Some(selection) = show_book_selection(&books) {
        clear_screen();
        
        let selected_book = &books[selection];
        println!("\nHighlights for '{}':\n", selected_book.title);
        
        // Fetch and display highlights
        let highlights = fetch_highlights(&conn, &selected_book.id)?;
        for highlight in highlights {
            println!("[{}] {}", highlight.date, highlight.text);
        }
    }

    Ok(())
}