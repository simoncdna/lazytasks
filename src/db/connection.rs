use std::fs;

use rusqlite::Connection;

pub struct Db {
    pub connection: Connection,
}

impl Db {
    pub fn new() -> Self {
        let data_dir = dirs::data_dir()
            .expect("Couldn't find your data directory")
            .join("lazytasks");

        if !data_dir.exists() {
            fs::create_dir(&data_dir).expect("Couldn't create your data directory")
        }

        let db_path = data_dir.join("tasks.db");

        let connection = Connection::open(&db_path).expect("Couldn't open database");

        let db = Db { connection };

        db.init_schema().expect("Couldn't init schema");

        db
    }

    fn init_schema(&self) -> Result<(), rusqlite::Error> {
        let schema = include_str!("schema/tasks.sql");

        self.connection.execute_batch(schema)
    }
}
