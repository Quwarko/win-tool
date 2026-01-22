use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

mod modules;
mod tui;
mod utils;

use modules::{explorer, packages, personalization, system, uwp};
use utils::admin;

#[derive(Debug, Clone, PartialEq)]
enum AppState {
    MainMenu,
    PackagesMenu,
    WingetPackages,
    UwpApps,
    OtherInstallers,
    ExplorerSettings,
    SystemSettings,
    PersonalizationMenu,
    PersonalizationBasic,
    PersonalizationContext,
}

struct App {
    state: AppState,
    selected: usize,
    is_admin: bool,
}

impl App {
    fn new() -> Self {
        Self {
            state: AppState::MainMenu,
            selected: 0,
            is_admin: admin::is_admin(),
        }
    }

    fn get_menu_items(&self) -> Vec<&str> {
        match self.state {
            AppState::MainMenu => vec![
                "📦 Управление пакетами",
                "📁 Проводник и рабочий стол",
                "⚙️  Система",
                "🎨 Персонализация",
                "🔄 Перезагрузить проводник",
            ],
            AppState::PackagesMenu => vec![
                "🌐 Winget-пакеты",
                "📱 UWP-приложения",
                "💾 Другое (.exe / DISM)",
            ],
            AppState::PersonalizationMenu => vec![
                "✨ Основное",
                "🖱️  Контекстное меню",
            ],
            _ => vec![],
        }
    }

    fn handle_enter(&mut self) -> Option<Action> {
        match self.state {
            AppState::MainMenu => match self.selected {
                0 => {
                    self.state = AppState::PackagesMenu;
                    self.selected = 0;
                }
                1 => {
                    self.state = AppState::ExplorerSettings;
                    self.selected = 0;
                }
                2 => {
                    self.state = AppState::SystemSettings;
                    self.selected = 0;
                }
                3 => {
                    self.state = AppState::PersonalizationMenu;
                    self.selected = 0;
                }
                4 => return Some(Action::RestartExplorer),
                _ => {}
            },
            AppState::PackagesMenu => match self.selected {
                0 => {
                    self.state = AppState::WingetPackages;
                    self.selected = 0;
                }
                1 => {
                    self.state = AppState::UwpApps;
                    self.selected = 0;
                }
                2 => {
                    self.state = AppState::OtherInstallers;
                    self.selected = 0;
                }
                _ => {}
            },
            AppState::PersonalizationMenu => match self.selected {
                0 => {
                    self.state = AppState::PersonalizationBasic;
                    self.selected = 0;
                }
                1 => {
                    self.state = AppState::PersonalizationContext;
                    self.selected = 0;
                }
                _ => {}
            },
            _ => {}
        }
        None
    }

    fn handle_escape(&mut self) {
        self.state = match self.state {
            AppState::PackagesMenu | AppState::ExplorerSettings | AppState::SystemSettings | AppState::PersonalizationMenu => {
                AppState::MainMenu
            }
            AppState::WingetPackages | AppState::UwpApps | AppState::OtherInstallers => {
                AppState::PackagesMenu
            }
            AppState::PersonalizationBasic | AppState::PersonalizationContext => {
                AppState::PersonalizationMenu
            }
            _ => self.state.clone(),
        };
        self.selected = 0;
    }

    fn move_selection(&mut self, delta: isize) {
        let items = self.get_menu_items();
        if items.is_empty() {
            return;
        }
        let len = items.len() as isize;
        let new_selected = (self.selected as isize + delta + len) % len;
        self.selected = new_selected as usize;
    }
}

enum Action {
    RestartExplorer,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация терминала
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let mut running = true;

    while running {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(5),  // Заголовок
                    Constraint::Min(0),     // Контент
                    Constraint::Length(3),  // Подсказки
                ])
                .split(size);

            // Заголовок
            let admin_status = if app.is_admin {
                Span::styled("✅ Запущено с правами администратора", Style::default().fg(Color::Green))
            } else {
                Span::styled("❌ НЕ запущено с правами администратора", Style::default().fg(Color::Red))
            };

            let title = vec![
                Line::from(vec![
                    Span::styled("Win-Tool ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled("🛠️", Style::default()),
                ]),
                Line::from(""),
                Line::from(admin_status),
            ];

            let title_widget = Paragraph::new(title)
                .block(Block::default().borders(Borders::ALL).title("Утилита настройки Windows 11"));
            f.render_widget(title_widget, chunks[0]);

            // Основное меню
            let current_title = match app.state {
                AppState::MainMenu => "Главное меню",
                AppState::PackagesMenu => "Управление пакетами",
                AppState::PersonalizationMenu => "Персонализация",
                AppState::WingetPackages => "Winget-пакеты",
                AppState::UwpApps => "UWP-приложения",
                AppState::OtherInstallers => "Другое (.exe / DISM)",
                AppState::ExplorerSettings => "Проводник и рабочий стол",
                AppState::SystemSettings => "Система",
                AppState::PersonalizationBasic => "Персонализация - Основное",
                AppState::PersonalizationContext => "Персонализация - Контекстное меню",
            };

            let menu_items: Vec<ListItem> = app
                .get_menu_items()
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == app.selected {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(*item).style(style)
                })
                .collect();

            let menu = List::new(menu_items)
                .block(Block::default().borders(Borders::ALL).title(current_title))
                .highlight_symbol("➤ ")
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

            let mut list_state = ListState::default();
            list_state.select(Some(app.selected));
            f.render_stateful_widget(menu, chunks[1], &mut list_state);

            // Подсказки управления
            let hints = Paragraph::new("↑↓ - Навигация | Enter - Выбрать | Esc - Назад | Q - Выход")
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(hints, chunks[2]);
        })?;

        // Обработка событий
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    running = false;
                }
                KeyCode::Up => app.move_selection(-1),
                KeyCode::Down => app.move_selection(1),
                KeyCode::Enter => {
                    if let Some(action) = app.handle_enter() {
                        match action {
                            Action::RestartExplorer => {
                                // Здесь будет логика перезапуска проводника
                                explorer::restart_explorer()?;
                            }
                        }
                    }
                }
                KeyCode::Esc => app.handle_escape(),
                _ => {}
            }
        }
    }

    // Очистка терминала
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
