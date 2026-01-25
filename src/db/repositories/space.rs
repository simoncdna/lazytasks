use std::error::Error;

use chrono::{DateTime, Utc};
use rusqlite::{Connection, Row};
use uuid::Uuid;

use crate::models::Space;

pub struct SpaceRepository;

impl SpaceRepository {
    pub fn create(connection: &Connection, space: &Space) -> Result<(), rusqlite::Error> {
        connection.execute(
            "INSERT INTO spaces (id, title, created_at) VALUES (?1, ?2, ?3)",
            (
                space.id.to_string(),
                &space.title,
                space.created_at.to_rfc3339(),
            ),
        )?;

        Ok(())
    }

    pub fn get_all(connection: &Connection) -> Result<Vec<Space>, Box<dyn Error>> {
        let mut stmt = connection.prepare("SELECT * from spaces")?;
        let mut rows = stmt.query([])?;

        let mut spaces: Vec<Space> = Vec::new();
        while let Some(row) = rows.next()? {
            spaces.push(Self::parse_row(row)?);
        }

        Ok(spaces)
    }

    fn parse_row(row: &Row) -> Result<Space, Box<dyn Error>> {
        let id: String = row.get("id")?;
        let created_at: String = row.get("created_at")?;
        let updated_at: Option<String> = row.get("updated_at")?;
        let archived_at: Option<String> = row.get("archived_at")?;

        Ok(Space {
            id: Uuid::parse_str(&id)?,
            title: row.get("title")?,
            archived: row.get::<_, i32>("archived")? != 0,
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
            updated_at: updated_at
                .map(|s| DateTime::parse_from_rfc3339(&s).map(|d| d.with_timezone(&Utc)))
                .transpose()?,
            archived_at: archived_at
                .map(|s| DateTime::parse_from_rfc3339(&s).map(|d| d.with_timezone(&Utc)))
                .transpose()?,
        })
    }
}
