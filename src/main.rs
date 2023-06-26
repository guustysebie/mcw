mod commands;
mod core;
mod repo_selector;

use std::cell::RefCell;
use std::io::Write;
use clap::{arg, Arg, ArgAction, Command, command};
use colored::Colorize;
use walkdir::{DirEntry, WalkDir};
use crate::commands::{GetLatestCommits, McwExecuteCommand, VersionCommand};
use crate::core::{McwCommand, McwContext};
use crate::repo_selector::select_repo_menu;

const COMMAND_EXEC: &str = "exec";
const COMMAND_GITLOG: &str = "gitlog";
const COMMAND_VERSION: &str = "version";


fn generate_cli() -> Command {
    Command::new("mcw")
        .about("Multi command wizard\nAuthor: Guust Y. Stella A.")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(
            Arg::new("version")
                .action(ArgAction::SetTrue)
                .short('v')
                .long("version")
                .help("Prints version and build info")
        )
        //TODO
        //  .arg(
        //      Arg::new("path")
        //          .short('p')
        //          .long("path")
        //          .value_name("PATH")
        //          .help("Base directory default: ./")
        //  ).arg(
        //  Arg::new("repos")
        //      .short('r')
        //      .long("repo")
        //      .help("Specific repos <,> or < > seperated"))
        .subcommand(
            Command::new(COMMAND_EXEC)
                .about("Executes the desired command")
                .arg(arg!(<COMMAND> "The command to be executed"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new(COMMAND_GITLOG)
            .about("Shows the latest commits"))
}

fn main() {
    let mut context = McwContext {
        base_path: "./".to_string(),
        repositories: RefCell::new(Vec::new()),
        mcw_command: None,
        verbose: true,
    };

    let cli = generate_cli();

    let sub_command_matchers = cli.get_matches();

    let is_version_command = sub_command_matchers.get_one::<bool>("version");
    if is_version_command.is_some() && *is_version_command.unwrap() {
        VersionCommand.execute(&context);
        return;
    }

    match sub_command_matchers.subcommand() {
        Some((COMMAND_EXEC, sub_matches)) => {
            let command = sub_matches.get_one::<String>("COMMAND").unwrap().to_string();
            context.mcw_command = Some(Box::new(McwExecuteCommand { command }))
        }
        Some((COMMAND_GITLOG, _)) => {
            context.mcw_command = Some(Box::new(GetLatestCommits))
        }
        _ => {
            println!("Dude, come on");
            unreachable!();
        }
    }


    match &context.mcw_command {
        None => {
            println!("No command was successfully parsed");
        }
        Some(command) => {
            select_repo_menu(&context).expect("TODO: panic message");
            command.execute(&context);
        }
    }
}
