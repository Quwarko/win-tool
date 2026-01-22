use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Отрисовывает заголовок приложения
pub fn render_header(f: &mut Frame, area: Rect, is_admin: bool) {
    let admin_status = if is_admin {
        Span::styled(
            "✅ Запущено с правами администратора",
            Style::default().fg(Color::Green)
        )
    } else {
        Span::styled(
            "❌ НЕ запущено с правами администратора (некоторые функции недоступны)",
            Style::default().fg(Color::Red)
        )
    };

    let title = vec![
        Line::from(vec![
            Span::styled("Win-Tool ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("🛠️ ", Style::default()),
            Span::styled("- Утилита настройки Windows 11", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(admin_status),
    ];

    let title_widget = Paragraph::new(title)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Left);
    
    f.render_widget(title_widget, area);
}

/// Отрисовывает нижнюю панель с подсказками
pub fn render_footer(f: &mut Frame, area: Rect) {
    let hints = Paragraph::new("↑↓ - Навигация | Space - Отметить | Enter - Выбрать/Применить | Esc - Назад | Q - Выход")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    
    f.render_widget(hints, area);
}

/// Отрисовывает диалог подтверждения
pub fn render_confirmation_dialog(f: &mut Frame, area: Rect, message: &str) {
    let block = Block::default()
        .title("Подтверждение")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));
    
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(message, Style::default().fg(Color::Yellow))),
        Line::from(""),
        Line::from(Span::raw("Продолжить? (Y/N)")),
    ];
    
    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);
    
    // Центрируем диалог
    let dialog_area = centered_rect(60, 20, area);
    f.render_widget(paragraph, dialog_area);
}

/// Создаёт центрированный прямоугольник
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Отрисовывает индикатор загрузки
pub fn render_loading(f: &mut Frame, area: Rect, message: &str) {
    let block = Block::default()
        .title("Обработка...")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));
    
    let text = vec![
        Line::from(""),
        Line::from(Span::styled("⏳ Пожалуйста, подождите...", Style::default().fg(Color::Yellow))),
        Line::from(""),
        Line::from(Span::raw(message)),
    ];
    
    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);
    
    let dialog_area = centered_rect(60, 20, area);
    f.render_widget(paragraph, dialog_area);
}
