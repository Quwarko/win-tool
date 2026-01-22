@echo off
chcp 65001 >nul
echo.
echo ╔════════════════════════════════════════════╗
echo ║      Win-Tool - Скрипт сборки проекта      ║
echo ╚════════════════════════════════════════════╝
echo.

REM Проверка наличия Rust
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Rust не установлен!
    echo.
    echo Установите Rust используя одну из команд:
    echo   winget install Rustlang.Rustup
    echo   или скачайте с https://rustup.rs/
    echo.
    pause
    exit /b 1
)

echo ✅ Rust обнаружен
cargo --version
echo.

REM Создание необходимых директорий
echo 📁 Создание структуры директорий...
if not exist "data\config_templates\firefox\" mkdir "data\config_templates\firefox"
if not exist "data\config_templates\hiddify\" mkdir "data\config_templates\hiddify"
if not exist "data\installers\" mkdir "data\installers"
echo ✅ Директории созданы
echo.

REM Очистка предыдущей сборки
echo 🧹 Очистка предыдущей сборки...
cargo clean
echo ✅ Очистка завершена
echo.

REM Проверка кода
echo 🔍 Проверка кода...
cargo check
if %errorlevel% neq 0 (
    echo.
    echo ❌ Обнаружены ошибки в коде!
    pause
    exit /b 1
)
echo ✅ Проверка успешна
echo.

REM Сборка в режиме разработки
echo 🔨 Сборка в режиме разработки...
cargo build
if %errorlevel% neq 0 (
    echo.
    echo ❌ Ошибка сборки!
    pause
    exit /b 1
)
echo ✅ Разработческая сборка завершена
echo.

REM Сборка в режиме release
echo 🚀 Сборка оптимизированной версии...
cargo build --release
if %errorlevel% neq 0 (
    echo.
    echo ❌ Ошибка сборки release версии!
    pause
    exit /b 1
)
echo ✅ Release сборка завершена
echo.

REM Информация о результате
echo ╔════════════════════════════════════════════╗
echo ║           Сборка успешно завершена!        ║
echo ╚════════════════════════════════════════════╝
echo.
echo Исполняемые файлы:
echo   Debug:   target\debug\win-tool.exe
echo   Release: target\release\win-tool.exe
echo.

REM Размер файлов
for %%f in (target\debug\win-tool.exe) do echo Debug размер:   %%~zf байт
for %%f in (target\release\win-tool.exe) do echo Release размер: %%~zf байт
echo.

REM Предложение запустить
echo 💡 Хотите запустить Win-Tool от имени администратора? (Y/N)
set /p run_choice=
if /i "%run_choice%"=="Y" (
    echo.
    echo 🚀 Запуск Win-Tool...
    powershell -Command "Start-Process 'target\release\win-tool.exe' -Verb RunAs"
) else (
    echo.
    echo ℹ️  Для запуска используйте:
    echo    target\release\win-tool.exe
    echo.
    echo ⚠️  Рекомендуется запускать от имени администратора!
)

echo.
pause
