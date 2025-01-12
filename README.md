### CLI Task Tracker 
Project idea taken from [roadmap.sh](https://roadmap.sh/projects/task-tracker). Built with Rust 1.83.0. 

#### Building and running the program 
```
cargo run -- <commands>
```
or 
```
cargo build
target/debug/task-tracker <commands>
```
#### List of commands
- `add <description>` - adds a new task 
- `update <id> <description>` - updates the description of a task with the provided ID 
- `delete <id>` - deletes a task with the provided ID
- `mark-in-progress <id>` - updates task status to "in progress" 
- `mark-done <id>` - updates task status to "done"
- `mark-todo <id>` - updates task status to "todo"
- `list` - lists all tasks 
- `list <status>` - lists all tasks with a given status, which can be one of: `todo`, `done`, `in-progress` 
#### Example 
```
cargo run -- add "Finish the project"
# Output: Successfully added task (ID: 1). 
cargo run -- list 
# Output: 
# ------------
# ID: 1 [todo]
# Task: Finish the project
# Created at: 2025-01-12 14:02:28
# Last Update: -
```
#### Crates used 
- `chrono` - for working with dates and times.
- `serde` and `serde_json` - for serializing Rust structures into json and deserializing and parsing json.  