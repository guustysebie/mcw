
use clap::{ArgMatches, Command};

use crate::core::{McwContext, McwSubCommand};

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
