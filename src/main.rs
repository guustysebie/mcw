use std::cell::RefCell;

use clap::Command;

use crate::commands::exec_command::McwExecuteCommand;
use crate::commands::gitlog_command::GetLatestCommits;
use crate::commands::version_command::VersionCommand;
use crate::core::{McwContext, McwSubCommand};
use crate::repo_selector::select_repo_menu;

mod commands;
mod core;
mod repo_selector;

fn main() {
    let context = McwContext {
        base_path: "./".to_string(),
        repositories: RefCell::new(Vec::new()),
        verbose: true,
    };
    let mut sub_commands: Vec<Box<dyn McwSubCommand>> = vec![
        Box::new(McwExecuteCommand { command: vec![] }),
        Box::new(GetLatestCommits {}),
        Box::new(VersionCommand {}),
    ];


    let cmd = Command::new("mcw")
        .about("Multi command wizard\nAuthor: Guust Y. Stella A.")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommands(sub_commands.iter().map(|f| f.build_cli_opts()));


    let sub_command_matchers = cmd.clone().get_matches();
    let mut command = None;

    match sub_command_matchers.subcommand() {
        Some((command_exec, sub_matches)) => {
            let cc = sub_commands
                .iter_mut()
                .find(|f| f.command_name() == command_exec);
            if cc.is_some() {
                let unwrapped = cc.unwrap();
                unwrapped.fill_from_arguments(sub_matches);
                command = Some(unwrapped);
            };
        }
        _ => {}
    }


    match command {
        None => {
            println!("No subcommand was found");
        }
        Some(command) => {
            select_repo_menu(&context).expect("TODO: panic message");
            command.execute(&context);
        }
    }
}
