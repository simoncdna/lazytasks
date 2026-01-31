use std::error::Error;

use chrono::{DateTime, Utc};
use rusqlite::{Connection, Row};
use uuid::Uuid;

use crate::models::Workspace;

pub struct WorkspaceRepository;

impl WorkspaceRepository {
    pub fn create(connection: &Connection, workspace: &Workspace) -> Result<(), rusqlite::Error> {
        connection.execute(
            "INSERT INTO workspaces (id, title, created_at) VALUES (?1, ?2, ?3)",
            (
                workspace.id.to_string(),
                &workspace.title,
                workspace.created_at.to_rfc3339(),
            ),
        )?;

        Ok(())
    }

    pub fn get_all(connection: &Connection) -> Result<Vec<Workspace>, Box<dyn Error>> {
        let mut stmt = connection.prepare("SELECT * from workspaces")?;
        let mut rows = stmt.query([])?;

        let mut workspaces: Vec<Workspace> = Vec::new();
        while let Some(row) = rows.next()? {
            workspaces.push(Self::parse_row(row)?);
        }

        Ok(workspaces)
    }

    fn parse_row(row: &Row) -> Result<Workspace, Box<dyn Error>> {
        let id: String = row.get("id")?;
        let created_at: String = row.get("created_at")?;
        let updated_at: Option<String> = row.get("updated_at")?;
        let archived_at: Option<String> = row.get("archived_at")?;

        Ok(Workspace {
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

    pub fn update(connection: &Connection, workspace: &Workspace) -> Result<(), rusqlite::Error> {
        connection.execute(
            "UPDATE workspaces SET
                title = ?2,
                archived = ?3,
                updated_at = ?4,
                archived_at = ?5
            WHERE id = ?1",
            (
                workspace.id.to_string(),
                &workspace.title,
                workspace.archived as u32,
                workspace.updated_at.map(|d| d.to_rfc3339()),
                workspace.archived_at.map(|d| d.to_rfc3339()),
            ),
        )?;

        Ok(())
    }

    pub fn delete(connection: &Connection, workspace_id: &Uuid) -> Result<(), rusqlite::Error> {
        let tx = connection.unchecked_transaction()?;

        tx.execute(
            "DELETE FROM tasks WHERE workspace_id = ?1",
            [workspace_id.to_string()],
        )?;
        tx.execute(
            "DELETE FROM workspaces WHERE id = ?1",
            [workspace_id.to_string()],
        )?;

        tx.commit()?;

        Ok(())
    }
}
