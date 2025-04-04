use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Local>,
    pub due_date: Option<DateTime<Local>>,
    pub priority: Priority,
    pub completed: bool,
}

impl Task {
    pub fn new(
        id: usize,
        title: String,
        description: String,
        due_date: Option<DateTime<Local>>,
        priority: Priority,
    ) -> Self {
        Task {
            id,
            title,
            description,
            created_at: Local::now(),
            due_date,
            priority,
            completed: false,
        }
    }

    pub fn mark_as_complete(&mut self) {
        self.completed = true;
    }

    pub fn display(&self) -> String {
        let status = if self.completed { "✅" } else { "❌" };
        let due = match &self.due_date {
            Some(date) => date.format("%Y-%m-%d %H:%M").to_string(),
            None => "No due date".to_string(),
        };

        format!(
            "ID: {}\nTitle: {}\nDescription: {}\nPriority: {}\nDue: {}\nStatus: {}\n",
            self.id, self.title, self.description, self.priority, due, status
        )
    }
}