use rusqlite::{Connection, CachedStatement};

pub fn random_start(db: &Connection) -> CachedStatement<'_> {

    return db.prepare_cached("SELECT werd, nextwerd, prevwerd FROM werdz
        WHERE _ROWID_ >= (abs(random()) % (SELECT max(_ROWID_) FROM werdz))
        AND (prevwerd IS NOT NULL
        OR nextwerd IS NOT NULL)
        LIMIT 1;").unwrap();
}

pub fn random_start_like(db: &Connection) -> CachedStatement<'_> {

    return db.prepare_cached("SELECT werd, nextwerd, prevwerd FROM werdz
        WHERE _ROWID_ >= (abs(random()) % (SELECT max(_ROWID_) FROM werdz))
        AND (prevwerd IS NOT NULL
        OR nextwerd IS NOT NULL)
        AND normalizedsaidby = lower(:saidby)
        LIMIT 1;").unwrap();
}

pub fn search_start(db: &Connection) -> CachedStatement<'_> {
    return db.prepare_cached("SELECT werd, nextwerd, prevwerd FROM werdz 
        WHERE rowid IN (
            SELECT rowid FROM werdz
            WHERE werd = :werd AND (prevwerd IS NOT NULL OR nextwerd IS NOT NULL)
            ORDER BY RANDOM()
            LIMIT 1
        );").unwrap();
}

pub fn search_start_like(db: &Connection) -> CachedStatement<'_> {
    return db.prepare_cached("SELECT werd, nextwerd, prevwerd FROM werdz 
        WHERE rowid IN (
            SELECT rowid FROM werdz
            WHERE werd = :werd AND (prevwerd IS NOT NULL OR nextwerd IS NOT NULL)
                AND normalizedsaidby = lower(:saidby)
            ORDER BY RANDOM()
            LIMIT 1
        );").unwrap();
}

pub fn search_next(db: &Connection) -> CachedStatement<'_> {
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

pub fn search_next_like(db: &Connection) -> CachedStatement<'_> {
    return db.prepare_cached("SELECT werd, nextwerd, prevwerd FROM werdz
        WHERE rowid IN (
            SELECT rowid FROM werdz
            WHERE werd = :werd
            AND prevwerd = :prevwerd
            AND normalizedsaidby = lower(:saidby)
            ORDER BY RANDOM()
            LIMIT 1
        )
        LIMIT 1;").unwrap();
}

pub fn search_prev(db: &Connection) -> CachedStatement<'_> {
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

pub fn search_prev_like(db: &Connection) -> CachedStatement<'_> {
    return db.prepare_cached("SELECT werd, nextwerd, prevwerd FROM werdz
        WHERE rowid IN (
            SELECT rowid FROM werdz
            WHERE werd = :werd
            AND nextwerd = :nextwerd
            AND normalizedsaidby = lower(:saidby)
            ORDER BY RANDOM()
            LIMIT 1
        )
        LIMIT 1;").unwrap();
}
