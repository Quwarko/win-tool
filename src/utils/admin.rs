use std::ptr;

#[cfg(target_os = "windows")]
use winapi::um::winnt::TOKEN_ELEVATION;
#[cfg(target_os = "windows")]
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
#[cfg(target_os = "windows")]
use winapi::um::securitybaseapi::GetTokenInformation;
#[cfg(target_os = "windows")]
use winapi::um::winnt::{TokenElevation, HANDLE};

/// Проверяет, запущена ли программа с правами администратора
#[cfg(target_os = "windows")]
pub fn is_admin() -> bool {
    unsafe {
        let mut token_handle: HANDLE = ptr::null_mut();
        let current_process = GetCurrentProcess();

        if OpenProcessToken(current_process, winapi::um::winnt::TOKEN_QUERY, &mut token_handle) == 0 {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut return_length = 0u32;

        let result = GetTokenInformation(
            token_handle,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length,
        );

        winapi::um::handleapi::CloseHandle(token_handle);

        if result == 0 {
            return false;
        }

        elevation.TokenIsElevated != 0
    }
}

#[cfg(not(target_os = "windows"))]
pub fn is_admin() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_check() {
        // Просто проверяем, что функция не паникует
        let _ = is_admin();
    }
}
