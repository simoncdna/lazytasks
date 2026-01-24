use std::{fs, process::Command};

use chrono::Local;
use ratatui::DefaultTerminal;

use crate::models::{Priority, Task};

const EDIT_FILE: &str = "/tmp/lazytasks_edit.md";
const DEFAULT_EDITOR: &str = "vim";

pub struct TaskUpdate {
    pub title: String,
    pub description: String,
}

pub fn render_template(task: &Task) -> String {
    let template = include_str!("template.md");
    let description = task.description.clone().unwrap_or_default();
    let created_at = task
        .created_at
        .with_timezone(&Local)
        .format("%d/%m/%Y %H:%M")
        .to_string();
    let updated_at = task
        .updated_at
        .map(|d| d.with_timezone(&Local).format("%d/%m/%Y %H:%M").to_string())
        .unwrap_or_else(|| "-".to_string());
    let archived_at = task
        .archived_at
        .map(|d| d.with_timezone(&Local).format("%d/%m/%Y %H:%M").to_string())
        .unwrap_or_else(|| "-".to_string());

    template
        .replace("{title}", &task.title)
        .replace("{description}", &description)
        .replace("{id}", &task.id.to_string())
        .replace("{completed}", &task.completed.to_string())
        .replace("{created_at}", &created_at)
        .replace("{updated_at}", &updated_at)
        .replace("{archived_at}", &archived_at)
        .replace(
            "{priority}",
            match &task.priority {
                Some(p) => Priority::label(p),
                None => "No priority",
            },
        )
}

pub fn parse_content(content: &str) -> TaskUpdate {
    let mut title = String::new();
    let mut description_lines: Vec<&str> = Vec::new();

    enum Section {
        None,
        Title,
        Description,
    }

    let mut current_section = Section::None;

    for line in content.lines() {
        let trimmed = line.trim();

        // Check for section markers
        if trimmed == "# TITLE" {
            current_section = Section::Title;
            continue;
        }
        if trimmed.starts_with("# DESCRIPTION") {
            current_section = Section::Description;
            continue;
        }

        // Skip comments
        if trimmed.starts_with('#') {
            continue;
        }

        match current_section {
            Section::Title => {
                if title.is_empty() && !trimmed.is_empty() {
                    title = trimmed.to_string();
                }
            }
            Section::Description => {
                description_lines.push(line);
            }
            Section::None => {}
        }
    }

    TaskUpdate {
        title,
        description: description_lines.join("\n").trim().to_string(),
    }
}

pub fn open_in_editor(task: &Task, terminal: &mut DefaultTerminal) -> TaskUpdate {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| DEFAULT_EDITOR.to_string());
    let content = render_template(task);

    fs::write(EDIT_FILE, &content).unwrap();

    ratatui::restore();

    Command::new(&editor).arg(EDIT_FILE).status().unwrap();

    *terminal = ratatui::init();

    let edited_content = fs::read_to_string(EDIT_FILE).unwrap_or_default();
    parse_content(&edited_content)
}
