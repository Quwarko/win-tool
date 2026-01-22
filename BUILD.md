# Инструкции по сборке Win-Tool

## Требования для разработки

### Windows
- Windows 11 (рекомендуется)
- Visual Studio Build Tools 2019+ или Visual Studio 2019+
- Rust 1.70 или новее

### Установка Rust

```powershell
# Скачайте и установите rustup с https://rustup.rs/
# Или используйте winget:
winget install Rustlang.Rustup

# Проверьте установку
rustc --version
cargo --version
```

## Сборка проекта

### Разработческая сборка

```bash
# Клонируйте репозиторий
git clone <repository-url>
cd win-tool

# Создайте необходимые директории
mkdir -p data/config_templates/firefox
mkdir -p data/config_templates/hiddify
mkdir -p data/installers

# Соберите проект в режиме разработки
cargo build

# Запустите
cargo run
```

### Release сборка

```bash
# Оптимизированная сборка для релиза
cargo build --release

# Исполняемый файл будет в:
# target/release/win-tool.exe
```

### Минимизация размера exe

Для создания максимально компактного исполняемого файла:

```bash
# 1. Соберите с оптимизацией
cargo build --release

# 2. (Опционально) Используйте UPX для сжатия
# Скачайте UPX с https://upx.github.io/
upx --best --lzma target/release/win-tool.exe

# Размер файла может уменьшиться с ~5 МБ до ~2 МБ
```

## Настройка проекта

### Cargo.toml оптимизация

Файл `Cargo.toml` уже настроен с оптимальными параметрами:

```toml
[profile.release]
strip = true          # Удаляет отладочную информацию
opt-level = "z"       # Оптимизация по размеру
lto = true           # Link Time Optimization
codegen-units = 1    # Улучшает оптимизацию
```

### Зависимости

Основные зависимости проекта:

- **ratatui** - Терминальный UI фреймворк
- **crossterm** - Кроссплатформенный терминальный бэкенд
- **winreg** - Работа с реестром Windows
- **winapi** (опционально) - Прямые вызовы Windows API
- **serde/serde_json** - Сериализация/десериализация данных
- **anyhow** - Удобная обработка ошибок
- **tokio** - Асинхронный runtime (для будущих расширений)

## Разработка

### Запуск тестов

```bash
# Запустить все тесты
cargo test

# Запустить тесты с выводом
cargo test -- --nocapture

# Запустить конкретный тест
cargo test test_admin_check
```

### Проверка кода

```bash
# Запустить clippy (линтер)
cargo clippy

# Форматирование кода
cargo fmt

# Проверка без сборки
cargo check
```

### Отладка

```bash
# Сборка с отладочной информацией
cargo build

# Запуск с логами
RUST_LOG=debug cargo run

# Использование отладчика в VS Code
# Создайте .vscode/launch.json:
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug",
            "cargo": {
                "args": [
                    "build",
                    "--bin=win-tool"
                ]
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

## Добавление новых функций

### Добавление нового пакета Winget

Отредактируйте `src/modules/packages.rs`:

```rust
pub const WINGET_PACKAGES: &[(&str, &str, &str)] = &[
    // ... существующие пакеты ...
    ("YourApp.AppName", "latest", "🎯 Описание вашего приложения"),
];
```

### Добавление новой настройки проводника

Отредактируйте `src/modules/explorer.rs`:

```rust
pub fn your_new_setting(enable: bool) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey("Software\\Microsoft\\Windows\\...")?;
    
    key.set_value("YourSetting", &(if enable { 1u32 } else { 0u32 }))?;
    
    println!("{} Ваша настройка {}", 
        if enable { "✅" } else { "❌" }, 
        if enable { "включена" } else { "отключена" }
    );
    
    Ok(())
}
```

### Добавление нового модуля

1. Создайте файл в `src/modules/your_module.rs`
2. Добавьте `pub mod your_module;` в `src/modules/mod.rs`
3. Реализуйте необходимый функционал
4. Добавьте пункт меню в `src/main.rs`

## Распространение

### Создание инсталлятора

Вы можете использовать следующие инструменты:

1. **Inno Setup** - Создание Windows инсталлятора
2. **WiX Toolset** - MSI пакеты
3. **NSIS** - Nullsoft Scriptable Install System

Пример скрипта Inno Setup (`installer.iss`):

```ini
[Setup]
AppName=Win-Tool
AppVersion=0.1.0
DefaultDirName={pf}\Win-Tool
DefaultGroupName=Win-Tool
OutputBaseFilename=win-tool-setup
Compression=lzma2
SolidCompression=yes

[Files]
Source: "target\release\win-tool.exe"; DestDir: "{app}"
Source: "data\*"; DestDir: "{app}\data"; Flags: recursesubdirs

[Icons]
Name: "{group}\Win-Tool"; Filename: "{app}\win-tool.exe"
```

## Устранение проблем

### Ошибка компиляции winapi

```bash
# Установите Visual Studio Build Tools
# https://visualstudio.microsoft.com/downloads/
# Выберите "C++ build tools"
```

### Ошибка прав доступа

```bash
# Запустите PowerShell или CMD от имени администратора
# Затем выполните cargo build
```

### Проблемы с зависимостями

```bash
# Очистите кэш cargo
cargo clean

# Обновите зависимости
cargo update

# Пересоберите проект
cargo build --release
```

## Continuous Integration

Пример GitHub Actions (`.github/workflows/build.yml`):

```yaml
name: Build

on: [push, pull_request]

jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - uses: actions/upload-artifact@v2
        with:
          name: win-tool
          path: target/release/win-tool.exe
```

## Дополнительные ресурсы

- [Rust Book](https://doc.rust-lang.org/book/)
- [Ratatui Docs](https://docs.rs/ratatui/)
- [Windows Registry Documentation](https://docs.microsoft.com/windows/win32/sysinfo/registry)
- [WinGet Documentation](https://docs.microsoft.com/windows/package-manager/)
