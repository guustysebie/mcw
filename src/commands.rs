use std::io::{Stdin, Write};
use std::process::Stdio;
use colored::Colorize;
use crate::core::{McwCommand, McwContext};

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

impl McwCommand for McwExecuteCommand {
    fn execute(&self, context: &McwContext) {
        for repository in context.repositories.borrow().iter() {
            let title = format!("[{:_^width$}]", repository, width = 78);
            println!("{}", title.purple());
            execute_process_in_current_shell(repository, &self.command);
            std::io::stdout().flush().expect("TODO: panic message");
        }
    }
}


pub struct GetLatestCommits;

impl McwCommand for GetLatestCommits {
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
}


pub struct VersionCommand;

impl McwCommand for VersionCommand {
    fn execute(&self, context: &McwContext) {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("MyProgram v{}", VERSION);
    }
}