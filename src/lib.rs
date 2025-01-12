use std::{env, fmt::Display, fs::{File, OpenOptions}, io::{Read, Write}, path::Path, error::Error
};
use chrono::{Local, NaiveDateTime}; 
use serde::{Serialize, Deserialize}; 

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Status {
    Todo, 
    InProgress,
    Done
}

#[derive(Debug)]
enum Command {
    Add(String), // Adding a new task with the given description 
    Update(String, u32), // Updating the description of the task with the given id  
    Delete(u32), // Delete task with given id  
    Mark(Status, u32), // Marking task with the id with the given status 
    List(Option<Status>), // For listing tasks with the given status
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32, 
    description: String,
    status: Status, 
    created_at: NaiveDateTime, 
    updated_at: Option<NaiveDateTime> 
}

impl Task {
    fn new(id: u32, description: String) -> Self {
        Self { id, description, status: Status::Todo, created_at: Local::now().naive_local(), updated_at: None }
    }
    fn update_status(&mut self, status: Status) {
        self.status = status; 
        self.updated_at = Some(Local::now().naive_local());
    }
    fn update_description(&mut self, description: String) {
        self.description = description;
        self.updated_at = Some(Local::now().naive_local());
    }
    fn next_id(tasks: &[Task]) -> u32 {
        tasks.last().map_or(0, |task| task.id) + 1
    }
    fn print(tasks: &[Task]) {
        for task in tasks {
            println!("{}", task); 
        }
    }
}


impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Todo => write!(f, "todo"),
            Status::Done => write!(f, "done"),
            Status::InProgress => write!(f, "in progress")
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let created_at = self.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
        let updated_at = match self.updated_at {
            Some(value) => value.format("%Y-%m-%d %H:%M:%S").to_string(),
            None => "-".to_string()
        };
        write!(f, "------------\nid: {} [{}]\nTask: {}\nCreated at: {}\nLast Update: {}", self.id, self.status, self.description, created_at, updated_at)
    }
}

/// Creates a new JSON file as a database with an empty list, if such a file does not already exist. 
fn create_db(file_path: &str) -> Result<(), std::io::Error> {
    if !Path::new(file_path).exists() {
        let mut file = File::create(file_path)?;
        let _ = file.write_all(b"[]")?;
    }
    Ok(())
}

/// Opens the JSON file and parses the string into a vector of Tasks using serde_json (from_reader can also be used here, but docs say it is usually slower). 
fn read_db(file_path: &str) -> Result<Vec<Task>, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut data = String::new(); 
    file.read_to_string(&mut data)?;
    let tasks: Vec<Task> = serde_json::from_str(&data)?; 
    Ok(tasks)
}

/// Overwrites the contents of the database/JSON file, using the current version of the tasks.  
fn write_db(file_path: &str, tasks: &[Task]) -> Result<(), std::io::Error> {
    let updated_data = serde_json::to_string_pretty(tasks)?;
    let mut file =  OpenOptions::new().write(true).truncate(true).open(file_path)?;
    file.write_all(updated_data.as_bytes())?;
    Ok(())
}

/// Parses args into the desired command (min number of args: 1 + 1, max number of args: 3 + 1)
fn parse_args(args: Vec<String>) -> Result<Command, String> {
    if args.len() < 2 {
        Err("Not enough arguments".to_string())
    } else if args.len() > 4 {
        Err("Too many arguments".to_string())
    } else {
        let cmd = args[1].as_str(); 
        let requires_id = ["update", "delete", "mark-todo", "mark-done", "mark-in-progress"]; 
        if requires_id.contains(&cmd) {
            let id = args
            .get(2)
            .ok_or("Not enough arguments".to_string())?
            .parse::<u32>()
            .map_err(|error| error.to_string())?;
            
            match cmd {
                "update" => {
                    let description = args.get(3).ok_or("Not enough arguments".to_string())?; 
                    return Ok(Command::Update(description.to_string(), id))
                }, 
                "delete" => return Ok(Command::Delete(id)), 
                "mark-todo" => return Ok(Command::Mark(Status::Todo, id)),
                "mark-done" => return Ok(Command::Mark(Status::Done, id)),
                "mark-in-progress" => return Ok(Command::Mark(Status::InProgress, id)),
                _ => return Err("Invalid argument".to_string())
            };
        } else if cmd == "add" {
            let description = args.get(2).ok_or("Not enough arguments".to_string())?;
            return Ok(Command::Add(description.to_string()))
        } else if cmd == "list" {
            let status = args.get(2);
            if let Some(status) = status {
                match status.as_str() {
                    "done" => return Ok(Command::List(Some(Status::Done))),
                    "todo" => return Ok(Command::List(Some(Status::Todo))),
                    "in-progress" => return Ok(Command::List(Some(Status::InProgress))), 
                    _ => return Err("Invalid option".to_string())
                } 
            } else {
                return Ok(Command::List(None))
            }
        } else {
            return Err("Invalid argument".to_string())
        }
    }
}

