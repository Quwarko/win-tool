use std::io;
use winreg::enums::*;
use winreg::RegKey;

/// Читает значение из реестра
pub fn read_registry_value(hkey: HKEY, path: &str, name: &str) -> io::Result<String> {
    let key = RegKey::predef(hkey);
    let subkey = key.open_subkey(path)?;
    let value: String = subkey.get_value(name)?;
    Ok(value)
}

/// Записывает строковое значение в реестр
pub fn write_registry_string(hkey: HKEY, path: &str, name: &str, value: &str) -> io::Result<()> {
    let key = RegKey::predef(hkey);
    let (subkey, _) = key.create_subkey(path)?;
    subkey.set_value(name, &value)?;
    Ok(())
}

/// Записывает DWORD значение в реестр
pub fn write_registry_dword(hkey: HKEY, path: &str, name: &str, value: u32) -> io::Result<()> {
    let key = RegKey::predef(hkey);
    let (subkey, _) = key.create_subkey(path)?;
    subkey.set_value(name, &value)?;
    Ok(())
}

/// Удаляет значение из реестра
pub fn delete_registry_value(hkey: HKEY, path: &str, name: &str) -> io::Result<()> {
    let key = RegKey::predef(hkey);
    let subkey = key.open_subkey_with_flags(path, KEY_WRITE)?;
    subkey.delete_value(name)?;
    Ok(())
}

/// Удаляет ключ из реестра
pub fn delete_registry_key(hkey: HKEY, path: &str) -> io::Result<()> {
    let key = RegKey::predef(hkey);
    key.delete_subkey_all(path)?;
    Ok(())
}

/// Проверяет существование ключа в реестре
pub fn registry_key_exists(hkey: HKEY, path: &str) -> bool {
    let key = RegKey::predef(hkey);
    key.open_subkey(path).is_ok()
}

/// Проверяет существование значения в реестре
pub fn registry_value_exists(hkey: HKEY, path: &str, name: &str) -> bool {
    let key = RegKey::predef(hkey);
    if let Ok(subkey) = key.open_subkey(path) {
        subkey.get_value::<String, _>(name).is_ok()
    } else {
        false
    }
}
