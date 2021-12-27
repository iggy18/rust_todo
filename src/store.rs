use std::fs::OpenOptions;
use std::io::{BuffReader, Result, Seek, SeekFrom};

// this file seems to deal almost entirely with storing the data in a file for persistence

// these functions all need a pathbuf to work. this is a path to the file where the tasks will be stored.
// the fuction will return a Result<()> which I brought in up top with use std::io::Result;
pub fn add_task(journal_path: PathBuf, task:Task) -> Result<()> {
    // seems like this is where I define crud options/privlaages for the file and the path(open) this one is specific to opening the file.
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // consume files contaents. as a vector? using the deserializer here.
    // I need to learn a lot more about whats going on in this one. variables feel a bit different in rust than other laguages I've learned. 
    let mut tasks: Vec<Task> = match serde_json::from_reader(&file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e),
    };


    // rewind file to beginning?
    file.seek(SeekFrom::Start(0))?;

    // add new task to vector: vector is list/array?
    tasks.push(task);

    // write modified task file back to disk
    // the question mark is used to return errors?
    serde_json::to_writer(file, &tasks)?;

    OK(())
}

pub fn complete_task(journal_path: PathBuf, task_postition:usize) -> Result<()> { ... }

pub fn list_tasks(journal_path: PathBuf) -> Result<()> { ... }

