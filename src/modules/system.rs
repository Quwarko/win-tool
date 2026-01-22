use std::io;
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

/// Системные настройки
pub struct SystemSettings {
    pub disable_sticky_keys: bool,
    pub enable_clipboard: bool,
    pub disable_uac: bool,
    pub disable_smartscreen: bool,
    pub disable_hibernation: bool,
    pub disable_bing_search: bool,
    pub disable_bitlocker: bool,
    pub disable_telemetry: bool,
}

impl Default for SystemSettings {
    fn default() -> Self {
        Self {
            disable_sticky_keys: false,
            enable_clipboard: false,
            disable_uac: false,
            disable_smartscreen: false,
            disable_hibernation: false,
            disable_bing_search: false,
            disable_bitlocker: false,
            disable_telemetry: false,
        }
    }
}

/// Отключает залипание клавиш
pub fn disable_sticky_keys(enable: bool) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey("Control Panel\\Accessibility\\StickyKeys")?;
    
    key.set_value("Flags", &(if enable { "506" } else { "510" }))?;
    
    println!("{} Залипание клавиш {}", 
        if enable { "✅" } else { "❌" }, 
        if enable { "отключено" } else { "включено" }
    );
    
    Ok(())
}

/// Включает буфер обмена
pub fn enable_clipboard(enable: bool) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey("Software\\Microsoft\\Clipboard")?;
    
    key.set_value("EnableClipboardHistory", &(if enable { 1u32 } else { 0u32 }))?;
    
    println!("{} Буфер обмена {}", 
        if enable { "✅" } else { "❌" }, 
        if enable { "включен" } else { "отключен" }
    );
    
    Ok(())
}

/// Отключает контроль учётных записей (UAC)
/// ВНИМАНИЕ: Снижает безопасность системы!
pub fn disable_uac(enable: bool) -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    match hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System", KEY_WRITE) {
        Ok(key) => {
            key.set_value("EnableLUA", &(if enable { 0u32 } else { 1u32 }))?;
            
            println!("{} UAC {}", 
                if enable { "⚠️" } else { "✅" }, 
                if enable { "отключен (требуется перезагрузка)" } else { "включен" }
            );
        }
        Err(_) => {
            println!("❌ Недостаточно прав для изменения UAC. Требуются права администратора.");
        }
    }
    
    Ok(())
}

/// Отключает Smart Screen
pub fn disable_smartscreen(enable: bool) -> io::Result<()> {
    Command::new("powershell")
        .args(&[
            "-Command",
            &format!(
                "Set-ItemProperty -Path 'HKLM:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer' -Name 'SmartScreenEnabled' -Value '{}'",
                if enable { "Off" } else { "Warn" }
            )
        ])
        .output()?;
    
    println!("{} Smart Screen {}", 
        if enable { "✅" } else { "❌" }, 
        if enable { "отключен" } else { "включен" }
    );
    
    Ok(())
}

/// Отключает гибернацию
pub fn disable_hibernation(enable: bool) -> io::Result<()> {
    let command = if enable { "off" } else { "on" };
    
    let status = Command::new("powercfg")
        .args(&["/hibernate", command])
        .status()?;
    
    if status.success() {
        println!("{} Гибернация {}", 
            if enable { "✅" } else { "❌" }, 
            if enable { "отключена" } else { "включена" }
        );
    } else {
        println!("❌ Не удалось изменить настройку гибернации.");
    }
    
    Ok(())
}

/// Отключает онлайн поиск через Bing в Windows Search
pub fn disable_bing_search(enable: bool) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Search")?;
    
    key.set_value("BingSearchEnabled", &(if enable { 0u32 } else { 1u32 }))?;
    key.set_value("CortanaConsent", &(if enable { 0u32 } else { 1u32 }))?;
    
    println!("{} Поиск Bing {}", 
        if enable { "✅" } else { "❌" }, 
        if enable { "отключен" } else { "включен" }
    );
    
    Ok(())
}

/// Отключает автоматическое шифрование BitLocker
pub fn disable_bitlocker_auto(enable: bool) -> io::Result<()> {
    Command::new("powershell")
        .args(&[
            "-Command",
            &format!(
                "Set-ItemProperty -Path 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\BitLocker' -Name 'PreventDeviceEncryption' -Value {}",
                if enable { "1" } else { "0" }
            )
        ])
        .output()?;
    
    println!("{} Авто-шифрование BitLocker {}", 
        if enable { "✅" } else { "❌" }, 
        if enable { "отключено" } else { "включено" }
    );
    
    Ok(())
}

/// Отключает телеметрию Windows
pub fn disable_telemetry(enable: bool) -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    // Отключаем DiagTrack службу
    Command::new("sc")
        .args(&["stop", "DiagTrack"])
        .output()?;
    
    Command::new("sc")
        .args(&["config", "DiagTrack", "start=", if enable { "disabled" } else { "auto" }])
        .output()?;
    
    // Отключаем dmwappushservice
    Command::new("sc")
        .args(&["stop", "dmwappushservice"])
        .output()?;
    
    Command::new("sc")
        .args(&["config", "dmwappushservice", "start=", if enable { "disabled" } else { "auto" }])
        .output()?;
    
    // Изменяем настройки в реестре
    if let Ok(key) = hklm.open_subkey_with_flags("SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection", KEY_WRITE) {
        key.set_value("AllowTelemetry", &(if enable { 0u32 } else { 1u32 }))?;
    }
    
    println!("{} Телеметрия {}", 
        if enable { "✅" } else { "❌" }, 
        if enable { "отключена" } else { "включена" }
    );
    
    Ok(())
}

/// Применяет все системные настройки
pub fn apply_system_settings(settings: &SystemSettings) -> io::Result<()> {
    println!("⚙️  Применение системных настроек...");
    
    disable_sticky_keys(settings.disable_sticky_keys)?;
    enable_clipboard(settings.enable_clipboard)?;
    disable_uac(settings.disable_uac)?;
    disable_smartscreen(settings.disable_smartscreen)?;
    disable_hibernation(settings.disable_hibernation)?;
    disable_bing_search(settings.disable_bing_search)?;
    disable_bitlocker_auto(settings.disable_bitlocker)?;
    disable_telemetry(settings.disable_telemetry)?;
    
    println!("✅ Системные настройки применены.");
    
    Ok(())
}
