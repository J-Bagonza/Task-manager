use crate::task::{Priority, Task};
use chrono::{DateTime, Local, NaiveDateTime};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// Title of the task
        #[arg(short, long)]
        title: String,
        
        /// Description of the task
        #[arg(short, long)]
        description: String,
        
        /// Due date in format YYYY-MM-DD HH:MM
        #[arg(short, long)]
        due: Option<String>,
        
        /// Priority: low, medium, high
        #[arg(short, long, default_value = "medium")]
        priority: String,
    },
    
    /// List all tasks
    List {
        /// Filter by completion status
        #[arg(short, long)]
        completed: Option<bool>,
        
        /// Filter by priority
        #[arg(short, long)]
        priority: Option<String>,
    },
    
    /// View a specific task
    View {
        /// Task ID
        #[arg(short, long)]
        id: usize,
    },
    
    /// Mark a task as complete
    Complete {
        /// Task ID
        #[arg(short, long)]
        id: usize,
    },
    
    /// Update an existing task
    Update {
        /// Task ID
        #[arg(short, long)]
        id: usize,
        
        /// New title for the task
        #[arg(short, long)]
        title: Option<String>,
        
        /// New description for the task
        #[arg(short, long)]
        description: Option<String>,
        
        /// New due date in format YYYY-MM-DD HH:MM
        #[arg(short, long)]
        due: Option<String>,
        
        /// New priority: low, medium, high
        #[arg(short, long)]
        priority: Option<String>,
    },
    
    /// Delete a task
    Delete {
        /// Task ID
        #[arg(short, long)]
        id: usize,
    },
}

pub fn parse_priority(priority_str: &str) -> Result<Priority, String> {
    match priority_str.to_lowercase().as_str() {
        "low" => Ok(Priority::Low),
        "medium" => Ok(Priority::Medium),
        "high" => Ok(Priority::High),
        _ => Err(format!("Invalid priority: {}. Use low, medium, or high", priority_str)),
    }
}

pub fn parse_due_date(due_date_str: &str) -> Result<DateTime<Local>, String> {
    let naive_date = NaiveDateTime::parse_from_str(due_date_str, "%Y-%m-%d %H:%M")
        .map_err(|e| format!("Invalid date format: {}. Use YYYY-MM-DD HH:MM", e))?;
    
    let local_date = Local.from_local_datetime(&naive_date)
        .single()
        .ok_or("Unable to convert to local timezone")?;
    
    Ok(local_date)
}