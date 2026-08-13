use rusqlite::Connection;

use crate::infrastructure::errors::InfraError;

pub fn open(path: &str) -> Result<Connection, InfraError> {
    let connection = Connection::open(path).map_err(|e| InfraError::Database(e.to_string()))?;
    init_schema(&connection)?;
    Ok(connection)
}

fn init_schema(connection: &Connection) -> Result<(), InfraError> {
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                email_verified INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )
        .map_err(|e| InfraError::Database(e.to_string()))?;

    Ok(())
}
