use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io::{Result, stdout};

/// Defines the current view/state of the application
enum AppMode {
    Login,
    Chat,
}

/// Active field on the login screen
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
}

impl App {
    fn new() -> Self {
        Self {
            mode: AppMode::Login,
            active_field: LoginField::Username,
            username: String::new(),
            password: String::new(),
            chat_input: String::new(),
            messages: vec![
                "System: Welcome to the Chat Room!".to_string(),
                "Bot: Type a message and hit Enter to speak.".to_string(),
            ],
        }
    }
}

fn main() -> Result<()> {
    // Setup terminal
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new();

    // Main App Loop
    loop {
        terminal.draw(|frame| render_ui(frame, &app))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    // Global exit on Esc
                    if key.code == KeyCode::Esc {
                        break;
                    }

                    match app.mode {
                        AppMode::Login => handle_login_input(&mut app, key.code),
                        AppMode::Chat => handle_chat_input(&mut app, key.code),
                    }
                }
            }
        }
    }

    // Restore terminal
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
            // Toggle between Username and Password fields
            app.active_field = match app.active_field {
                LoginField::Username => LoginField::Password,
                LoginField::Password => LoginField::Username,
            };
        }
        KeyCode::Enter => {
            // "Authenticate" and switch screen
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
    match key {
        KeyCode::Enter => {
            if !app.chat_input.trim().is_empty() {
                let msg = format!("{}: {}", app.username, app.chat_input.trim());
                app.messages.push(msg);
                app.chat_input.clear();
            }
        }
        KeyCode::Char(c) => app.chat_input.push(c),
        KeyCode::Backspace => {
            app.chat_input.pop();
        }
        _ => {}
    }
}

// ==========================================
// UI RENDERING
// ==========================================

fn render_ui(frame: &mut Frame, app: &App) {
    match app.mode {
        AppMode::Login => render_login_screen(frame, app),
        AppMode::Chat => render_chat_screen(frame, app),
    }
}

fn render_login_screen(frame: &mut Frame, app: &App) {
    // Center a box on screen for the login form
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

    // Main Outer Container
    let main_block = Block::default()
        .title(" Login Screen (Press ESC to Exit) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(main_block, block_area);

    // Inner layout for inputs
    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Username input
            Constraint::Length(3), // Password input
            Constraint::Length(1), // Instruction
        ])
        .split(block_area);

    // Dynamic borders depending on active field focus
    let user_border_color = match app.active_field {
        LoginField::Username => Color::Yellow,
        _ => Color::DarkGray,
    };
    let pass_border_color = match app.active_field {
        LoginField::Password => Color::Yellow,
        _ => Color::DarkGray,
    };

    // Username Widget
    let username_widget = Paragraph::new(app.username.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Username ")
            .border_style(Style::default().fg(user_border_color)),
    );
    frame.render_widget(username_widget, inner_layout[0]);

    // Password Widget (Masked input)
    let masked_password: String = "*".repeat(app.password.len());
    let password_widget = Paragraph::new(masked_password).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Password ")
            .border_style(Style::default().fg(pass_border_color)),
    );
    frame.render_widget(password_widget, inner_layout[1]);

    // Help Text
    let help_text = Paragraph::new("Press Tab/Arrow Keys to switch fields | Enter to Login")
        .style(Style::default().fg(Color::Gray));
    frame.render_widget(help_text, inner_layout[2]);
}

fn render_chat_screen(frame: &mut Frame, app: &App) {
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
        .map(|msg| ListItem::new(msg.as_str()))
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
                " Type Message as [{}] (Press ESC to Quit) ",
                app.username
            ))
            .border_style(Style::default().fg(Color::Yellow)),
    );
    frame.render_widget(input, chunks[1]);
}
