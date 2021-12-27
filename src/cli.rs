use std::path::PathBuf;
use structopt::StructOpt;

// this file seems to deal almost entirely with the command line interface
// read more in the documentation of this create... right now it's hard to tell what is rust and what is cate specific.

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

//learn more about this #[] syntax
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

