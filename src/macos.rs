use ::FailureReason;

#[link(name = "login", kind = "framework")]
extern {
    fn SACLockScreenImmediate();
}

/// Locks macOS system.
pub fn lock_sytem() -> Result<(), FailureReason> {
    unsafe {
        SACLockScreenImmediate();
    }
    Ok(())
}
