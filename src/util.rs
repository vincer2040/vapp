use std::{
    io::{Error, Write},
    process::{Command, ExitStatus},
};

pub fn read_line(prompt: Option<&'static str>) -> std::io::Result<String> {
    match prompt {
        Some(p) => {
            print!("{}", p);
            std::io::stdout().flush()?;
        }
        None => (),
    };
    let mut res = String::new();
    let _ = std::io::stdin().read_line(&mut res)?;
    return Ok(res.trim().to_string());
}

pub fn yn_to_bool(yn: &str) -> Option<bool> {
    match yn {
        "y" => Some(true),
        "n" => Some(false),
        _ => None,
    }
}

pub fn get_git_username() -> Option<String> {
    let mut command = Command::new("git");
    command.arg("config").arg("--list");
    let result = get_command_output(&mut command);
    match result {
        Some(output) => {
            let lines: Vec<&str> = output.split('\n').collect();
            for mut line in lines {
                line = line.trim();
                let mut line_split = line.splitn(2, '=');
                if let Some(key) = line_split.next() {
                    if let Some(value) = line_split.next() {
                        if key == "user.name" {
                            return Some(value.to_string());
                        }
                    }
                }
            }
            None
        }
        None => None,
    }
}

pub fn get_command_output(command: &mut Command) -> Option<String> {
    let result = command.output();
    match result {
        Ok(output) => {
            let exit_code = get_exit_code(Ok(output.status));
            if exit_code == 0 {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let line = stdout.trim();

                return Some(line.to_string());
            }
            return None;
        }
        Err(_) => None,
    }
}

pub fn get_exit_code(exit_status: Result<ExitStatus, Error>) -> i32 {
    match exit_status {
        Ok(code) => {
            if !code.success() {
                match code.code() {
                    Some(value) => value,
                    None => -1,
                }
            } else {
                0
            }
        }
        _ => -1,
    }
}
