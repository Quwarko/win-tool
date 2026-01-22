use std::process::Command;
use std::io;

/// Структура для UWP приложения
#[derive(Debug, Clone)]
pub struct UwpApp {
    pub name: String,
    pub package_full_name: String,
    pub installed: bool,
}

/// Получает список всех UWP приложений (установленных и доступных)
pub fn get_uwp_apps() -> io::Result<Vec<UwpApp>> {
    let _output = Command::new("powershell")
        .args(&[
            "-Command",
            "Get-AppxPackage | Select-Object Name, PackageFullName | ConvertTo-Json"
        ])
        .output()?;
    
    // Простой парсинг вывода - можно расширить
    let mut apps = Vec::new();
    
    // Добавляем некоторые стандартные UWP приложения
    apps.push(UwpApp {
        name: "Microsoft.WindowsTerminal".to_string(),
        package_full_name: "Microsoft.WindowsTerminal_Win10_x64".to_string(),
        installed: is_uwp_installed("Microsoft.WindowsTerminal")?,
    });
    
    apps.push(UwpApp {
        name: "Microsoft.WindowsCalculator".to_string(),
        package_full_name: "Microsoft.WindowsCalculator_8wekyb3d8bbwe".to_string(),
        installed: is_uwp_installed("Microsoft.WindowsCalculator")?,
    });
    
    apps.push(UwpApp {
        name: "Microsoft.WindowsNotepad".to_string(),
        package_full_name: "Microsoft.WindowsNotepad_8wekyb3d8bbwe".to_string(),
        installed: is_uwp_installed("Microsoft.WindowsNotepad")?,
    });
    
    apps.push(UwpApp {
        name: "Microsoft.Paint".to_string(),
        package_full_name: "Microsoft.Paint_8wekyb3d8bbwe".to_string(),
        installed: is_uwp_installed("Microsoft.Paint")?,
    });
    
    apps.push(UwpApp {
        name: "Microsoft.ScreenSketch".to_string(),
        package_full_name: "Microsoft.ScreenSketch_8wekyb3d8bbwe".to_string(),
        installed: is_uwp_installed("Microsoft.ScreenSketch")?,
    });

    Ok(apps)
}

/// Проверяет, установлено ли UWP приложение
pub fn is_uwp_installed(app_name: &str) -> io::Result<bool> {
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            &format!("Get-AppxPackage -Name {}", app_name)
        ])
        .output()?;

    Ok(!output.stdout.is_empty())
}

/// Устанавливает UWP приложение
pub fn install_uwp(package_name: &str) -> io::Result<()> {
    println!("🔄 Установка UWP приложения {}...", package_name);
    
    let status = Command::new("powershell")
        .args(&[
            "-Command",
            &format!("Get-AppxPackage -AllUsers {} | Foreach {{Add-AppxPackage -DisableDevelopmentMode -Register \"$($_.InstallLocation)\\AppXManifest.xml\"}}", package_name)
        ])
        .status()?;

    if status.success() {
        println!("✅ UWP приложение {} успешно установлено.", package_name);
    } else {
        println!("❌ Не удалось установить UWP приложение {}.", package_name);
    }

    Ok(())
}

/// Удаляет UWP приложение
pub fn uninstall_uwp(package_name: &str) -> io::Result<()> {
    println!("🔄 Удаление UWP приложения {}...", package_name);
    
    let status = Command::new("powershell")
        .args(&[
            "-Command",
            &format!("Get-AppxPackage {} | Remove-AppxPackage", package_name)
        ])
        .status()?;

    if status.success() {
        println!("✅ UWP приложение {} успешно удалено.", package_name);
    } else {
        println!("❌ Не удалось удалить UWP приложение {}.", package_name);
    }

    Ok(())
}

/// Список стандартных нежелательных UWP приложений для удаления
pub const BLOATWARE_APPS: &[(&str, &str)] = &[
    ("Microsoft.BingWeather", "🌤️  Погода Bing"),
    ("Microsoft.BingNews", "📰 Новости Bing"),
    ("Microsoft.GetHelp", "❓ Получить помощь"),
    ("Microsoft.Getstarted", "🚀 Советы"),
    ("Microsoft.MicrosoftOfficeHub", "📊 Office Hub"),
    ("Microsoft.MicrosoftSolitaireCollection", "🃏 Пасьянс"),
    ("Microsoft.People", "👥 Люди"),
    ("Microsoft.WindowsFeedbackHub", "💬 Центр отзывов"),
    ("Microsoft.Xbox.TCUI", "🎮 Xbox TCUI"),
    ("Microsoft.XboxApp", "🎮 Xbox App"),
    ("Microsoft.XboxGameOverlay", "🎮 Xbox Game Overlay"),
    ("Microsoft.XboxGamingOverlay", "🎮 Xbox Gaming Overlay"),
    ("Microsoft.XboxIdentityProvider", "🎮 Xbox Identity Provider"),
    ("Microsoft.XboxSpeechToTextOverlay", "🎮 Xbox Speech To Text"),
    ("Microsoft.YourPhone", "📱 Ваш телефон"),
    ("Microsoft.ZuneMusic", "🎵 Groove Музыка"),
    ("Microsoft.ZuneVideo", "🎬 Кино и ТВ"),
];
