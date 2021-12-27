mod cli;
use structopt::StructOpt;
mod tasks;
use std::path::PathBuf;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn find_default_journal_file() -> Options<Pathbuff> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-todo.json");
        path
    })
}

fn main() {
    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack the journal file.
    let journal_file = journal_file
    .or_else(find_default_journal_file)
    .expect("Can not file todo file.");

    // Perform the action.
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }
    .expect("Failed to perform action")
}