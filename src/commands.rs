use std::process::Stdio;

pub(crate) mod exec_command;
pub(crate) mod version_command;
pub(crate) mod gitlog_command;

pub fn execute_process_in_current_shell(source_dir: &str, command: &Vec<String>) {
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




