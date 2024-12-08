use crate::models::{Book, Highlight};
use rusqlite::{Connection, Result};

pub fn fetch_books(conn: &Connection) -> Result<Vec<Book>> {
    let mut stmt = conn.prepare(
        "
        SELECT 
            lib.ZASSETID AS BookID,
            lib.ZTITLE AS Title,
            lib.ZAUTHOR AS Author,
            lib.ZCONTENTTYPE AS ContentType
        FROM 
            ZBKLIBRARYASSET lib
        JOIN 
            highlights.ZAEANNOTATION anno
        ON 
            lib.ZASSETID = anno.ZANNOTATIONASSETID
        WHERE 
            lib.ZTITLE IS NOT NULL
            AND lib.ZCONTENTTYPE != 3  -- Exclude PDFs
        GROUP BY 
            lib.ZASSETID, lib.ZTITLE
        ORDER BY 
            MAX(anno.ZANNOTATIONCREATIONDATE) DESC;
    ",
    )?;

    let books = stmt
        .query_map([], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                content_type: row
                    .get::<_, Option<i32>>(3)?
                    .map(Book::get_content_type_string),
            })
        })?
        .filter_map(|res| res.ok())
        .collect();

    Ok(books)
}

pub fn fetch_highlights(conn: &Connection, book_id: &str) -> Result<Vec<Highlight>> {
    let mut stmt = conn.prepare("
        SELECT 
            Z_PK AS AnnotationID,
            COALESCE(ZANNOTATIONSELECTEDTEXT, ZANNOTATIONREPRESENTATIVETEXT, '[No Text Available]') AS HighlightText,
            datetime(ZANNOTATIONCREATIONDATE + strftime('%s', '2001-01-01'), 'unixepoch') AS CreationDate,
            datetime(ZANNOTATIONMODIFICATIONDATE + strftime('%s', '2001-01-01'), 'unixepoch') AS ModificationDate,
            ZANNOTATIONSTYLE AS Style,
            ZANNOTATIONNOTE AS Note
        FROM 
            highlights.ZAEANNOTATION
        WHERE 
            ZANNOTATIONASSETID = ?1
            AND ZANNOTATIONDELETED = 0
            AND (ZANNOTATIONSELECTEDTEXT IS NOT NULL OR ZANNOTATIONREPRESENTATIVETEXT IS NOT NULL)
        ORDER BY 
            ZPLLOCATIONRANGESTART;
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


