# lock-rs

Locks your system. Shows the login screen the next time you use the computer.

### Binary usage

You can use this as a binary with compiling and adding the binary to your path. Usage is very simple:

```
$ lock
```

### Library usage

You can also use this as a library in your rust program. Firstly, add this dependency to your Cargo.toml:

```toml
[dependencies]
lock = "0.1"
```

And use it like this. Again, usage is very simple:

```rust
extern crate lock;

use lock::{FailureReason, lock};

fn main() {
    let result = lock();

    match result {
        Err(FailureReason::CannotExecute) => {
            // Failed to execute command, do something.
        },
        Err(FailureReason::LinuxCommandNotFound) => {
            // There is no xdg-screensaver, gnome-screensaver or dm-tool for linux,
            // do something else.
        }
        Ok(()) => (),
    }
}
```
