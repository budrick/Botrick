use regex::Regex;
use rusqlite::{named_params, Connection};
mod statements;
use anyhow::Result;

// List of blocked initial words
static BLOCKLIST: [&str; 2] = ["!speak", "!talklike"];

// Represents a word record. Current word, and next and prev words.
// Has methods to quickly retrieve the next and previous words from the DB.
#[derive(Debug)]
pub struct Foon {
    prev: Option<String>,
    werd: String,
    next: Option<String>,
}

impl Foon {
    // Fetch the next word record. Returns the word record or None.
    pub fn next_word(&self, s: &Spork) -> Option<Foon> {
        let mut stmt = statements::search_next(s.get_db());

        let res = stmt.query_row(
            named_params! {":werd": self.next.clone(), ":prevwerd": self.werd},
            |row| {
                Ok(Foon {
                    werd: row.get(0)?,
                    next: row.get(1)?,
                    prev: row.get(2)?,
                })
            },
        );

        match res {
            Ok(r) => Some(r),
            Err(_e) => None,
        }
    }

    // Fetch the next word record. Returns the word record or None.
    pub fn next_word_like(&self, s: &Spork, saidby: &str) -> Option<Foon> {
        let mut stmt = statements::search_next_like(s.get_db());

        let res = stmt.query_row(
            named_params! {":werd": self.next.clone(), ":prevwerd": self.werd, ":saidby": saidby},
            |row| {
                Ok(Foon {
                    werd: row.get(0)?,
                    next: row.get(1)?,
                    prev: row.get(2)?,
                })
            },
        );

        match res {
            Ok(r) => Some(r),
            Err(_e) => None,
        }
    }

    // Fetch the previous word record. Returns the word record or None.
    pub fn prev_word(&self, s: &Spork) -> Option<Foon> {
        let mut stmt = statements::search_prev(s.get_db());

        let res = stmt.query_row(
            named_params! {":werd": self.prev.clone(), ":nextwerd": self.werd},
            |row| {
                Ok(Foon {
                    werd: row.get(0)?,
                    next: row.get(1)?,
                    prev: row.get(2)?,
                })
            },
        );

        match res {
            Ok(r) => Some(r),
            Err(_e) => None,
        }
    }

    // Fetch the previous word record. Returns the word record or None.
    pub fn prev_word_like(&self, s: &Spork, saidby: &str) -> Option<Foon> {
        let mut stmt = statements::search_prev_like(s.get_db());

        let res = stmt.query_row(
            named_params! {":werd": self.prev.clone(), ":nextwerd": self.werd, ":saidby": saidby},
            |row| {
                Ok(Foon {
                    werd: row.get(0)?,
                    next: row.get(1)?,
                    prev: row.get(2)?,
                })
            },
        );

        match res {
            Ok(r) => Some(r),
            Err(_e) => None,
        }
    }

    // Get the current word as String
    pub fn get(&self) -> String {
        self.werd.clone()
    }
}

// Implement the Display trait for word records, so we can println! them.
impl std::fmt::Display for Foon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "prev: {:?} word: {} next: {:?}",
            self.prev, self.werd, self.next
        )
    }
}

// pub struct Message {
//     sender: String,
//     words: Vec<String>,
// }

// A controller of sorts. Holds a database handle, has methods to fetch starting words.
#[derive(Debug)]
pub struct Spork {
    db: Connection,
    spacere: Regex,
}

impl Spork {
    // Constructor
    pub fn new(db: Connection) -> Self {
        Spork {
            db,
            spacere: Regex::new(r"\s+").unwrap(),
        }
    }

    // Fetch a random start word record from the database.
    pub fn start(&self) -> Option<Foon> {
        let mut stmt = statements::random_start(&self.db);
        let res = stmt.query_row([], |row| {
            Ok(Foon {
                werd: row.get(0)?,
                next: row.get(1)?,
                prev: row.get(2)?,
            })
        });

        match res {
            Ok(x) => Some(x),
            Err(_e) => None,
        }
    }