fn list_tasks(status: Option<Status>, tasks: Vec<Task>) {
    match status {
        None => Task::print(&tasks),
        Some(s) => {
            let filtered_tasks: Vec<Task> = tasks.into_iter().filter(|task| task.status == s).collect(); 
            if filtered_tasks.is_empty() {
                println!("No tasks with the status {}", s)
            } else {
                Task::print(&filtered_tasks)
            }
        }
    }
}

const FILE_PATH: &'static str = "tasks.json"; 

pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    create_db(FILE_PATH)?;
    let mut tasks = read_db(FILE_PATH)?; 
    let parsed_args = parse_args(args)?;
    match parsed_args {
        Command::List(status) => list_tasks(status, tasks), 
        Command::Mark(status, id) => {
            if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                task.update_status(status);
                write_db(FILE_PATH, &tasks)?;
                println!("Successfully updated task {}.", id);
            } else {
                println!("Error: ID not found.")
            }
        }, 
        Command::Delete(id) => {
            if let Some(index) = tasks.iter().position(|task| task.id == id) {
                tasks.remove(index);
                write_db(FILE_PATH, &tasks)?;
                println!("Successfully deleted task {}.", id);
            }
        }, 
        Command::Update(description, id) => {
            if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                task.update_description(description);
                write_db(FILE_PATH, &tasks)?;
                println!("Successfully updated task {}.", id);
            } else {
                println!("Error: ID not found.")
            }
        }, 
        Command::Add(description) => {
            let id = Task::next_id(&tasks); 
            let new_task = Task::new(id, description); 
            tasks.push(new_task); 
            write_db(FILE_PATH, &tasks)?;
            println!("Successfully added task.");
        }
    }
    Ok(()) 
}


// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*; 
    #[test]
    fn file_does_not_exist() {
        let result = read_db("nonexistent.json"); 
        assert!(result.is_err())
    }
    #[test]
    fn create_and_read_empty_file() {
        let result = create_db("test.json"); 
        assert!(result.is_ok()); 
        let tasks = read_db("test.json"); 
        assert!(tasks.is_ok()); 
        assert!(tasks.unwrap().is_empty()); 
        // Clean up
        let result = std::fs::remove_file("test.json"); 
        assert!(result.is_ok())
    }
    // #[test]
    #[test]
    fn add_task() {
        let mut tasks = vec![];
        let description = "New Task".to_string();
        let id = Task::next_id(&tasks);
        let new_task = Task::new(id, description.clone());
        tasks.push(new_task);

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, description);
        assert_eq!(tasks[0].status, Status::Todo);
    }

    #[test]
    fn update_task_description() {
        let mut task = Task::new(1, "Old Description".to_string());
        task.update_description("New Description".to_string());

        assert_eq!(task.description, "New Description");
        assert!(task.updated_at.is_some());
    }

    #[test]
    fn update_task_status() {
        let mut task = Task::new(1, "Task".to_string());
        task.update_status(Status::InProgress);

        assert_eq!(task.status, Status::InProgress);
        assert!(task.updated_at.is_some());
    }

    #[test]
    fn delete_task() {
        let mut tasks = vec![Task::new(1, "Task to be deleted".to_string())];
        tasks.retain(|task| task.id != 1);

        assert!(tasks.is_empty());
    }

    #[test]
    fn list_tasks_by_status() {
        let tasks = vec![
            Task::new(1, "Task 1".to_string()),
            Task::new(2, "Task 2".to_string()),
        ];
        let filtered_tasks: Vec<Task> = tasks.into_iter().filter(|task| task.status == Status::Todo).collect();

        assert_eq!(filtered_tasks.len(), 2);
    }

    #[test]
    fn parse_add_command() {
        let args = vec!["task-tracker".to_string(), "add".to_string(), "New Task".to_string()];
        let command = parse_args(args).unwrap();

        match command {
            Command::Add(description) => assert_eq!(description, "New Task"),
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn parse_update_command() {
        let args = vec!["task-tracker".to_string(), "update".to_string(), "1".to_string(), "Updated Task".to_string()];
        let command = parse_args(args).unwrap();

        match command {
            Command::Update(description, id) => {
                assert_eq!(description, "Updated Task");
                assert_eq!(id, 1);
            },
            _ => panic!("Expected Update command"),
        }
    }

    #[test]
    fn parse_delete_command() {
        let args = vec!["task-tracker".to_string(), "delete".to_string(), "1".to_string()];
        let command = parse_args(args).unwrap();

        match command {
            Command::Delete(id) => assert_eq!(id, 1),
            _ => panic!("Expected Delete command"),
        }
    }

    #[test]
    fn parse_mark_command() {
        let args = vec!["task-tracker".to_string(), "mark-done".to_string(), "1".to_string()];
        let command = parse_args(args).unwrap();

        match command {
            Command::Mark(status, id) => {
                assert_eq!(status, Status::Done);
                assert_eq!(id, 1);
            },
            _ => panic!("Expected Mark command"),
        }
    }

    #[test]
    fn parse_list_command() {
        let args = vec!["task-tracker".to_string(), "list".to_string()];
        let command = parse_args(args).unwrap();

        match command {
            Command::List(status) => assert!(status.is_none()),
            _ => panic!("Expected List command"),
        }
    }
    
}