mod cli;
mod storage;
mod task;

use clap::Parser;
use cli::{parse_due_date, parse_priority, Cli, Commands};
use storage::TaskStorage;
use task::Task;

const STORAGE_FILE: &str = "tasks.json";

fn main() {
    let cli = Cli::parse();
    let mut storage = TaskStorage::new(STORAGE_FILE);

    match &cli.command {
        Commands::Add {
            title,
            description,
            due,
            priority,
        } => {
            let due_date = match due {
                Some(date_str) => match parse_due_date(date_str) {
                    Ok(date) => Some(date),
                    Err(e) => {
                        eprintln!("Error parsing due date: {}", e);
                        return;
                    }
                },
                None => None,
            };

            let task_priority = match parse_priority(priority) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            let id = storage.get_next_id();
            let task = Task::new(
                id,
                title.clone(),
                description.clone(),
                due_date,
                task_priority,
            );

            if let Err(e) = storage.add_task(task) {
                eprintln!("Failed to add task: {}", e);
            } else {
                println!("Task added successfully with ID: {}", id);
            }
        }

        Commands::List { completed, priority } => {
            let tasks = storage.get_tasks();
            
            if tasks.is_empty() {
                println!("No tasks found.");
                return;
            }

            let filtered_tasks: Vec<&Task> = tasks
                .iter()
                .filter(|task| {
                    if let Some(completed_filter) = completed {
                        if task.completed != *completed_filter {
                            return false;
                        }
                    }

                    if let Some(priority_filter) = priority {
                        if let Ok(p) = parse_priority(priority_filter) {
                            let p_str = format!("{}", p);
                            let task_p_str = format!("{}", task.priority);
                            if p_str != task_p_str {
                                return false;
                            }
                        }
                    }

                    true
                })
                .collect();

            if filtered_tasks.is_empty() {
                println!("No tasks match the filter criteria.");
                return;
            }

            for task in filtered_tasks {
                println!("{}\n", task.display());
            }
        }

        Commands::View { id } => {
            if let Some(task) = storage.get_task_by_id(*id) {
                println!("{}", task.display());
            } else {
                eprintln!("Task with ID {} not found.", id);
            }
        }

        Commands::Complete { id } => {
            if let Some(task) = storage.get_task_by_id(*id) {
                let mut updated_task = task.clone();
                updated_task.mark_as_complete();

                if let Err(e) = storage.update_task(updated_task) {
                    eprintln!("Failed to mark task as complete: {}", e);
                } else {
                    println!("Task marked as complete!");
                }
            } else {
                eprintln!("Task with ID {} not found.", id);
            }
        }

        Commands::Update {
            id,
            title,
            description,
            due,
            priority,
        } => {
            if let Some(task) = storage.get_task_by_id(*id) {
                let mut updated_task = task.clone();

                if let Some(new_title) = title {
                    updated_task.title = new_title.clone();
                }

                if let Some(new_description) = description {
                    updated_task.description = new_description.clone();
                }

                if let Some(new_due) = due {
                    match parse_due_date(new_due) {
                        Ok(date) => updated_task.due_date = Some(date),
                        Err(e) => {
                            eprintln!("Error parsing due date: {}", e);
                            return;
                        }
                    }
                }

                if let Some(new_priority) = priority {
                    match parse_priority(new_priority) {
                        Ok(p) => updated_task.priority = p,
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            return;
                        }
                    }
                }

                if let Err(e) = storage.update_task(updated_task) {
                    eprintln!("Failed to update task: {}", e);
                } else {
                    println!("Task updated successfully!");
                }
            } else {
                eprintln!("Task with ID {} not found.", id);
            }
        }

        Commands::Delete { id } => {
            if let Err(e) = storage.delete_task(*id) {
                eprintln!("Failed to delete task: {}", e);
            } else {
                println!("Task deleted successfully!");
            }
        }
    }
}
