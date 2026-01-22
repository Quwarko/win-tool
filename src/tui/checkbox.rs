use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};

/// Элемент чекбокса
#[derive(Debug, Clone)]
pub struct CheckboxItem {
    pub name: String,
    pub description: String,
    pub version: String,
    pub checked: bool,
}

impl CheckboxItem {
    pub fn new(name: String, description: String, version: String) -> Self {
        Self {
            name,
            description,
            version,
            checked: false,
        }
    }
    
    pub fn toggle(&mut self) {
        self.checked = !self.checked;
    }
    
    pub fn to_list_item(&self, is_selected: bool) -> ListItem {
        let checkbox = if self.checked { "[X]" } else { "[ ]" };
        
        let style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        
        let content = format!(
            "{} {:<30} {:<10} {}",
            checkbox,
            truncate(&self.name, 30),
            self.version,
            truncate(&self.description, 40)
        );
        
        ListItem::new(content).style(style)
    }
}

/// Список чекбоксов
pub struct CheckboxList {
    pub items: Vec<CheckboxItem>,
    pub selected: usize,
}

impl CheckboxList {
    pub fn new(items: Vec<CheckboxItem>) -> Self {
        Self {
            items,
            selected: 0,
        }
    }
    
    pub fn toggle_selected(&mut self) {
        if let Some(item) = self.items.get_mut(self.selected) {
            item.toggle();
        }
    }
    
    pub fn move_selection(&mut self, delta: isize) {
        if self.items.is_empty() {
            return;
        }
        
        let len = self.items.len() as isize;
        let new_selected = (self.selected as isize + delta + len) % len;
        self.selected = new_selected as usize;
    }
    
    pub fn get_checked_items(&self) -> Vec<&CheckboxItem> {
        self.items.iter().filter(|item| item.checked).collect()
    }
    
    pub fn render(&self, title: &str) -> (List, ListState) {
        let header = Line::from(vec![
            Span::raw("    "),
            Span::styled("Название", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("                       "),
            Span::styled("Версия", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("    "),
            Span::styled("Описание", Style::default().add_modifier(Modifier::BOLD)),
        ]);
        
        let mut list_items = vec![ListItem::new(header)];
        
        for (i, item) in self.items.iter().enumerate() {
            list_items.push(item.to_list_item(i == self.selected));
        }
        
        let list = List::new(list_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{} (Space - отметить, Enter - подтвердить)", title))
            )
            .highlight_symbol("➤ ");
        
        let mut state = ListState::default();
        state.select(Some(self.selected + 1)); // +1 из-за заголовка
        
        (list, state)
    }
}

/// Обрезает строку до указанной длины
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_checkbox_toggle() {
        let mut item = CheckboxItem::new(
            "Test".to_string(),
            "Description".to_string(),
            "1.0".to_string()
        );
        
        assert!(!item.checked);
        item.toggle();
        assert!(item.checked);
        item.toggle();
        assert!(!item.checked);
    }
    
    #[test]
    fn test_truncate() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("very long string", 10), "very lo...");
    }
}
