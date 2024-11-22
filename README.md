# Snapshotting-rs

A Rust library for creating process snapshots on Windows systems. This crate provides a safe Rust wrapper around the Windows process snapshot functionality.

## Features
- Create process snapshots by PID
- Safe handling of Windows snapshot handles with automatic cleanup
- Error handling with descriptive messages

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
snapshotting_rs = "0.1.0"
```

## Usage

### As a Command Line Tool

```bash
snapshotting-rs.exe <PID>
```

Where `<PID>` is the process ID of the target process you want to snapshot.

### As a Library

```rust
use snapshotting_rs::ProcessSnapshot;

fn main() -> Result<(), String> {
    // Create a snapshot of a process with PID 1234
    let snapshot = ProcessSnapshot::new(1234)
        .map_err(|e| format!("Failed to create snapshot: {}", e))?;

    // The snapshot handle is automatically cleaned up when it goes out of scope
    println!("Snapshot handle: {:?}", snapshot);

    Ok(())
}
```

## Error Handling

The library returns `Result` types with descriptive error messages. Common errors include:
- Invalid PID
- Process access denied
- Process not found

