use std::cell::RefCell;

use clap::{ArgMatches, Command };

pub struct McwContext {
    pub base_path: String,
    pub repositories: RefCell<Vec<String>>,
    pub verbose: bool,
}


pub trait McwSubCommand {
    fn execute(&self, context: &McwContext);
    fn build_cli_opts(&self) -> Command;
    fn command_name(&self) -> String;
    fn fill_from_arguments(&mut self, matches: &ArgMatches);
}