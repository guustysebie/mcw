use std::io::Write;
use colored::Colorize;
use crate::core::{McwCommand, McwContext};






fn pre_process_command_windows(command: &str) -> String {
    let mut cmd: Vec<&str> = command.trim().split(" ").collect();
    let command_name = cmd.first().unwrap().to_owned();
    ;
    match command_name {
        "git" => {
            cmd.insert(1, "-c color.status=always")
        }
        "ls" => {
            cmd.insert(1, "--color=always")
        }
        _ => {}
    }
    return cmd.join(" ");
}

fn execute_process_and_get_result(source_dir: &str, command: &str) -> String {
    let output = if cfg!(target_os = "windows") {
        let windows_cmd = "cd ".to_owned() + source_dir + "" + "&& " + pre_process_command_windows(command).as_str();
        std::process::Command::new("cmd")
            .args(["/C", &windows_cmd])
            .output()
            .expect("failed to execute process")
    } else {
        todo!();
        // let linux_cmd = "cd ".to_owned() + repository + "&& " + &self.command;
        // std::process::Command::new("sh")
        //     .arg("-c")
        //     .env("LS_COLORS", "rs=0:di=38;5;27:mh=44;38;5;15")
        //     .arg(&linux_cmd)
        //     .output()
        //     .expect("failed to execute process")
    };
  //  println!("{:#?}", output);
    let hello = output.stdout;
    let bing = output.stderr;
    if bing.len() >0 {
        println!("{}",std::str::from_utf8(&bing).unwrap().to_string().red());
    }
    return std::str::from_utf8(&hello).unwrap().to_string();
}



pub struct McwExecuteCommand {
    pub command: String,
}
impl McwCommand for McwExecuteCommand {
    fn execute(&self, context: &McwContext) {
        for repository in context.repositories.borrow().iter() {
            let title = format!("[{:_^width$}]", repository, width = 78);
            println!("{}", title.purple());
            print!("{}", execute_process_and_get_result(repository, &self.command));
            std::io::stdout().flush().expect("TODO: panic message");
        }
    }
}


pub struct GetLatestCommits;

impl McwCommand for GetLatestCommits{
    fn execute(&self, context: &McwContext) {
        for repository in context.repositories.borrow().iter() {
            println!("{}", repository.purple());
            let command = "git rev-list --max-count=5 --no-commit-header --format='%h__REPLACE_ME__%s' HEAD";
            print!("{}", execute_process_and_get_result(repository, command).replace("__REPLACE_ME__","    ").replace("'",""));
            std::io::stdout().flush().expect("TODO: panic message");
        }
    }
}
