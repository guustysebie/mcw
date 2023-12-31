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
    } else if cfg!(target_os = "linux") {
        let command_linux = "cd ".to_string() + source_dir + " && " + command.join(" ").as_str();
        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .stdin(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .arg(command_linux)
            .spawn()
            .expect("failed to execute process");
        child.wait().expect("TODO: panic message");
    };
}

pub fn execute_process_and_get_response(source_dir: &str, command: &Vec<String>) -> String {
    if cfg!(target_os = "windows") {
        let mut command_windows = vec!["/C".to_owned(), "cd".to_owned(), source_dir.to_owned(), "&&".to_owned()];
        command.iter().for_each(|c| command_windows.push(c.to_string()));
        let child = std::process::Command::new("cmd")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(command_windows)
            .output()
            .expect("failed to execute process");
        return String::from_utf8_lossy(&child.stdout).to_string();
    } else if cfg!(target_os = "linux") {
        let mut command_linux = vec![ "cd".to_owned(), source_dir.to_owned(), "&&".to_owned()];
        command.iter().for_each(|c| command_linux.push(c.to_string()));
        let command_linux = "cd ".to_string() + source_dir + " && " + command.join(" ").as_str();
        let child = std::process::Command::new("sh")
            .arg("-c")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg(  command_linux   )
            .output()
            .expect("failed to execute process");
        return String::from_utf8_lossy(&child.stdout).to_string();

    } else {
        todo!("Not implemented for this OS");
    };
}



