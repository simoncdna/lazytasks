use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn label(&self) -> &'static str {
        match self {
            models::Priority::High => "P0",
            models::Priority::Medium => "P1",
            models::Priority::Low => "P2",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            models::Priority::High => Color::Red,
            models::Priority::Medium => Color::Yellow,
            models::Priority::Low => Color::Blue,
        }
    }
}
