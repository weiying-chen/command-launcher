use std::io::{stdin, stdout, Write};
use std::process::Command;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct KeyboardShortcut {
    key: char,
    description: &'static str,
    command: &'static str,
}

impl KeyboardShortcut {
    fn execute_command(&self, stdout: &mut impl Write) {
        let output = Command::new("sh").arg("-c").arg(self.command).status();

        match output {
            Ok(status) => {
                if status.success() {
                    write!(stdout, "Command executed successfully\r\n").unwrap();
                } else {
                    write!(stdout, "Command execution failed\r\n").unwrap();
                }
            }
            Err(e) => {
                write!(stdout, "Error executing command: {:?}\r\n", e).unwrap();
            }
        }
    }
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let keyboard_shortcuts = vec![KeyboardShortcut {
        key: 'g',
        description: "This is a custom command",
        command: "git status",
    }];

    write!(stdout, "Please select a command:\r\n").unwrap();

    for keyboard_shortcut in &keyboard_shortcuts {
        write!(
            stdout,
            "{}  {}\r\n",
            keyboard_shortcut.key, keyboard_shortcut.description
        )
        .unwrap();
    }

    stdout.flush().unwrap();

    let input = stdin().keys();

    for key in input {
        match key.unwrap() {
            Key::Char(k) if keyboard_shortcuts.iter().any(|c| c.key == k) => {
                let keyboard_shortcut = keyboard_shortcuts.iter().find(|c| c.key == k).unwrap();
                keyboard_shortcut.execute_command(&mut stdout);
                break;
            }
            Key::Char(c) => {
                write!(stdout, "You pressed: {}\r\n", c).unwrap();
                stdout.flush().unwrap();
            }
            _ => {}
        }
    }
}
