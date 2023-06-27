use std::io::Write;
use clap::{ArgMatches, Command};
use colored::Colorize;
use crate::commands::execute_process_in_current_shell;
use crate::core::{McwContext, McwSubCommand};

pub struct GetLatestCommits;

impl McwSubCommand for GetLatestCommits {
    fn execute(&self, context: &McwContext) {
        for repository in context.repositories.borrow().iter() {
            println!("{}", repository.purple());
            let command = vec![
                "git".to_owned(),
                "rev-list".to_owned(),
                "--max-count=5".to_owned(),
                "--no-commit-header".to_owned(),
                "--format=%h %s".to_owned(),
                "HEAD".to_owned(),
            ];
            execute_process_in_current_shell(repository, &command);
            std::io::stdout().flush().expect("TODO: panic message");
        }
    }

    fn build_cli_opts(&self) -> Command {
        Command::new("gitlog")
            .about("Shows the latest commits")
    }

    fn command_name(&self) -> String {
        "gitlog".to_string()
    }

    fn fill_from_arguments(&mut self, _matches: &ArgMatches) {

    }
}