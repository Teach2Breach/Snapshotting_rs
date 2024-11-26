use Snapshotting_rs::ProcessSnapshot;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::{HANDLE, PROCESS_ALL_ACCESS};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let pid = match parse_arguments(&args) {
        Ok(pid) => pid,
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    };

    println!("Attempting to capture snapshot for PID: {}", pid);
    
    // Open process handle with OpenProcess
    let process_handle = unsafe {
        OpenProcess(PROCESS_ALL_ACCESS, false.into(), pid)
    };
    
    match capture_process_snapshot(process_handle) {
        Ok(_) => println!("Main function ending - snapshot should be freed here"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn parse_arguments(args: &[String]) -> Result<u32, String> {
    if args.len() < 2 {
        let prog_name = args[0].rsplit('\\').next().unwrap_or(&args[0]);
        return Err(format!(
            "Usage: {} <PID>\n    PID must be a positive number",
            prog_name
        ));
    }

    let pid = args[1].parse()
        .map_err(|_| "PID must be a valid number".to_string())?;
    
    if pid == 0 {
        return Err("PID must be greater than 0".to_string());
    }

    Ok(pid)
}

fn capture_process_snapshot(handle: HANDLE) -> Result<(), String> {
    println!("Capturing process...");
    match ProcessSnapshot::new(handle) {
        Ok(snap) => {
            println!("Process snapshot completed successfully");
            println!("Snapshot handle: {:?}", snap);
            println!("Snapshot will be automatically freed when it goes out of scope");
            Ok(())
        },
        Err(e) => Err(format!("Error capturing process snapshot: {}", e))
    }
}