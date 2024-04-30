use mysql::{prelude::Queryable, Error, Pool, PooledConn};

pub fn establish_connection() -> Result<Pool, Error> {
    // Establish a connection to MySQL
    Pool::new("mysql://root:d4HBadNT@212.132.108.197:3306/ranking")
}

pub fn insert_player_name(conn: &mut PooledConn, name: &str) -> Result<(), Error> {
    // Insert data into the database
    let stmt = conn.prep("INSERT INTO player_name (name) VALUES (?)")?;
    conn.exec_drop(stmt, (name,))
}

pub fn insert_games_played(conn: &mut PooledConn, games: &str) -> Result<(), Error> {
    // Insert data into the database
    let stmt = conn.prep("INSERT INTO games_played (games) VALUES (?)")?;
    conn.exec_drop(stmt, (games,))
}

pub fn insert_wins(conn: &mut PooledConn, wins: &str) -> Result<(), Error> {
    // Insert data into the database
    let stmt = conn.prep("INSERT INTO wins (wins) VALUES (?)")?;
    conn.exec_drop(stmt, (wins,))
}

pub fn insert_losses(conn: &mut PooledConn, losses: &str) -> Result<(), Error> {
    // Insert data into the database
    let stmt = conn.prep("INSERT INTO losses (losses) VALUES (?)")?;
    conn.exec_drop(stmt, (losses,))
}

pub fn insert_character(conn: &mut PooledConn, character: &str) -> Result<(), Error> {
    // Insert data into the database
    let stmt = conn.prep("INSERT INTO character (character) VALUES (?)")?;
    conn.exec_drop(stmt, (character,))
}
