use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

pub struct CmdRunner {
    pub command: std::process::Command,
}

impl CmdRunner {
    pub fn new(command_string: &str, input: Option<&str>) -> CmdRunner {
        let mut command = Command::new("script");

        if let Some(input_str) = input {
            command
                .arg("-qec")
                .arg(command_string.replace("{}", input_str));
        } else {
            command.arg("-qec").arg(command_string);
        }

        command.arg("/dev/null");

        CmdRunner { command }
    }

    pub fn run(&mut self, stdout: &mut impl Write) {
        self.command.stdout(Stdio::piped());
        self.command.stderr(Stdio::piped());

        let mut child = self.command.spawn().expect("failed to spawn command");

        let stdout_pipe = child.stdout.take().unwrap();
        let stderr_pipe = child.stderr.take().unwrap();

        let stdout_reader = BufReader::new(stdout_pipe);
        let stderr_reader = BufReader::new(stderr_pipe);

        let stdout_thread = std::thread::spawn(move || {
            for line in stdout_reader.lines() {
                if let Ok(line) = line {
                    // To-do: Should this be changed to `write!`?
                    print!("{}\r\n", line);
                }
            }
        });

        let stderr_thread = std::thread::spawn(move || {
            for line in stderr_reader.lines() {
                if let Ok(line) = line {
                    print!("{}\r\n", line);
                }
            }
        });

        let status = child.wait().expect("failed to wait for command");

        stdout_thread.join().expect("failed to join stdout thread");
        stderr_thread.join().expect("failed to join stderr thread");

        if status.success() {
            write!(stdout, "Command executed successfully\r\n").unwrap();
        } else {
            write!(
                stdout,
                "Command failed with exit code {}\r\n",
                status.code().unwrap_or(-1)
            )
            .unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_ok() {
        let mut cmd_runner = CmdRunner::new("echo {}", Some("hello"));
        let mut stdout = Vec::new();

        cmd_runner.run(&mut stdout);

        let stdout_str = String::from_utf8(stdout).unwrap();

        assert_eq!(stdout_str.trim(), "hello");
    }

    #[test]
    fn test_execute_status() {
        let mut cmd_runner = CmdRunner::new("exit 1", Some(""));
        let mut stdout = Vec::new();

        cmd_runner.run(&mut stdout);

        let stderr_str = String::from_utf8_lossy(&stdout);

        assert!(stderr_str.contains("exit status: 1"));
    }

    #[test]
    fn test_execute_err() {
        let mut cmd_runner = CmdRunner::new("non-existent-command", Some(""));
        let mut stdout = Vec::new();

        cmd_runner.run(&mut stdout);

        let stderr_str = String::from_utf8_lossy(&stdout);

        assert!(stderr_str.contains("command not found"));
    }
}
