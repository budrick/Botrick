use rusqlite::{CachedStatement, Connection};

pub fn random_start(db: &Connection) -> CachedStatement<'_> {
    db
        .prepare_cached(
            "
            SELECT werd, NULLIF(nextwerd, ''), NULLIF(prevwerd, '') FROM werdz
        WHERE _ROWID_ >= (abs(random()) % (SELECT max(_ROWID_) FROM werdz))
        AND (prevwerd != ''
        OR nextwerd != '')
        LIMIT 1;
        ",
        )
        .unwrap()
}

pub fn random_start_like(db: &Connection) -> CachedStatement<'_> {
    // SEE https://blog.rodolfocarvalho.net/2012/05/how-to-select-random-rows-from-sqlite.html
    // SEE ALSO: https://gist.github.com/alecco/9976dab8fda8256ed403054ed0a65d7b for a different technique.
    // ORIGINALLY: https://gist.github.com/swayson/84fc86da20db89b56eac
    db
        .prepare_cached(
            "
            SELECT werd, NULLIF(nextwerd, ''), NULLIF(prevwerd, '') FROM werdz
        WHERE (prevwerd != ''
        OR nextwerd != '')
        AND normalizedsaidby = lower(:saidby)
        AND random() % 143 = 0
        LIMIT 1;
        ",
        )
        .unwrap()
}

pub fn search_start(db: &Connection) -> CachedStatement<'_> {
    db
        .prepare_cached(
            "
            SELECT werd, NULLIF(nextwerd, ''), NULLIF(prevwerd, '') FROM werdz
        WHERE rowid IN (
            SELECT rowid FROM werdz
            WHERE werd = :werd AND (prevwerd != '' OR nextwerd != '')
            ORDER BY RANDOM()
            LIMIT 1
        );
        ",
        )
        .unwrap()
}

pub fn search_start_like(db: &Connection) -> CachedStatement<'_> {
    db
        .prepare_cached(
            "
            SELECT werd, NULLIF(nextwerd, ''), NULLIF(prevwerd, '') FROM werdz
        WHERE rowid IN (
            SELECT rowid FROM werdz
            WHERE werd = :werd AND (prevwerd != '' OR nextwerd != '')
                AND normalizedsaidby = lower(:saidby)
            ORDER BY RANDOM()
            LIMIT 1
        );
        ",
        )
        .unwrap()
}

pub fn search_next(db: &Connection) -> CachedStatement<'_> {
    db
        .prepare_cached(
            "
            SELECT werd, NULLIF(nextwerd, ''), NULLIF(prevwerd, '') FROM werdz
        WHERE rowid IN (
            SELECT rowid FROM werdz
            WHERE werd = :werd
            AND prevwerd = :prevwerd
            ORDER BY RANDOM()
            LIMIT 1
        )
        LIMIT 1;
        ",
        )
        .unwrap()
}

pub fn search_next_like(db: &Connection) -> CachedStatement<'_> {
    db
        .prepare_cached(
            "SELECT werd, NULLIF(nextwerd, ''), NULLIF(prevwerd, '') FROM werdz
        WHERE rowid IN (
            SELECT rowid FROM werdz
            WHERE werd = :werd
            AND prevwerd = :prevwerd
            AND normalizedsaidby = lower(:saidby)
            ORDER BY RANDOM()
            LIMIT 1
        )
        LIMIT 1;
        ",
        )
        .unwrap()
}

pub fn search_prev(db: &Connection) -> CachedStatement<'_> {
    db
        .prepare_cached(
            "SELECT werd, NULLIF(nextwerd, ''), NULLIF(prevwerd, '') FROM werdz
        WHERE rowid IN (
            SELECT rowid FROM werdz
            WHERE werd = :werd
            AND nextwerd = :nextwerd
            ORDER BY RANDOM()
            LIMIT 1
        )
        LIMIT 1;
        ",
        )
        .unwrap()
}

pub fn search_prev_like(db: &Connection) -> CachedStatement<'_> {
    db
        .prepare_cached(
            "SELECT werd, NULLIF(nextwerd, ''), NULLIF(prevwerd, '') FROM werdz
        WHERE rowid IN (
            SELECT rowid FROM werdz
            WHERE werd = :werd
            AND nextwerd = :nextwerd
            AND normalizedsaidby = lower(:saidby)
            ORDER BY RANDOM()
            LIMIT 1
        )
        LIMIT 1;
        ",
        )
        .unwrap()
}

pub fn save_word(db: &Connection) -> CachedStatement<'_> {
    
    db
        .prepare_cached(
            "
            INSERT INTO werdz (werd, nextwerd, prevwerd, saidby, normalizedsaidby) VALUES (
        :werd,
        :nextwerd,
        :prevwerd,
        :saidby,
        lower(:saidby)
    )
    ",
        )
        .unwrap()
}
