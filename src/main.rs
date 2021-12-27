mod cli;
mod tasks;

use anyhow::anyhow;
use structopt::StructOpt;
use std::path::PathBuf;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;


fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-todo.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack the journal file.
    let journal_file = journal_file
    .or_else(find_default_journal_file)
    .ok_or(anyhow!("Can not file todo file."))?;

    // Perform the action.
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;
    Ok(())
}