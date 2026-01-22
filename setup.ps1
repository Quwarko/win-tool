# Win-Tool Setup Script
# Автоматическая установка всех необходимых зависимостей

$ErrorActionPreference = "Stop"

Write-Host ""
Write-Host "╔════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║   Win-Tool - Установка окружения разработки ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Проверка прав администратора
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "⚠️  ВНИМАНИЕ: Скрипт запущен БЕЗ прав администратора" -ForegroundColor Yellow
    Write-Host "   Некоторые функции могут быть недоступны" -ForegroundColor Yellow
    Write-Host ""
}

# Функция проверки установленного ПО
function Test-CommandExists {
    param($command)
    $null = Get-Command $command -ErrorAction SilentlyContinue
    return $?
}

# Проверка и установка Rust
Write-Host "🦀 Проверка Rust..." -ForegroundColor Yellow
if (Test-CommandExists cargo) {
    $rustVersion = cargo --version
    Write-Host "✅ Rust уже установлен: $rustVersion" -ForegroundColor Green
} else {
    Write-Host "❌ Rust не обнаружен" -ForegroundColor Red
    Write-Host "📥 Установка Rust через rustup..." -ForegroundColor Yellow
    
    # Скачивание и установка rustup
    if (Test-CommandExists winget) {
        winget install --id Rustlang.Rustup --silent --accept-source-agreements --accept-package-agreements
    } else {
        Write-Host "⚠️  WinGet не найден. Скачайте Rust вручную с https://rustup.rs/" -ForegroundColor Yellow
        Start-Process "https://rustup.rs/"
        Read-Host "Нажмите Enter после установки Rust"
    }
    
    # Обновление PATH
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
    
    if (Test-CommandExists cargo) {
        Write-Host "✅ Rust успешно установлен!" -ForegroundColor Green
    } else {
        Write-Host "❌ Не удалось установить Rust. Установите вручную." -ForegroundColor Red
        exit 1
    }
}
Write-Host ""

# Проверка Visual Studio Build Tools
Write-Host "🔧 Проверка Visual Studio Build Tools..." -ForegroundColor Yellow
$vsPath = "C:\Program Files (x86)\Microsoft Visual Studio"
if (Test-Path $vsPath) {
    Write-Host "✅ Visual Studio Build Tools найдены" -ForegroundColor Green
} else {
    Write-Host "⚠️  Visual Studio Build Tools не найдены" -ForegroundColor Yellow
    Write-Host "   Для компиляции Windows-приложений рекомендуется установить:" -ForegroundColor Yellow
    Write-Host "   - Visual Studio 2019 Build Tools или новее" -ForegroundColor Yellow
    Write-Host "   - Или полную Visual Studio Community Edition" -ForegroundColor Yellow
    Write-Host ""
    $installVS = Read-Host "Открыть страницу загрузки Visual Studio? (Y/N)"
    if ($installVS -eq "Y" -or $installVS -eq "y") {
        Start-Process "https://visualstudio.microsoft.com/downloads/"
    }
}
Write-Host ""

# Проверка Git
Write-Host "📦 Проверка Git..." -ForegroundColor Yellow
if (Test-CommandExists git) {
    $gitVersion = git --version
    Write-Host "✅ Git установлен: $gitVersion" -ForegroundColor Green
} else {
    Write-Host "⚠️  Git не установлен (опционально для разработки)" -ForegroundColor Yellow
    $installGit = Read-Host "Установить Git через WinGet? (Y/N)"
    if ($installGit -eq "Y" -or $installGit -eq "y") {
        if (Test-CommandExists winget) {
            winget install --id Git.Git --silent --accept-source-agreements --accept-package-agreements
            Write-Host "✅ Git установлен!" -ForegroundColor Green
        }
    }
}
Write-Host ""

# Создание структуры директорий
Write-Host "📁 Создание структуры проекта..." -ForegroundColor Yellow
$directories = @(
    "src\modules",
    "src\tui",
    "src\utils",
    "data\config_templates\firefox",
    "data\config_templates\hiddify",
    "data\installers"
)

foreach ($dir in $directories) {
    if (-not (Test-Path $dir)) {
        New-Item -Path $dir -ItemType Directory -Force | Out-Null
        Write-Host "  ✅ Создана: $dir" -ForegroundColor Gray
    } else {
        Write-Host "  ℹ️  Существует: $dir" -ForegroundColor DarkGray
    }
}
Write-Host "✅ Структура директорий готова" -ForegroundColor Green
Write-Host ""

# Обновление Rust toolchain
Write-Host "🔄 Обновление Rust toolchain..." -ForegroundColor Yellow
rustup update
Write-Host "✅ Rust toolchain обновлен" -ForegroundColor Green
Write-Host ""

# Установка дополнительных компонентов Rust
Write-Host "📦 Установка дополнительных компонентов Rust..." -ForegroundColor Yellow
rustup component add clippy rustfmt
Write-Host "✅ Компоненты установлены (clippy, rustfmt)" -ForegroundColor Green
Write-Host ""

# Проверка зависимостей cargo
Write-Host "📚 Проверка зависимостей проекта..." -ForegroundColor Yellow
if (Test-Path "Cargo.toml") {
    cargo check
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Все зависимости доступны" -ForegroundColor Green
    } else {
        Write-Host "⚠️  Обнаружены проблемы с зависимостями" -ForegroundColor Yellow
        Write-Host "   Выполните 'cargo build' для установки зависимостей" -ForegroundColor Yellow
    }
} else {
    Write-Host "⚠️  Cargo.toml не найден в текущей директории" -ForegroundColor Yellow
}
Write-Host ""

# Итоговая информация
Write-Host "╔════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║         Установка окружения завершена!     ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""
Write-Host "📋 Следующие шаги:" -ForegroundColor Yellow
Write-Host "  1. Убедитесь, что все файлы проекта на месте" -ForegroundColor White
Write-Host "  2. Запустите 'cargo build' для сборки проекта" -ForegroundColor White
Write-Host "  3. Или используйте 'build.bat' для автоматической сборки" -ForegroundColor White
Write-Host ""
Write-Host "💡 Полезные команды:" -ForegroundColor Yellow
Write-Host "  cargo build          - Сборка в режиме разработки" -ForegroundColor Gray
Write-Host "  cargo build --release - Оптимизированная сборка" -ForegroundColor Gray
Write-Host "  cargo run            - Сборка и запуск" -ForegroundColor Gray
Write-Host "  cargo test           - Запуск тестов" -ForegroundColor Gray
Write-Host "  cargo clippy         - Проверка кода линтером" -ForegroundColor Gray
Write-Host "  cargo fmt            - Форматирование кода" -ForegroundColor Gray
Write-Host ""

$buildNow = Read-Host "🚀 Начать сборку проекта сейчас? (Y/N)"
if ($buildNow -eq "Y" -or $buildNow -eq "y") {
    Write-Host ""
    Write-Host "🔨 Запуск сборки..." -ForegroundColor Yellow
    cargo build --release
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "✅ Сборка успешно завершена!" -ForegroundColor Green
        Write-Host "📦 Исполняемый файл: target\release\win-tool.exe" -ForegroundColor Cyan
        Write-Host ""
        
        $runNow = Read-Host "Запустить Win-Tool от имени администратора? (Y/N)"
        if ($runNow -eq "Y" -or $runNow -eq "y") {
            Start-Process "target\release\win-tool.exe" -Verb RunAs
        }
    } else {
        Write-Host ""
        Write-Host "❌ Ошибка сборки. Проверьте вывод выше." -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "✨ Готово! Удачной разработки!" -ForegroundColor Green
Write-Host ""
