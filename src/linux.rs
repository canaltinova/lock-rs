use std::process::Command;
use super::FailureReason;

// Linux command list. It's in (command, argument) form.
static COMMANDS: &'static [(&str, &str)] = &[
    ("xdg-screensaver", "lock"),
    ("gnome-screensaver-command", "--lock"),
    ("dm-tool", "lock")
];


/// Checks for existing command and returns if there is one.
fn existing_command() -> Option<(&'static str, &'static str)> {
    COMMANDS.iter().find(|command| {
        Command::new("which")
                .arg(command.0)
                .output()
                .expect("Failed to execute command")
                .status.success()
    }).map(|x| *x)
}


/// Locks Linux system.
/// Returns
///  - FailureReason::LinuxCommandNotFound if there is no command to lock,
///  - FailureReason::CannotExecute if command execution is failed.
pub fn lock_sytem() -> Result<(), FailureReason> {
    let command = match existing_command() {
        Some(command) => command,
        None => return Err(FailureReason::LinuxCommandNotFound),
    };

    let status = Command::new(command.0)
                         .arg(command.1)
                         .status()
                         .expect("Failed to execute lock command");

    if !status.success() {
        return Err(FailureReason::CannotExecute);
    }

    Ok(())
}
