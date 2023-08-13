use color_eyre::eyre::Result;
use std::env;

#[derive(Debug)]
pub struct Foon {
    prev: Option<String>,
    werd: String,
    next: Option<String>,
}

fn main() -> Result<()> {
    // Spin up the database, and a Spork to use it.
    let db = sporker::getdb()?;

    let mut stmt = getaword(&db);

    let res = stmt.query_row([], |row| {
        Ok(Foon {
            werd: row.get(0)?,
            next: row.get(1)?,
            prev: row.get(2)?,
        })
    });

    println!("{:?}", res);

    Ok(())
}

// // Fetch the next word record. Returns the word record or None.
// pub fn next_word(&self, s: &sporker::Spork) -> Option<sporker::Foon> {
//     let mut stmt = sporker::statements::search_next(s.get_db());

//     let res = stmt.query_row(
//         rusqlite::named_params! {":werd": self.next.clone(), ":prevwerd": self.werd},
//         |row| {
//             Ok(Foon {
//                 werd: row.get(0)?,
//                 next: row.get(1)?,
//                 prev: row.get(2)?,
//             })
//         },
//     );

//     match res {
//         Ok(r) => Some(r),
//         Err(_e) => None,
//     }
// }

pub fn getaword(db: &rusqlite::Connection) -> rusqlite::CachedStatement<'_> {
    return db
        .prepare_cached(
            "
            SELECT werd, NULLIF(nextwerd, ''), NULLIF(prevwerd, '') FROM werdz
        WHERE rowid IN (
            SELECT rowid FROM werdz
            WHERE prevwerd = '' OR nextwerd = ''
            ORDER BY RANDOM()
            LIMIT 1
        );
        ",
        )
        .unwrap();
}
