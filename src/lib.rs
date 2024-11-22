#![allow(non_snake_case)]

use winapi::um::processthreadsapi::{OpenProcess, GetProcessId};
use winapi::um::processsnapshot::{
    PssCaptureSnapshot, PssFreeSnapshot, PSS_CAPTURE_FLAGS, PSS_CAPTURE_HANDLES, HPSS,
    PSS_CAPTURE_VA_CLONE, PSS_CAPTURE_HANDLE_NAME_INFORMATION, PSS_CAPTURE_HANDLE_BASIC_INFORMATION,
    PSS_CAPTURE_HANDLE_TYPE_SPECIFIC_INFORMATION, PSS_CAPTURE_HANDLE_TRACE, PSS_CAPTURE_THREADS,
    PSS_CAPTURE_THREAD_CONTEXT, PSS_CAPTURE_THREAD_CONTEXT_EXTENDED, PSS_CREATE_BREAKAWAY,
    PSS_CREATE_BREAKAWAY_OPTIONAL, PSS_CREATE_USE_VM_ALLOCATIONS, PSS_CREATE_RELEASE_SECTION,
};
use winapi::um::winnt::{CONTEXT_ALL, HANDLE, PROCESS_ALL_ACCESS};

#[derive(Debug)]
pub struct ProcessSnapshot {
    process_handle: winapi::shared::ntdef::HANDLE,
    snapshot_handle: HPSS,
}

impl ProcessSnapshot {
    pub fn new(pid: u32) -> Result<Self, i32> {
        let process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, pid) };
        if process_handle.is_null() {
            return Err(unsafe { winapi::um::errhandlingapi::GetLastError() as i32 });
        }

        let flags: PSS_CAPTURE_FLAGS = PSS_CAPTURE_VA_CLONE 
            | PSS_CAPTURE_HANDLES 
            | PSS_CAPTURE_HANDLE_NAME_INFORMATION 
            | PSS_CAPTURE_HANDLE_BASIC_INFORMATION 
            | PSS_CAPTURE_HANDLE_TYPE_SPECIFIC_INFORMATION 
            | PSS_CAPTURE_HANDLE_TRACE 
            | PSS_CAPTURE_THREADS 
            | PSS_CAPTURE_THREAD_CONTEXT 
            | PSS_CAPTURE_THREAD_CONTEXT_EXTENDED 
            | PSS_CREATE_BREAKAWAY 
            | PSS_CREATE_BREAKAWAY_OPTIONAL 
            | PSS_CREATE_USE_VM_ALLOCATIONS 
            | PSS_CREATE_RELEASE_SECTION;

        let mut snapshot_handle: HPSS = std::ptr::null_mut();
        let result = unsafe { 
            PssCaptureSnapshot(process_handle, flags, CONTEXT_ALL, &mut snapshot_handle)
        };

        if result != 0 {
            unsafe { winapi::um::handleapi::CloseHandle(process_handle) };
            return Err(result as i32);
        }

        Ok(ProcessSnapshot {
            process_handle,
            snapshot_handle,
        })
    }

    pub fn get_process_id(&self) -> u32 {
        unsafe { GetProcessId(self.process_handle) }
    }

    pub fn is_valid(&self) -> bool {
        self.process_handle != 0 as HANDLE && self.snapshot_handle as HANDLE != 0 as HANDLE
    }

}

impl Drop for ProcessSnapshot {
    fn drop(&mut self) {
        unsafe {
            PssFreeSnapshot(self.process_handle, self.snapshot_handle);
            winapi::um::handleapi::CloseHandle(self.process_handle);
        }
    }
}
