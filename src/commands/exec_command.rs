use std::io::Write;
use clap::{arg, ArgMatches, Command};
use colored::Colorize;
use crate::commands::execute_process_in_current_shell;
use crate::core::{McwContext, McwSubCommand};

pub struct McwExecuteCommand {
    pub command: Vec<String>,
}

impl McwSubCommand for McwExecuteCommand {
    fn execute(&self, context: &McwContext) {
        for repository in context.repositories.borrow().iter() {
            let title = format!("[{:_^width$}]", repository, width = 78);
            println!("{}", title.purple());
            execute_process_in_current_shell(repository, &self.command);
            std::io::stdout().flush().expect("TODO: panic message");
        }
    }

    fn build_cli_opts(&self) -> Command {
        Command::new("exec")
            .about("Executes the desired command")
            .arg(arg!(<COMMAND> "The command to be executed").num_args(1..))
            .arg_required_else_help(true)
    }

    fn command_name(&self) -> String {
        return "exec".to_string();
    }

    fn fill_from_arguments(&mut self, matches: &ArgMatches) {
        let command: Vec<String> = matches.get_many::<String>("COMMAND")
            .map(|vals| vals.collect::<Vec<_>>())
            .unwrap_or_default()
            .iter()
            .map(|r| r.to_string())
            .collect();
        self.command = command;
    }
}