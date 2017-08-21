#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
use macos::lock_sytem;
#[cfg(target_os = "linux")]
use linux::lock_sytem;
#[cfg(target_os = "windows")]
use windows::lock_sytem;

/// FailureReason enum for posible failures.
pub enum FailureReason {
    /// Lock command execution failure.
    CannotExecute,
    /// No applicable command found for Linux.
    /// This crate supports 3 following Linux commands:
    ///  - xdg-screensaver,
    ///  - gnome-screensaver,
    ///  - dm-tool.
    LinuxCommandNotFound,
}

/// Locks the system.
/// Returns
///  - FailureReason::LinuxCommandNotFound if there is no command to lock,
///  - FailureReason::CannotExecute if command execution is failed.
pub fn lock() -> Result<(), FailureReason> {
    lock_sytem()
}
