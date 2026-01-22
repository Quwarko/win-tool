use std::process::Command;
use std::io;
use std::path::Path;

/// Список рекомендуемых Winget пакетов
pub const WINGET_PACKAGES: &[(&str, &str, &str)] = &[
    ("Valve.Steam", "latest", "🎮 Платформа цифровой дистрибуции игр"),
    ("Mozilla.Firefox", "latest", "🌐 Браузер с открытым исходным кодом"),
    ("Hiddify.HiddifyNext", "latest", "🔒 VPN-клиент"),
    ("7zip.7zip", "latest", "📦 Архиватор файлов"),
    ("clsid2.mpc-hc", "latest", "🎬 Медиа-плеер"),
    ("Microsoft.VisualStudioCode", "latest", "💻 Редактор кода"),
    ("Notepad++.Notepad++", "latest", "📝 Текстовый редактор"),
    ("Python.Python.3.12", "latest", "🐍 Язык программирования Python"),
    ("Oracle.JavaRuntimeEnvironment", "latest", "☕ Java Runtime Environment"),
    ("OpenJS.NodeJS", "latest", "📗 Node.js JavaScript runtime"),
    ("Git.Git", "latest", "🔧 Система контроля версий"),
    ("JetBrains.JetBrainsMono.NF", "latest", "🔤 Моноширинный шрифт"),
    ("Microsoft.DotNet.Runtime.8", "latest", ".NET Runtime 8"),
    ("Microsoft.VCRedist.2015+.x64", "latest", "Visual C++ 2015-2022 x64"),
    ("Microsoft.VCRedist.2015+.x86", "latest", "Visual C++ 2015-2022 x86"),
];

/// Структура для представления пакета
#[derive(Debug, Clone)]
pub struct Package {
    pub id: String,
    pub version: String,
    pub description: String,
    pub installed: bool,
}

/// Проверяет установлен ли пакет
pub fn is_package_installed(package_id: &str) -> io::Result<bool> {
    let output = Command::new("winget")
        .args(&["list", "--id", package_id])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.contains(package_id))
}

/// Устанавливает пакет через winget
pub fn install_package(id: &str) -> io::Result<()> {
    println!("🔄 Установка пакета {}...", id);
    
    let status = Command::new("winget")
        .args(&["install", "--id", id, "--silent", "--accept-source-agreements", "--accept-package-agreements"])
        .status()?;

    if status.success() {
        println!("✅ Пакет {} успешно установлен.", id);
        
        // Специальная обработка для Firefox
        if id.contains("Firefox") {
            configure_firefox()?;
        }
        
        // Специальная обработка для Hiddify
        if id.contains("Hiddify") {
            configure_hiddify()?;
        }
    } else {
        println!("❌ Не удалось установить пакет {}", id);
    }

    Ok(())
}

/// Удаляет пакет через winget
pub fn uninstall_package(id: &str) -> io::Result<()> {
    println!("🔄 Удаление пакета {}...", id);
    
    let status = Command::new("winget")
        .args(&["uninstall", "--id", id, "--silent"])
        .status()?;

    if status.success() {
        println!("✅ Пакет {} успешно удалён.", id);
    } else {
        println!("❌ Не удалось удалить пакет {}", id);
    }

    Ok(())
}

/// Обновляет все пакеты через winget
pub fn update_all_packages() -> io::Result<()> {
    println!("🔄 Обновление всех пакетов через winget...");
    
    let status = Command::new("winget")
        .args(&["upgrade", "--all", "--silent", "--accept-source-agreements", "--accept-package-agreements"])
        .status()?;

    if status.success() {
        println!("✅ Все пакеты успешно обновлены.");
    } else {
        println!("❌ Не удалось обновить пакеты.");
    }

    Ok(())
}

/// Конфигурирует Firefox после установки
fn configure_firefox() -> io::Result<()> {
    println!("⚙️  Настройка Firefox...");
    
    // Путь к профилю Firefox
    let appdata = std::env::var("APPDATA").unwrap_or_default();
    let firefox_path = Path::new(&appdata).join("Mozilla").join("Firefox").join("Profiles");
    
    if firefox_path.exists() {
        println!("ℹ️  Найдена папка профилей Firefox: {:?}", firefox_path);
        println!("ℹ️  Для применения настроек, скопируйте конфигурационные файлы из data/config_templates/firefox/");
    } else {
        println!("⚠️  Папка профилей Firefox не найдена. Запустите Firefox один раз для создания профиля.");
    }
    
    Ok(())
}

/// Конфигурирует Hiddify после установки
fn configure_hiddify() -> io::Result<()> {
    println!("⚙️  Настройка Hiddify...");
    
    let appdata = std::env::var("APPDATA").unwrap_or_default();
    let hiddify_path = Path::new(&appdata).join("Hiddify").join("hiddify");
    
    if hiddify_path.exists() {
        println!("ℹ️  Найдена папка Hiddify: {:?}", hiddify_path);
        println!("ℹ️  Для применения настроек, замените файлы конфигурации из data/config_templates/hiddify/");
    } else {
        println!("⚠️  Папка Hiddify не найдена.");
    }
    
    Ok(())
}

/// Получает список всех установленных winget пакетов
pub fn get_installed_packages() -> io::Result<Vec<String>> {
    let output = Command::new("winget")
        .args(&["list"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let packages: Vec<String> = stdout
        .lines()
        .skip(2) // Пропускаем заголовки
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                Some(parts[0].to_string())
            } else {
                None
            }
        })
        .collect();

    Ok(packages)
}
