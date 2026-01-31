CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    completed INTEGER NOT NULL DEFAULT 0,
    archived INTEGER NOT NULL DEFAULT 0,
    priority TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    archived_at TEXT,
		workspace_id TEXT,

		FOREIGN KEY(workspace_id) REFERENCES workspaces(id)
);
