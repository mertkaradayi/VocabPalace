use crate::models::{Book, Highlight};
use rusqlite::{Connection, Result};

/// Fetches all books that have highlights from the iBooks library
/// Returns a vector of Book structs ordered by most recently highlighted
pub fn fetch_books(conn: &Connection) -> Result<Vec<Book>> {
    let mut stmt = conn.prepare("
        SELECT 
            lib.ZASSETID,      -- Unique identifier for the book
            lib.ZTITLE,        -- Book title
            lib.ZAUTHOR,       -- Book author
            lib.ZCONTENTTYPE   -- Content type (excluding PDFs)
        FROM ZBKLIBRARYASSET lib
        JOIN highlights.ZAEANNOTATION anno ON lib.ZASSETID = anno.ZANNOTATIONASSETID
        WHERE 
            lib.ZTITLE IS NOT NULL
            AND lib.ZCONTENTTYPE != 3  -- Exclude PDFs
        GROUP BY lib.ZASSETID, lib.ZTITLE
        ORDER BY MAX(anno.ZANNOTATIONCREATIONDATE) DESC
    ")?;

    let books = stmt
        .query_map([], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                content_type: row.get::<_, Option<i32>>(3)?
                    .map(Book::get_content_type_string),
            })
        })?
        .filter_map(|res| res.ok())
        .collect();

    Ok(books)
}

/// Fetches all highlights for a specific book
/// Returns a vector of Highlight structs ordered by their position in the book
pub fn fetch_highlights(conn: &Connection, book_id: &str) -> Result<Vec<Highlight>> {
    let mut stmt = conn.prepare("
        SELECT 
            Z_PK,              -- Unique identifier for the highlight
            COALESCE(         -- Get the first non-null value
                ZANNOTATIONSELECTEDTEXT,
                ZANNOTATIONREPRESENTATIVETEXT,
                '[No Text Available]'
            ) as highlight_text,
            -- Convert from Apple's timestamp (seconds since 2001) to SQLite datetime
            datetime(ZANNOTATIONCREATIONDATE + strftime('%s', '2001-01-01'), 'unixepoch'),
            datetime(ZANNOTATIONMODIFICATIONDATE + strftime('%s', '2001-01-01'), 'unixepoch'),
            ZANNOTATIONSTYLE,  -- Highlight color
            ZANNOTATIONNOTE    -- User's note (if any)
        FROM highlights.ZAEANNOTATION
        WHERE 
            ZANNOTATIONASSETID = ?1
            AND ZANNOTATIONDELETED = 0
            AND (
                ZANNOTATIONSELECTEDTEXT IS NOT NULL 
                OR ZANNOTATIONREPRESENTATIVETEXT IS NOT NULL
            )
        ORDER BY ZPLLOCATIONRANGESTART
    ")?;

    let highlights = stmt
        .query_map([book_id], |row| {
            Ok(Highlight {
                id: row.get(0)?,
                text: row.get(1)?,
                date_created: row.get(2)?,
                date_modified: row.get(3)?,
                style: row.get(4)?,
                note: row.get(5)?,
            })
        })?
        .filter_map(|res| res.ok())
        .collect();

    Ok(highlights)
}


