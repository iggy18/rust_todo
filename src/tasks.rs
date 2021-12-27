use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;

//NOTE the "::" seems to be what rust uses instead of dot notation?


// This will allow the task to serialize and deserialize
#[derive(Debug, Deserialize, Serialize)]
/// this struct is basically like the __init__() of a class in python
pub struct Task {
    // it seems like here is wehre I name the varaibles and the type of the data that they hold
    pub text: String,

    // use[serde to serialize datetime]
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>, 
}


/// this adds a method to the Task struct that allows me to create a new todo
impl Task{
    // this "new" method creates a new task instance the "->" describes what this method will return. a new Task.
    pub fn new(text: String) -> Task {
        // I call the utc::now() method to get the current date and time. created_at is the vatiable, DateTime<Utc> is the type, and then I call a method to put a DT value into the varible.
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}