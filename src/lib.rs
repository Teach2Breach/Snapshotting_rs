#![allow(non_snake_case)]

use winapi::um::processthreadsapi::GetProcessId;
use winapi::um::processsnapshot::{
    PssCaptureSnapshot, PssFreeSnapshot, PSS_CAPTURE_FLAGS, HPSS,
    PSS_CAPTURE_VA_CLONE};
use winapi::um::winnt::{CONTEXT_ALL, HANDLE};
use winapi::um::processsnapshot::PSS_CAPTURE_VA_SPACE;
use winapi::um::processsnapshot::PSS_CAPTURE_VA_SPACE_SECTION_INFORMATION;

#[derive(Debug)]
pub struct ProcessSnapshot {
    pub process_handle: winapi::shared::ntdef::HANDLE,
    pub snapshot_handle: HPSS,
}

impl ProcessSnapshot {
    pub fn new(process_handle: HANDLE) -> Result<Self, i32> {
        if process_handle.is_null() {
            return Err(unsafe { winapi::um::errhandlingapi::GetLastError() as i32 });
        }

    let flags: PSS_CAPTURE_FLAGS = PSS_CAPTURE_VA_CLONE | PSS_CAPTURE_VA_SPACE | PSS_CAPTURE_VA_SPACE_SECTION_INFORMATION;

        let mut snapshot_handle: HPSS = std::ptr::null_mut();
        let result = unsafe { 
            PssCaptureSnapshot(process_handle, flags, CONTEXT_ALL, &mut snapshot_handle)
        };

        if result != 0 {
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
        }
    }
}
