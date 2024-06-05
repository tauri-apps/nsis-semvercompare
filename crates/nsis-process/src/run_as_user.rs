use core::{mem, ops::Deref, ptr};

use alloc::borrow::ToOwned;
use nsis_plugin_api::{encode_utf16, nsis_fn, popstr, push, ONE, ZERO};
use windows_sys::Win32::{
    Foundation::{
        CloseHandle, GetLastError, LocalFree, ERROR_INSUFFICIENT_BUFFER, FALSE, HANDLE, HLOCAL,
    },
    System::{
        Memory::{LocalAlloc, LMEM_FIXED},
        Threading::{
            CreateProcessW, InitializeProcThreadAttributeList, OpenProcess,
            UpdateProcThreadAttribute, CREATE_NEW_PROCESS_GROUP, CREATE_UNICODE_ENVIRONMENT,
            EXTENDED_STARTUPINFO_PRESENT, PROCESS_CREATE_PROCESS, PROCESS_INFORMATION,
            PROC_THREAD_ATTRIBUTE_PARENT_PROCESS, STARTUPINFOEXW, STARTUPINFOW,
        },
    },
    UI::WindowsAndMessaging::{GetShellWindow, GetWindowThreadProcessId},
};

/// Run command as unelevated user
///
/// Needs 2 strings on the stack
/// $1: command
/// $2: arguments
#[nsis_fn]
fn RunAsUser() -> Result<(), Error> {
    let command = popstr()?;
    let arguments = popstr()?;
    if run_as_user(&command, &arguments) {
        push(ZERO)
    } else {
        push(ONE)
    }
}

/// Return true if success
unsafe fn run_as_user(command: &str, arguments: &str) -> bool {
    let hwnd = OwnedHandle::new(GetShellWindow());
    if *hwnd == 0 {
        return false;
    }
    let mut proccess_id = 0;
    if GetWindowThreadProcessId(*hwnd, &mut proccess_id) != 0 {
        return false;
    }
    let handle = OwnedHandle::new(OpenProcess(PROCESS_CREATE_PROCESS, FALSE, proccess_id));
    if *handle == 0 {
        return false;
    }
    let mut size = 0;
    if !(InitializeProcThreadAttributeList(ptr::null_mut(), 1, 0, &mut size) == FALSE
        && GetLastError() == ERROR_INSUFFICIENT_BUFFER)
    {
        return false;
    }
    let Some(attribute_list) = OwnedLocalMemory::new(size) else {
        return false;
    };
    if InitializeProcThreadAttributeList(*attribute_list, 1, 0, &mut size) == FALSE {
        return false;
    }
    if UpdateProcThreadAttribute(
        *attribute_list,
        0,
        PROC_THREAD_ATTRIBUTE_PARENT_PROCESS as _,
        &*handle as *const _ as _,
        mem::size_of::<HANDLE>(),
        ptr::null_mut(),
        ptr::null(),
    ) == FALSE
    {
        return false;
    }
    let startup_info = STARTUPINFOEXW {
        StartupInfo: STARTUPINFOW {
            cb: mem::size_of::<STARTUPINFOEXW>() as _,
            ..mem::zeroed()
        },
        lpAttributeList: *attribute_list,
    };
    let process_info = PROCESS_INFORMATION { ..mem::zeroed() };
    let mut command_line = command.to_owned();
    command_line.push_str(" ");
    command_line.push_str(&arguments);
    if CreateProcessW(
        encode_utf16(&command).as_ptr(),
        encode_utf16(&command_line).as_mut_ptr(),
        ptr::null(),
        ptr::null(),
        FALSE,
        CREATE_UNICODE_ENVIRONMENT | CREATE_NEW_PROCESS_GROUP | EXTENDED_STARTUPINFO_PRESENT,
        ptr::null(),
        ptr::null(),
        &startup_info as *const _ as _,
        &process_info as *const _ as _,
    ) != 0
    {
        CloseHandle(process_info.hProcess);
        CloseHandle(process_info.hThread);
    }
    true
}

struct OwnedHandle(HANDLE);

impl OwnedHandle {
    fn new(handle: HANDLE) -> Self {
        Self(handle)
    }
}

impl Drop for OwnedHandle {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}

impl Deref for OwnedHandle {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct OwnedLocalMemory(HLOCAL);

impl OwnedLocalMemory {
    fn new(size: usize) -> Option<Self> {
        let hlocal = unsafe { LocalAlloc(LMEM_FIXED, size) };
        if hlocal != ptr::null_mut() {
            Some(Self(hlocal))
        } else {
            None
        }
    }
}

impl Drop for OwnedLocalMemory {
    fn drop(&mut self) {
        unsafe { LocalFree(self.0) };
    }
}

impl Deref for OwnedLocalMemory {
    type Target = HLOCAL;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_cmd() {
        unsafe { run_as_user("cmd", "/c pause") };
    }
}
