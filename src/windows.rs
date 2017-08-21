use std::process::Command;
use super::FailureReason;

/// Locks Windows system.
/// Returns
///  - FailureReason::CannotExecute if command execution is failed.
pub fn lock_sytem() -> Result<(), FailureReason> {
    let status = Command::new("rundll32.exe")
                         .arg("user32.dll,LockWorkStation")
                         .status()
                         .expect("Failed to execute lock command");

    if !status.success() {
        return Err(FailureReason::CannotExecute);
    }
    Ok(())
}
