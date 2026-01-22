use std::io;
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

/// Настройки проводника
pub struct ExplorerSettings {
    pub show_hidden_files: bool,
    pub show_file_extensions: bool,
    pub open_this_pc: bool,
    pub remove_shortcut_suffix: bool,
}

impl Default for ExplorerSettings {
    fn default() -> Self {
        Self {
            show_hidden_files: false,
            show_file_extensions: false,
            open_this_pc: false,
            remove_shortcut_suffix: false,
        }
    }
}

/// Получает текущие настройки проводника
pub fn get_explorer_settings() -> io::Result<ExplorerSettings> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    let mut settings = ExplorerSettings::default();
    
    // Проверяем настройки из реестра
    if let Ok(explorer_key) = hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced") {
        settings.show_hidden_files = explorer_key.get_value::<u32, _>("Hidden").unwrap_or(0) == 1;
        settings.show_file_extensions = explorer_key.get_value::<u32, _>("HideFileExt").unwrap_or(1) == 0;
    }
    
    Ok(settings)
}

/// Показывает скрытые файлы и папки
pub fn show_hidden_files(enable: bool) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced")?;
    
    key.set_value("Hidden", &(if enable { 1u32 } else { 2u32 }))?;
    
    println!("{} Скрытые файлы и папки {}", 
        if enable { "✅" } else { "❌" }, 
        if enable { "показываются" } else { "скрыты" }
    );
    
    Ok(())
}

/// Показывает расширения файлов
pub fn show_file_extensions(enable: bool) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced")?;
    
    key.set_value("HideFileExt", &(if enable { 0u32 } else { 1u32 }))?;
    
    println!("{} Расширения файлов {}", 
        if enable { "✅" } else { "❌" }, 
        if enable { "показываются" } else { "скрыты" }
    );
    
    Ok(())
}

/// Открывает "Этот ПК" вместо "Главная" в проводнике
pub fn open_this_pc(enable: bool) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced")?;
    
    // LaunchTo: 1 = Этот ПК, 2 = Быстрый доступ
    key.set_value("LaunchTo", &(if enable { 1u32 } else { 2u32 }))?;
    
    println!("{} Проводник открывается в \"{}\"", 
        if enable { "✅" } else { "❌" }, 
        if enable { "Этот ПК" } else { "Быстрый доступ" }
    );
    
    Ok(())
}

/// Убирает окончание "-Ярлык" у новых ярлыков
pub fn remove_shortcut_suffix(enable: bool) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer")?;
    
    if enable {
        key.set_value("link", &"")?; // Пустая строка
    } else {
        let _ = key.delete_value("link"); // Удаляем ключ для значения по умолчанию
    }
    
    println!("{} Окончание \"-Ярлык\" {}", 
        if enable { "✅" } else { "❌" }, 
        if enable { "убрано" } else { "используется" }
    );
    
    Ok(())
}

/// Перезагружает проводник Windows
pub fn restart_explorer() -> io::Result<()> {
    println!("🔄 Перезагрузка проводника...");
    
    // Закрываем процесс explorer.exe
    Command::new("taskkill")
        .args(&["/F", "/IM", "explorer.exe"])
        .output()?;
    
    // Ждем немного
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Запускаем проводник снова
    Command::new("explorer.exe")
        .spawn()?;
    
    println!("✅ Проводник успешно перезапущен.");
    
    Ok(())
}

/// Применяет все настройки проводника
pub fn apply_explorer_settings(settings: &ExplorerSettings) -> io::Result<()> {
    println!("⚙️  Применение настроек проводника...");
    
    show_hidden_files(settings.show_hidden_files)?;
    show_file_extensions(settings.show_file_extensions)?;
    open_this_pc(settings.open_this_pc)?;
    remove_shortcut_suffix(settings.remove_shortcut_suffix)?;
    
    println!("✅ Настройки проводника применены. Для применения изменений перезагрузите проводник.");
    
    Ok(())
}