    // Fetch a random start word record from the database.
    pub fn start_like(&self, saidby: &str) -> Option<Foon> {
        let mut stmt = statements::random_start_like(&self.db);
        let res = stmt.query_row(named_params! {":saidby": saidby}, |row| {
            Ok(Foon {
                werd: row.get(0)?,
                next: row.get(1)?,
                prev: row.get(2)?,
            })
        });

        match res {
            Ok(x) => Some(x),
            Err(_e) => None,
        }
    }

    // Fetch a random instance of the given start word from the database.
    pub fn start_with_word<S: Into<String>>(&self, word: S) -> Option<Foon> {
        let word = word.into();
        let mut stmt = statements::search_start(&self.db);
        let res = stmt.query_row(named_params! {":werd": word}, |row| {
            Ok(Foon {
                werd: row.get(0)?,
                next: row.get(1)?,
                prev: row.get(2)?,
            })
        });

        match res {
            Ok(x) => Some(x),
            Err(_e) => None,
        }
    }

    // Fetch a random instance of the given start word from the database.
    pub fn start_with_word_like<S: Into<String>>(&self, word: S, saidby: &str) -> Option<Foon> {
        let word = word.into();
        let mut stmt = statements::search_start_like(&self.db);
        let res = stmt.query_row(named_params! {":werd": word, ":saidby": saidby}, |row| {
            Ok(Foon {
                werd: row.get(0)?,
                next: row.get(1)?,
                prev: row.get(2)?,
            })
        });

        match res {
            Ok(x) => Some(x),
            Err(_e) => None,
        }
    }

    // Return an immutable reference to our DB
    pub fn get_db(&self) -> &Connection {
        &self.db
    }

    // Save a log line as a collection of word records
    pub fn log_message(&self, who: &str, what: &str) {
        let mut stmt = statements::save_word(&self.db);
        let words: Vec<&str> = self.spacere.split(what).collect();
        let words_iter = words.iter().enumerate();

        // Single word? Don't even try.
        if words.len() < 2 {
            return;
        }

        // If the first word is in our blocklist, ignore it.
        if BLOCKLIST.contains(&words[0]) {
            return;
        }

        // Otherwise, LET'S LOGGING
        for (i, word) in words_iter {
            let mut prev: Option<&str> = None;
            let mut next: Option<&str> = None;
            if i != 0 {
                prev = Some(words[i - 1]);
            }
            if i != words.len() - 1 {
                next = Some(words[i + 1]);
            }
            let _res = stmt.execute(
                named_params! {":werd": word, ":nextwerd": next, ":prevwerd": prev, ":saidby": who},
            );
        }
    }
}

// Returns an SQLite DB handle
pub fn getdb() -> Result<Connection> {
    // Overly Correctly construct a path to a DB file
    let mut path = std::env::current_dir()?;
    path.push("data");
    path.push("werdz");
    path.set_extension("sqlite");

    let h: Connection = Connection::open(path)?;
    Ok(h)
}

// Does what it says. Given a start word and a Spork, do the needful.
pub fn build_words(w: Foon, s: &Spork) -> Vec<String> {
    // let mut words = Vec::<String>::new();
    let mut words = vec![w.get()];
    let initword = &w;
    let mut prev = initword.prev_word(s);
    let mut next = initword.next_word(s);

    while let Some(ref __) = prev {
        if let Some(ref prevword) = prev {
            words.insert(0, prevword.get());
            prev = prevword.prev_word(s);
        }
    }

    while let Some(ref __) = next {
        if let Some(ref nextword) = next {
            words.push(nextword.get());
            next = nextword.next_word(s);
        }
    }

    words
}

// Does what it says. Given a start word and a Spork, do the needful.
pub fn build_words_like(w: Foon, s: &Spork, saidby: &str) -> Vec<String> {
    // let mut words = Vec::<String>::new();
    let mut words = vec![w.get()];
    let initword = &w;
    let mut prev = initword.prev_word_like(s, saidby);
    let mut next = initword.next_word_like(s, saidby);

    while let Some(ref __) = prev {
        if let Some(ref prevword) = prev {
            words.insert(0, prevword.get());
            prev = prevword.prev_word_like(s, saidby);
        }
    }

    while let Some(ref __) = next {
        if let Some(ref nextword) = next {
            words.push(nextword.get());
            next = nextword.next_word_like(s, saidby);
        }
    }

    words
}
