use std::io::Result;
use std::path::PathBuf;

// this file seems to deal almost entirely with storing the data in a file for persistence

// these functions all need a pathbuf to work. this is a path to the file where the tasks will be stored.
// the fuction will return a Result<()> which I brought in up top with use std::io::Result;
pub fn add_task(journal_path: PathBuf, task:Task) -> Result<()> { ... }

pub fn complete_task(journal_path: PathBuf, task_postition:usize) -> Result<()> { ... }

pub fn list_tasks(journal_path: PathBuf) -> Result<()> { ... }

