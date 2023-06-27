use std::io::Write;
use std::process::Stdio;

use clap::{arg, ArgMatches, Command};
use colored::Colorize;

use crate::core::{McwContext, McwSubCommand};

fn execute_process_in_current_shell(source_dir: &str, command: &Vec<String>) {
    if cfg!(target_os = "windows") {
        let mut command_windows = vec!["/C".to_owned(), "cd".to_owned(), source_dir.to_owned(), "&&".to_owned()];
        command.iter().for_each(|c| command_windows.push(c.to_string()));
        let mut child = std::process::Command::new("cmd")
            .stdin(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .args(command_windows)
            .spawn()
            .expect("failed to execute process");
        child.wait().expect("TODO: panic message");
    } else {
        todo!();
    };
}


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
        unreachable!()
    }
}


pub struct VersionCommand;

impl McwSubCommand for VersionCommand {
    fn execute(&self, _context: &McwContext) {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("MyProgram v{}", VERSION);
    }

    fn build_cli_opts(&self) -> Command {
        Command::new("version")
            .about("Shows the version of the program")
    }

    fn command_name(&self) -> String {
        "version".to_string()
    }

    fn fill_from_arguments(&mut self, _matches: &ArgMatches) {
        unreachable!()
    }
}