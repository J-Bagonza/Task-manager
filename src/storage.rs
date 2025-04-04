use crate::task::Task;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

pub struct TaskStorage {
    file_path: String,
    tasks: Vec<Task>,
}

impl TaskStorage {
    pub fn new(file_path: &str) -> Self {
        let tasks = Self::load_tasks(file_path).unwrap_or_else(|_| Vec::new());
        
        TaskStorage {
            file_path: file_path.to_string(),
            tasks,
        }
    }

    fn load_tasks(file_path: &str) -> io::Result<Vec<Task>> {
        let path = Path::new(file_path);
        
        // Create the file if it doesn't exist
        if !path.exists() {
            let dir = path.parent().unwrap_or(Path::new(""));
            if !dir.exists() {
                fs::create_dir_all(dir)?;
            }
            File::create(path)?;
            return Ok(Vec::new());
        }
        
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        if contents.trim().is_empty() {
            return Ok(Vec::new());
        }
        
        let tasks: Vec<Task> = serde_json::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        Ok(tasks)
    }

    fn save_tasks(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.tasks)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        let mut file = File::create(&self.file_path)?;
        file.write_all(json.as_bytes())?;
        
        Ok(())
    }

    pub fn add_task(&mut self, task: Task) -> io::Result<()> {
        self.tasks.push(task);
        self.save_tasks()
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn get_task_by_id(&self, id: usize) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn update_task(&mut self, updated_task: Task) -> io::Result<()> {
        if let Some(index) = self.tasks.iter().position(|task| task.id == updated_task.id) {
            self.tasks[index] = updated_task;
            self.save_tasks()?;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Task not found"))
        }
    }

    pub fn delete_task(&mut self, id: usize) -> io::Result<()> {
        if let Some(index) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(index);
            self.save_tasks()?;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Task not fond"))
        }
    }

    pub fn get_next_id(&self) -> usize {
        self.tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1
    }
}