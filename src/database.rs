use anyhow::{anyhow, Result};
use rusqlite::Connection;

const QUERIES: &'static [&'static str] = &[
    "CREATE TABLE notification (id INTEGER PRIMARY KEY NOT NULL)",
    "ALTER TABLE notification ADD COLUMN title varchar(255) NOT NULL",
    "ALTER TABLE notification ADD COLUMN message varchar(255) NOT NULL",
    "ALTER TABLE notification ADD COLUMN time varchar(5) NOT NULL",
    "ALTER TABLE notification ADD COLUMN day INTEGER NOT NULL",
];

pub fn create_connection() -> Result<Connection> {
    let mut path = dirs::home_dir().ok_or(anyhow!("Could not find the home directory"))?;
    path.push(".herd");

    if !path.exists() {
        std::fs::create_dir(&path)?;
    }

    let file_name = if cfg!(debug_assertions) {
        "herd.sqlite.dbg"
    } else {
        "herd.sqlite"
    };
    path.push(file_name);

    let connection = Connection::open(&path)?;

    migrate(&connection)?;

    Ok(connection)
}

fn migrate(connection: &Connection) -> Result<()> {
    let current_version = get_current_version(&connection)?;

    for i in current_version..QUERIES.len() {
        connection.execute(QUERIES[i], ())?;
        connection.execute("UPDATE version set v = ?1;", (i + 1,))?;
    }

    Ok(())
}

fn get_current_version(connection: &Connection) -> Result<usize> {
    let mut query = connection
        .prepare("SELECT count(*) FROM sqlite_master WHERE type='table' and name = ?1")?;

    let exists: bool = query.query_row(("version",), |r| {
        let count: usize = r.get(0)?;
        Ok(count > 0)
    })?;

    if !exists {
        connection.execute("CREATE TABLE version (v integer);", ())?;
        connection.execute("INSERT INTO version (v) values (0);", ())?;
    }

    let mut current_version = connection.prepare("SELECT v FROM version")?;
    let current_version: usize = current_version.query_row((), |row| {
        let v: usize = row.get(0)?;
        Ok(v)
    })?;

    Ok(current_version)
}
