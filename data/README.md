# Директория данных Win-Tool

Эта директория содержит конфигурационные файлы и установщики для Win-Tool.

## Структура

```
data/
├── config_templates/    # Шаблоны конфигураций для приложений
│   ├── firefox/        # Настройки Firefox
│   └── hiddify/        # Настройки Hiddify
└── installers/         # .exe файлы установщиков
```

## config_templates/

### firefox/

Поместите сюда файл `user.js` с настройками Firefox, которые будут применяться автоматически после установки.

Пример содержимого `user.js`:

```javascript
// Отключить телеметрию
user_pref("toolkit.telemetry.enabled", false);
user_pref("toolkit.telemetry.unified", false);
user_pref("datareporting.healthreport.uploadEnabled", false);

// Отключить Pocket
user_pref("extensions.pocket.enabled", false);

// Включить HTTPS-Only режим
user_pref("dom.security.https_only_mode", true);

// Отключить рекламу на новой вкладке
user_pref("browser.newtabpage.activity-stream.showSponsored", false);
user_pref("browser.newtabpage.activity-stream.showSponsoredTopSites", false);

// Улучшить приватность
user_pref("privacy.trackingprotection.enabled", true);
user_pref("privacy.trackingprotection.socialtracking.enabled", true);

// Отключить автовоспроизведение
user_pref("media.autoplay.default", 5);
```

**Расположение файла после установки:**
- Обычно: `%APPDATA%\Mozilla\Firefox\Profiles\xxxxxxxx.default-release\user.js`
- Win-Tool автоматически найдет профиль и скопирует файл

### hiddify/

Поместите сюда конфигурационные файлы Hiddify:

1. `current-config.json` - Основной файл конфигурации
2. `profile-config.json` - Файл профиля (переименуйте UUID по необходимости)

Пример `current-config.json`:

```json
{
  "enable": true,
  "mode": "rule",
  "mixed-port": 7890,
  "allow-lan": false,
  "log-level": "info",
  "ipv6": false
}
```

**Расположение после установки:**
- `%APPDATA%\Hiddify\hiddify\current-config.json`
- `%APPDATA%\Hiddify\hiddify\configs\[uuid].json`

## installers/

Поместите сюда .exe файлы установщиков приложений, которые не доступны через WinGet.

### Поддерживаемые форматы:
- `.exe` - Исполняемые установщики
- Рекомендуется использовать silent-установщики

### Примеры приложений для размещения:
- Корпоративное ПО
- Приложения с лицензиями
- Старые версии программ
- Портативные версии

### Использование:
1. Поместите .exe файл в эту директорию
2. Запустите Win-Tool
3. Перейдите в "Управление пакетами" → "Другое (.exe / DISM)"
4. Выберите нужный установщик из списка
5. Подтвердите установку

### Автоматическая установка:

Создайте файл `install-config.json` для автоматической установки:

```json
{
  "installers": [
    {
      "filename": "app1.exe",
      "silent_args": "/S /SILENT",
      "description": "Описание приложения"
    },
    {
      "filename": "app2.exe",
      "silent_args": "/quiet /norestart",
      "description": "Другое приложение"
    }
  ]
}
```

## Примечания

- Не коммитьте приватные конфигурационные файлы в Git
- Файлы .exe не включаются в репозиторий (см. .gitignore)
- Храните резервные копии важных конфигураций
- Проверяйте файлы на вирусы перед использованием

## Безопасность

⚠️ **ВАЖНО:**
- Скачивайте .exe файлы только из официальных источников
- Проверяйте контрольные суммы (MD5/SHA256)
- Запускайте антивирус перед установкой
- Не используйте пиратское ПО)

## Дополнительные шаблоны

Вы можете создать дополнительные директории для других приложений:

```
data/config_templates/
├── vscode/          # Настройки VS Code (settings.json)
├── git/             # .gitconfig
├── powershell/      # profile.ps1
└── terminal/        # settings.json для Windows Terminal
```

Win-Tool можно легко расширить для поддержки дополнительных приложений.
