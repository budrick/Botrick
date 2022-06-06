use rusqlite::{named_params, Connection, CachedStatement};

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
        let mut stmt = get_search_next(s.get_db());
        
        let res = stmt.query_row(named_params!{":werd": self.next.clone(), ":prevwerd": self.werd}, |row| Ok(Foon{
            werd: row.get(0)?,
            next: row.get(1)?,
            prev: row.get(2)?
        }));

        match res {
            Ok(r) => Some(r),
            Err(_e) => None
        }
    }

    // Fetch the previous word record. Returns the word record or None.
    pub fn prev_word(&self, s: &Spork) -> Option<Foon> {
        let mut stmt = get_search_prev(s.get_db());
        
        let res = stmt.query_row(named_params!{":werd": self.prev.clone(), ":nextwerd": self.werd}, |row| Ok(Foon{
            werd: row.get(0)?,
            next: row.get(1)?,
            prev: row.get(2)?
        }));

        match res {
            Ok(r) => Some(r),
            Err(_e) => None
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
        write!(f, "prev: {:?} word: {} next: {:?}", self.prev, self.werd, self.next)
    }
}

// A controller of sorts. Holds a database handle, has methods to fetch starting words.
pub struct Spork {
    db: Connection
}

impl Spork {

    // Constructor
    pub fn new(db: Connection) -> Self {
        Spork {
            db
        }
    }

    // Fetch a random start word record from the database.
    pub fn start(&self) -> Option<Foon> {
        let mut stmt = get_random_start(&self.db);
        let res = stmt.query_row([], |row| Ok(Foon{
            werd: row.get(0)?,
            next: row.get(1)?,
            prev: row.get(2)?
        }));

        match res {
            Ok(x) => Some(x),
            Err(_e) => None
        }
    }

    // Fetch a random instance of the given start word from the database.
    pub fn start_with_word<S: Into<String>>(&self, word: S) -> Option<Foon> {
        let word = word.into();
        let mut stmt = get_search_start(&self.db);
        let res = stmt.query_row(named_params!{":werd": word}, |row| Ok(Foon{
            werd: row.get(0)?,
            next: row.get(1)?,
            prev: row.get(2)?
        }));

        match res {
            Ok(x) => Some(x),
            Err(_e) => None
        }
    }

    // Return an immutable reference to our DB
    pub fn get_db(&self) -> &Connection {
        &self.db
    }
}

// Returns an SQLite DB handle
pub fn getdb() -> Connection {
    // Overly Correctly construct a path to a DB file
    let mut path = std::env::current_dir().unwrap();
    path.push("data");
    path.push("werdz");
    path.set_extension("sqlite");

    Connection::open(path).unwrap()
}

// Returns a cached SQLite statement
fn get_random_start(db: &Connection) -> CachedStatement<'_> {

    return db.prepare_cached("SELECT werd, nextwerd, prevwerd FROM werdz
        WHERE _ROWID_ >= (abs(random()) % (SELECT max(_ROWID_) FROM werdz))
        AND (prevwerd IS NOT NULL
        OR nextwerd IS NOT NULL)
        LIMIT 1;").unwrap();
}

// Returns a cached SQLite statement
fn get_search_start(db: &Connection) -> CachedStatement<'_> {
    return db.prepare_cached("SELECT werd, nextwerd, prevwerd FROM werdz 
        WHERE rowid IN (
            SELECT rowid FROM werdz
            WHERE werd = :werd AND (prevwerd IS NOT NULL OR nextwerd IS NOT NULL)
            ORDER BY RANDOM()
            LIMIT 1
        );").unwrap();
}

// Returns a cached SQLite statement
fn get_search_next(db: &Connection) -> CachedStatement<'_> {
    return db.prepare_cached("SELECT werd, nextwerd, prevwerd FROM werdz
        WHERE rowid IN (
            SELECT rowid FROM werdz
            WHERE werd = :werd
            AND prevwerd = :prevwerd
            ORDER BY RANDOM()
            LIMIT 1
        )
        LIMIT 1;").unwrap();
}

// Returns a cached SQLite statement
fn get_search_prev(db: &Connection) -> CachedStatement<'_> {
    return db.prepare_cached("SELECT werd, nextwerd, prevwerd FROM werdz
        WHERE rowid IN (
            SELECT rowid FROM werdz
            WHERE werd = :werd
            AND nextwerd = :nextwerd
            ORDER BY RANDOM()
            LIMIT 1
        )
        LIMIT 1;").unwrap();
}

// Does what it says. Given a start word and a Spork, do the needful.
pub fn build_words(w: Foon, s: &Spork) -> Vec::<String> {
    // let mut words = Vec::<String>::new();
    let mut words = vec![w.get()];
    let initword = &w;
    let mut prev = initword.prev_word(s);
    let mut next = initword.next_word(s);

    while let Some(ref prevword) = prev {
        match prev {
            Some(ref prevword) => {
                words.insert(0, prevword.get());
                prev = prevword.prev_word(s);
            },
            None =>()
        }
    }

    while let Some(ref nextword) = next {
        match next {
            Some(ref nextword) => {
                words.push(nextword.get());
                next = nextword.next_word(s);
            },
            None => ()
        }
    }

    words
}