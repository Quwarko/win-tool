use std::io;
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

/// Настройки персонализации
pub struct PersonalizationSettings {
    pub add_end_task_button: bool,
    pub enable_dark_theme: bool,
    pub enable_verbose_status: bool,
    pub enable_old_context_menu: bool,
    pub remove_context_delay: bool,
}

impl Default for PersonalizationSettings {
    fn default() -> Self {
        Self {
            add_end_task_button: false,
            enable_dark_theme: false,
            enable_verbose_status: false,
            enable_old_context_menu: false,
            remove_context_delay: false,
        }
    }
}

/// Добавляет кнопку "Завершить задачу" на панель задач
pub fn add_end_task_button(enable: bool) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\\TaskbarDeveloperSettings")?;
    
    key.set_value("TaskbarEndTask", &(if enable { 1u32 } else { 0u32 }))?;
    
    println!("{} Кнопка \"Завершить задачу\" на панели задач {}", 
        if enable { "✅" } else { "❌" }, 
        if enable { "добавлена" } else { "убрана" }
    );
    
    Ok(())
}

/// Включает тёмную тему
pub fn enable_dark_theme(enable: bool) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    // Настройки для приложений
    let (key, _) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize")?;
    key.set_value("AppsUseLightTheme", &(if enable { 0u32 } else { 1u32 }))?;
    key.set_value("SystemUsesLightTheme", &(if enable { 0u32 } else { 1u32 }))?;
    
    println!("{} Тёмная тема {}", 
        if enable { "✅" } else { "❌" }, 
        if enable { "включена" } else { "отключена" }
    );
    
    Ok(())
}

/// Включает суперподробные сведения о системе при загрузке
pub fn enable_verbose_status(enable: bool) -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    match hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System", KEY_WRITE) {
        Ok(key) => {
            key.set_value("VerboseStatus", &(if enable { 1u32 } else { 0u32 }))?;
            
            println!("{} Подробные сведения о загрузке {}", 
                if enable { "✅" } else { "❌" }, 
                if enable { "включены" } else { "отключены" }
            );
        }
        Err(_) => {
            println!("❌ Недостаточно прав. Требуются права администратора.");
        }
    }
    
    Ok(())
}

/// Включает старое контекстное меню Windows 10
pub fn enable_old_context_menu(enable: bool) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    if enable {
        let (key, _) = hkcu.create_subkey("Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32")?;
        key.set_value("", &"")?;
        println!("✅ Старое контекстное меню Windows 10 включено");
    } else {
        if let Ok(_) = hkcu.delete_subkey_all("Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}") {
            println!("❌ Новое контекстное меню Windows 11 восстановлено");
        }
    }
    
    Ok(())
}

/// Убирает задержку контекстного меню
pub fn remove_context_delay(enable: bool) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey("Control Panel\\Desktop")?;
    
    key.set_value("MenuShowDelay", &(if enable { "0" } else { "400" }))?;
    
    println!("{} Задержка контекстного меню {}", 
        if enable { "✅" } else { "❌" }, 
        if enable { "убрана" } else { "установлена" }
    );
    
    Ok(())
}

/// Применяет основные настройки персонализации
pub fn apply_basic_personalization(settings: &PersonalizationSettings) -> io::Result<()> {
    println!("⚙️  Применение основных настроек персонализации...");
    
    add_end_task_button(settings.add_end_task_button)?;
    enable_dark_theme(settings.enable_dark_theme)?;
    enable_verbose_status(settings.enable_verbose_status)?;
    
    println!("✅ Основные настройки персонализации применены.");
    
    Ok(())
}

/// Применяет настройки контекстного меню
pub fn apply_context_menu_settings(settings: &PersonalizationSettings) -> io::Result<()> {
    println!("⚙️  Применение настроек контекстного меню...");
    
    enable_old_context_menu(settings.enable_old_context_menu)?;
    remove_context_delay(settings.remove_context_delay)?;
    
    println!("✅ Настройки контекстного меню применены. Перезагрузите проводник для применения изменений.");
    
    Ok(())
}
