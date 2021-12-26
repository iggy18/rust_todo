use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
pub enum Action {
    Add {
        #[structopt()]
        text: String,
    },

    Done {
        #[structopt()]
        postion: usize,
    },

    List, 
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "todo", 
    about = "A simple todo list"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}

