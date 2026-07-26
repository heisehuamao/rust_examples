use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};

use ratatui::layout::Alignment;
use std::io::{Result, stdout};

enum AppMode {
    Login,
    Chat,
}

enum LoginField {
    Username,
    Password,
}

struct App {
    mode: AppMode,
    // Login State
    active_field: LoginField,
    username: String,
    password: String,
    // Chat State
    chat_input: String,
    messages: Vec<String>,
    // Command Popup State
    available_commands: Vec<(&'static str, &'static str)>, // (Command, Description)
    command_list_state: ListState,
}

impl App {
    fn new() -> Self {
        let commands = vec![
            ("/help", "Show help information"),
            ("/clear", "Clear chat history"),
            ("/nick", "Change your display username"),
            ("/users", "List online users"),
            ("/quit", "Exit application"),
        ];

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            mode: AppMode::Login,
            active_field: LoginField::Username,
            username: String::new(),
            password: String::new(),
            chat_input: String::new(),
            messages: vec![
                "System: Welcome to the Chat Room!".to_string(),
                "Bot: Type '/' to open the command palette.".to_string(),
            ],
            available_commands: commands,
            command_list_state: list_state,
        }
    }

    fn filtered_commands(&self) -> Vec<(&'static str, &'static str)> {
        if let Some(query) = self.chat_input.strip_prefix('/') {
            self.available_commands
                .iter()
                .filter(|(cmd, _)| cmd.strip_prefix('/').unwrap_or("").starts_with(query))
                .copied()
                .collect()
        } else {
            Vec::new()
        }
    }

    fn next_command(&mut self) {
        let filtered = self.filtered_commands();
        if filtered.is_empty() {
            return;
        }
        let i = match self.command_list_state.selected() {
            Some(i) => {
                if i >= filtered.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.command_list_state.select(Some(i));
    }

    fn previous_command(&mut self) {
        let filtered = self.filtered_commands();
        if filtered.is_empty() {
            return;
        }
        let i = match self.command_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    filtered.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.command_list_state.select(Some(i));
    }
}

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new();

    loop {
        terminal.draw(|frame| render_ui(frame, &mut app))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Esc {
                        // If popup is open, Esc closes popup by clearing input prefix '/'
                        if app.chat_input.starts_with('/') {
                            app.chat_input.clear();
                        } else {
                            break;
                        }
                    } else {
                        match app.mode {
                            AppMode::Login => handle_login_input(&mut app, key.code),
                            AppMode::Chat => handle_chat_input(&mut app, key.code),
                        }
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

// ==========================================
// INPUT HANDLERS
// ==========================================

fn handle_login_input(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Tab | KeyCode::Down | KeyCode::Up => {
            app.active_field = match app.active_field {
                LoginField::Username => LoginField::Password,
                LoginField::Password => LoginField::Username,
            };
        }
        KeyCode::Enter => {
            if !app.username.trim().is_empty() {
                app.messages
                    .push(format!("System: Logged in as {}", app.username));
                app.mode = AppMode::Chat;
            }
        }
        KeyCode::Char(c) => match app.active_field {
            LoginField::Username => app.username.push(c),
            LoginField::Password => app.password.push(c),
        },
        KeyCode::Backspace => match app.active_field {
            LoginField::Username => {
                app.username.pop();
            }
            LoginField::Password => {
                app.password.pop();
            }
        },
        _ => {}
    }
}

fn handle_chat_input(app: &mut App, key: KeyCode) {
    let is_command_mode = app.chat_input.starts_with('/');

    if is_command_mode {
        let filtered = app.filtered_commands();

        match key {
            KeyCode::Up => {
                app.previous_command();
                return;
            }
            KeyCode::Down | KeyCode::Tab => {
                app.next_command();
                return;
            }
            KeyCode::Enter => {
                if !filtered.is_empty() {
                    if let Some(selected_idx) = app.command_list_state.selected() {
                        if let Some(&(cmd, _)) = filtered.get(selected_idx) {
                            execute_command(app, cmd);
                            return;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Standard Chat Input
    match key {
        KeyCode::Enter => {
            if !app.chat_input.trim().is_empty() {
                let msg = format!("{}: {}", app.username, app.chat_input.trim());
                app.messages.push(msg);
                app.chat_input.clear();
            }
        }
        KeyCode::Char(c) => {
            app.chat_input.push(c);
            // Reset command selection when input changes
            app.command_list_state.select(Some(0));
        }
        KeyCode::Backspace => {
            app.chat_input.pop();
            app.command_list_state.select(Some(0));
        }
        _ => {}
    }
}

fn execute_command(app: &mut App, cmd: &str) {
    match cmd {
        "/clear" => app.messages.clear(),
        "/help" => app
            .messages
            .push("System: Available commands: /clear, /nick, /users, /quit".to_string()),
        "/users" => app
            .messages
            .push("System: Online users: [You], Bot".to_string()),
        "/quit" => std::process::exit(0),
        "/nick" => app.chat_input = "/nick ".to_string(), // Keep prefix to complete command with args
        _ => {}
    }

    if cmd != "/nick" {
        app.chat_input.clear();
    }
}

// ==========================================
// UI RENDERING
// ==========================================

fn render_ui(frame: &mut Frame, app: &mut App) {
    match app.mode {
        AppMode::Login => render_login_screen(frame, app),
        AppMode::Chat => render_chat_screen(frame, app),
    }
}

fn render_login_screen(frame: &mut Frame, app: &App) {
    let area = frame.size();
    let vertical_center = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(10),
            Constraint::Percentage(30),
        ])
        .split(area)[1];

    let block_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Length(50),
            Constraint::Percentage(25),
        ])
        .split(vertical_center)[1];

    let main_block = Block::default()
        .title(" Login Screen (Press ESC to Exit) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(main_block, block_area);

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(block_area);

    let user_border_color = match app.active_field {
        LoginField::Username => Color::Yellow,
        _ => Color::DarkGray,
    };
    let pass_border_color = match app.active_field {
        LoginField::Password => Color::Yellow,
        _ => Color::DarkGray,
    };

    let username_widget = Paragraph::new(app.username.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Username ")
            .border_style(Style::default().fg(user_border_color)),
    );
    frame.render_widget(username_widget, inner_layout[0]);

    let masked_password: String = "*".repeat(app.password.len());
    let password_widget = Paragraph::new(masked_password).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Password ")
            .border_style(Style::default().fg(pass_border_color)),
    );
    frame.render_widget(password_widget, inner_layout[1]);

    let help_text = Paragraph::new("Press Tab/Arrow Keys to switch fields | Enter to Login")
        .style(Style::default().fg(Color::Gray));
    frame.render_widget(help_text, inner_layout[2]);
}

// fn render_chat_screen(frame: &mut Frame, app: &mut App) {
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Min(1),    // Messages area
//             Constraint::Length(3), // Input area
//         ])
//         .split(frame.size());
//
//     // Messages View
//     let items: Vec<ListItem> = app
//         .messages
//         .iter()
//         .map(|msg| ListItem::new(msg.as_str()))
//         .collect();
//
//     let messages_list = List::new(items).block(
//         Block::default()
//             .borders(Borders::ALL)
//             .title(" Chat Room ")
//             .border_style(Style::default().fg(Color::Green)),
//     );
//     frame.render_widget(messages_list, chunks[0]);
//
//     // Input View
//     let input = Paragraph::new(app.chat_input.as_str()).block(
//         Block::default()
//             .borders(Borders::ALL)
//             .title(format!(
//                 " Type Message as [{}] (Type '/' for commands) ",
//                 app.username
//             ))
//             .border_style(Style::default().fg(Color::Yellow)),
//     );
//     frame.render_widget(input, chunks[1]);
//
//     // Render Command Selection Popup overlay if input starts with '/'
//     if app.chat_input.starts_with('/') {
//         render_command_popup(frame, app, chunks[1]);
//     }
// }

fn render_chat_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Messages area
            Constraint::Length(3), // Input area
        ])
        .split(frame.size());

    // Messages View
    let items: Vec<ListItem> = app
        .messages
        .iter()
        .map(|msg| {
            // Check if the message is from "Bot:" or "System:" to align right
            if msg.starts_with("Bot:") || msg.starts_with("System:") {
                let line = Line::from(msg.as_str()).alignment(Alignment::Right);
                ListItem::new(line)
            } else {
                // User messages remain left-aligned
                let line = Line::from(msg.as_str()).alignment(Alignment::Left);
                ListItem::new(line)
            }
        })
        .collect();

    let messages_list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Chat Room ")
            .border_style(Style::default().fg(Color::Green)),
    );
    frame.render_widget(messages_list, chunks[0]);

    // Input View
    let input = Paragraph::new(app.chat_input.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(
                " Type Message as [{}] (Type '/' for commands) ",
                app.username
            ))
            .border_style(Style::default().fg(Color::Yellow)),
    );
    frame.render_widget(input, chunks[1]);

    // Render Command Selection Popup overlay if input starts with '/'
    if app.chat_input.starts_with('/') {
        render_command_popup(frame, app, chunks[1]);
    }
}

fn render_command_popup(frame: &mut Frame, app: &mut App, input_area: Rect) {
    let filtered_cmds = app.filtered_commands();
    if filtered_cmds.is_empty() {
        return;
    }

    let popup_height = (filtered_cmds.len() as u16 + 2).min(8);

    // Calculate popup area placed directly above the input box
    let popup_area = Rect {
        x: input_area.x + 2,
        y: input_area.y.saturating_sub(popup_height),
        width: 45.min(input_area.width.saturating_sub(4)),
        height: popup_height,
    };

    // Clear background behind popup overlay
    frame.render_widget(Clear, popup_area);

    let items: Vec<ListItem> = filtered_cmds
        .iter()
        .map(|(cmd, desc)| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{:<10} ", cmd),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(*desc, Style::default().fg(Color::DarkGray)),
            ]))
        })
        .collect();

    let popup_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Commands (▲/▼/Tab to select) ")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_stateful_widget(popup_list, popup_area, &mut app.command_list_state);
}
