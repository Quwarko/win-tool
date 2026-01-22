use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Получает список файлов в директории
pub fn list_files(path: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(name) = entry.file_name().to_str() {
                            files.push(name.to_string());
                        }
                    }
                }
            }
        }
    }
    
    Ok(files)
}

/// Получает список .exe файлов в директории
pub fn list_exe_files(path: &str) -> io::Result<Vec<String>> {
    let files = list_files(path)?;
    Ok(files.into_iter()
        .filter(|f| f.to_lowercase().ends_with(".exe"))
        .collect())
}

/// Копирует файл из source в destination
pub fn copy_file(source: &Path, destination: &Path) -> io::Result<()> {
    fs::copy(source, destination)?;
    Ok(())
}

/// Создает директорию, если она не существует
pub fn create_directory_if_not_exists(path: &str) -> io::Result<()> {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Проверяет существование файла
pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Получает путь к директории AppData\Roaming
pub fn get_appdata_roaming() -> Option<PathBuf> {
    std::env::var("APPDATA").ok().map(PathBuf::from)
}

/// Получает путь к директории AppData\Local
pub fn get_appdata_local() -> Option<PathBuf> {
    std::env::var("LOCALAPPDATA").ok().map(PathBuf::from)
}

/// Получает путь к директории ProgramFiles
pub fn get_program_files() -> Option<PathBuf> {
    std::env::var("ProgramFiles").ok().map(PathBuf::from)
}

/// Копирует файлы конфигурации для приложения
pub fn copy_config_files(app_name: &str, source_dir: &Path, dest_dir: &Path) -> io::Result<()> {
    println!("📋 Копирование конфигурационных файлов для {}...", app_name);
    
    if !source_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Директория с конфигурациями не найдена: {:?}", source_dir)
        ));
    }
    
    create_directory_if_not_exists(dest_dir.to_str().unwrap())?;
    
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let source_file = entry.path();
        
        if source_file.is_file() {
            let file_name = source_file.file_name().unwrap();
            let dest_file = dest_dir.join(file_name);
            
            fs::copy(&source_file, &dest_file)?;
            println!("  ✅ Скопирован: {:?}", file_name);
        }
    }
    
    Ok(())
}
