extern crate lock;

use lock::{FailureReason, lock};

fn main() {
    let result = lock();

    match result {
        Err(FailureReason::CannotExecute) => {
            panic!("Failed to execute lock command!")
        },
        Err(FailureReason::LinuxCommandNotFound) => {
            panic!("No applicable command found. Please consider installing \
                    xdg-screensaver, gnome-screensaver, or dm-tool, and try again.");
        }
        Ok(()) => (),
    }
}
