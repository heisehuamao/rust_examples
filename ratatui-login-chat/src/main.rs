use crossterm::{
    ExecutableCommand,
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
        MouseButton, MouseEvent, MouseEventKind,
    },
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};
use std::io::{Result, stdout};

// ==========================================
// STATE MODELS
// ==========================================

enum AppMode {
    Login,
    Chat,
}

enum LoginField {
    Username,
    Password,
}

#[derive(PartialEq)]
enum ActivePanel {
    Sidebar,
    ChatBox,
    Input,
}

struct ChatSession {
    name: String,
    messages: Vec<String>,
}

struct App {
    mode: AppMode,
    // Login State
    active_field: LoginField,
    username: String,
    password: String,
    // Chat & Sidebar State
    active_panel: ActivePanel,
    sessions: Vec<ChatSession>,
    session_list_state: ListState,
    chat_input: String,
    // Command Popup State
    available_commands: Vec<(&'static str, &'static str)>,
    command_list_state: ListState,
    // Clickable Layout Areas (Hit Testing)
    username_area: Rect,
    password_area: Rect,
    sidebar_area: Rect,
    messages_area: Rect,
    input_area: Rect,
}

impl App {
    fn new() -> Self {
        let commands = vec![
            ("/help", "Show help information"),
            ("/clear", "Clear current chat history"),
            ("/nick", "Change display username"),
            ("/users", "List online users"),
            ("/quit", "Exit application"),
        ];

        let mut command_list_state = ListState::default();
        command_list_state.select(Some(0));

        let initial_sessions = vec![
            ChatSession {
                name: "General".to_string(),
                messages: vec![
                    "System: Welcome to General Chat!".to_string(),
                    "Bot: Type '/' to open the command palette.".to_string(),
                ],
            },
            ChatSession {
                name: "Project Rust".to_string(),
                messages: vec![
                    "System: Welcome to Project Rust discussion.".to_string(),
                    "Bot: Let's discuss Ratatui updates here!".to_string(),
                ],
            },
            ChatSession {
                name: "Random".to_string(),
                messages: vec!["System: Off-topic lounge.".to_string()],
            },
        ];

        let mut session_list_state = ListState::default();
        session_list_state.select(Some(0));

        Self {
            mode: AppMode::Login,
            active_field: LoginField::Username,
            username: String::new(),
            password: String::new(),
            active_panel: ActivePanel::Input,
            sessions: initial_sessions,
            session_list_state,
            chat_input: String::new(),
            available_commands: commands,
            command_list_state,
            username_area: Rect::default(),
            password_area: Rect::default(),
            sidebar_area: Rect::default(),
            messages_area: Rect::default(),
            input_area: Rect::default(),
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

    fn next_session(&mut self) {
        if self.sessions.is_empty() {
            return;
        }
        let i = match self.session_list_state.selected() {
            Some(i) => {
                if i >= self.sessions.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.session_list_state.select(Some(i));
    }

    fn previous_session(&mut self) {
        if self.sessions.is_empty() {
            return;
        }
        let i = match self.session_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.sessions.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.session_list_state.select(Some(i));
    }

    fn current_session_mut(&mut self) -> Option<&mut ChatSession> {
        if let Some(idx) = self.session_list_state.selected() {
            self.sessions.get_mut(idx)
        } else {
            None
        }
    }

    fn current_session(&self) -> Option<&ChatSession> {
        if let Some(idx) = self.session_list_state.selected() {
            self.sessions.get(idx)
        } else {
            None
        }
    }
}

// ==========================================
// MAIN LOOP & TERMINAL SETUP
// ==========================================

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new();

    loop {
        terminal.draw(|frame| render_ui(frame, &mut app))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        if key.code == KeyCode::Esc {
                            if app.chat_input.starts_with('/') {
                                app.chat_input.clear();
                            } else {
                                break;
                            }
                        } else {
                            match app.mode {
                                AppMode::Login => handle_login_input(&mut app, key.code),
                                AppMode::Chat => {
                                    handle_chat_input(&mut app, key.code, key.modifiers)
                                }
                            }
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                        handle_mouse_click(&mut app, mouse);
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(DisableMouseCapture)?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

// ==========================================
// EVENT HANDLERS
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
                // Pre-compute string to avoid overlapping borrow on `app`
                let login_msg = format!("System: Logged in as {}", app.username);
                if let Some(session) = app.current_session_mut() {
                    session.messages.push(login_msg);
                }
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

fn handle_chat_input(app: &mut App, key: KeyCode, _modifiers: KeyModifiers) {
    let is_command_mode = app.chat_input.starts_with('/');

    // Command palette navigation when typing '/'
    if is_command_mode {
        let filtered = app.filtered_commands();
        match key {
            KeyCode::Up => {
                app.previous_command();
                return;
            }
            KeyCode::Down => {
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

    // Panel navigation via Tab / Shift+Tab
    if key == KeyCode::Tab {
        app.active_panel = match app.active_panel {
            ActivePanel::Input => ActivePanel::Sidebar,
            ActivePanel::Sidebar => ActivePanel::ChatBox,
            ActivePanel::ChatBox => ActivePanel::Input,
        };
        return;
    } else if key == KeyCode::BackTab {
        app.active_panel = match app.active_panel {
            ActivePanel::Input => ActivePanel::ChatBox,
            ActivePanel::ChatBox => ActivePanel::Sidebar,
            ActivePanel::Sidebar => ActivePanel::Input,
        };
        return;
    }

    // Active panel actions
    match app.active_panel {
        ActivePanel::Sidebar => match key {
            KeyCode::Up | KeyCode::Char('k') => app.previous_session(),
            KeyCode::Down | KeyCode::Char('j') => app.next_session(),
            _ => {}
        },
        ActivePanel::ChatBox => {}
        ActivePanel::Input => match key {
            KeyCode::Enter => {
                if !app.chat_input.trim().is_empty() {
                    let msg = format!("{}: {}", app.username, app.chat_input.trim());
                    if let Some(session) = app.current_session_mut() {
                        session.messages.push(msg);
                    }
                    app.chat_input.clear();
                }
            }
            KeyCode::Char(c) => {
                app.chat_input.push(c);
                app.command_list_state.select(Some(0));
            }
            KeyCode::Backspace => {
                app.chat_input.pop();
                app.command_list_state.select(Some(0));
            }
            _ => {}
        },
    }
}

fn handle_mouse_click(app: &mut App, mouse: MouseEvent) {
    let pos = Position::new(mouse.column, mouse.row);

    match app.mode {
        AppMode::Login => {
            if app.username_area.contains(pos) {
                app.active_field = LoginField::Username;
            } else if app.password_area.contains(pos) {
                app.active_field = LoginField::Password;
            }
        }
        AppMode::Chat => {
            if app.sidebar_area.contains(pos) {
                app.active_panel = ActivePanel::Sidebar;

                // Select channel clicked in sidebar
                let relative_y = pos.y.saturating_sub(app.sidebar_area.y + 1);
                let clicked_idx = relative_y as usize;
                if clicked_idx < app.sessions.len() {
                    app.session_list_state.select(Some(clicked_idx));
                }
            } else if app.messages_area.contains(pos) {
                app.active_panel = ActivePanel::ChatBox;
            } else if app.input_area.contains(pos) {
                app.active_panel = ActivePanel::Input;
            }
        }
    }
}

fn execute_command(app: &mut App, cmd: &str) {
    match cmd {
        "/clear" => {
            if let Some(session) = app.current_session_mut() {
                session.messages.clear();
            }
        }
        "/help" => {
            if let Some(session) = app.current_session_mut() {
                session
                    .messages
                    .push("System: Commands: /clear, /nick, /users, /quit".to_string());
            }
        }
        "/users" => {
            if let Some(session) = app.current_session_mut() {
                session
                    .messages
                    .push("System: Online users: [You], Bot".to_string());
            }
        }
        "/quit" => std::process::exit(0),
        "/nick" => app.chat_input = "/nick ".to_string(),
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

fn render_login_screen(frame: &mut Frame, app: &mut App) {
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

    // Record mouse hit bounds
    app.username_area = inner_layout[0];
    app.password_area = inner_layout[1];

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

    let help_text = Paragraph::new("Click field or Tab/Arrows to focus | Enter to Login")
        .style(Style::default().fg(Color::Gray));
    frame.render_widget(help_text, inner_layout[2]);

    // Set cursor based on active field
    match app.active_field {
        LoginField::Username => {
            let cursor_x = inner_layout[0].x + 1 + app.username.len() as u16;
            let cursor_y = inner_layout[0].y + 1;
            let max_x = inner_layout[0].x + inner_layout[0].width.saturating_sub(2);
            frame.set_cursor_position(Position::new(cursor_x.min(max_x), cursor_y));
        }
        LoginField::Password => {
            let cursor_x = inner_layout[1].x + 1 + app.password.len() as u16;
            let cursor_y = inner_layout[1].y + 1;
            let max_x = inner_layout[1].x + inner_layout[1].width.saturating_sub(2);
            frame.set_cursor_position(Position::new(cursor_x.min(max_x), cursor_y));
        }
    }
}

fn render_chat_screen(frame: &mut Frame, app: &mut App) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(22), // Sidebar width
            Constraint::Min(1),     // Chat area
        ])
        .split(frame.size());

    // Record Sidebar hit area
    app.sidebar_area = main_layout[0];

    // 1. Sidebar Channels List
    let sidebar_items: Vec<ListItem> = app
        .sessions
        .iter()
        .map(|s| ListItem::new(format!("# {}", s.name)))
        .collect();

    let sidebar_border_color = if app.active_panel == ActivePanel::Sidebar {
        Color::Yellow
    } else {
        Color::DarkGray
    };

    let sidebar_list = List::new(sidebar_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Channels ")
                .border_style(Style::default().fg(sidebar_border_color)),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(sidebar_list, main_layout[0], &mut app.session_list_state);

    // 2. Chat Layout (Messages top, Input bottom)
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Messages
            Constraint::Length(3), // Input
        ])
        .split(main_layout[1]);

    // Record Messages and Input hit areas
    app.messages_area = right_chunks[0];
    app.input_area = right_chunks[1];

    let chat_border_color = if app.active_panel == ActivePanel::ChatBox {
        Color::Yellow
    } else {
        Color::Green
    };

    let current_session_name = app
        .current_session()
        .map(|s| s.name.as_str())
        .unwrap_or("Chat Room");

    // Align Bot & System messages to the right border
    let items: Vec<ListItem> = app
        .current_session()
        .map(|s| {
            s.messages
                .iter()
                .map(|msg| {
                    if msg.starts_with("Bot:") || msg.starts_with("System:") {
                        ListItem::new(Line::from(msg.as_str()).alignment(Alignment::Right))
                    } else {
                        ListItem::new(Line::from(msg.as_str()).alignment(Alignment::Left))
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let messages_list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" #{} ", current_session_name))
            .border_style(Style::default().fg(chat_border_color)),
    );
    frame.render_widget(messages_list, right_chunks[0]);

    // Input View
    let input_border_color = if app.active_panel == ActivePanel::Input {
        Color::Yellow
    } else {
        Color::DarkGray
    };

    let input = Paragraph::new(app.chat_input.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(
                " Message [{}] (Tab to switch | '/' for commands) ",
                app.username
            ))
            .border_style(Style::default().fg(input_border_color)),
    );
    frame.render_widget(input, right_chunks[1]);

    // -------------------------------------------------------------
    // NEW: Display the terminal cursor inside the input field
    // -------------------------------------------------------------
    if app.active_panel == ActivePanel::Input {
        // x: left border (1 cell offset) + current character count
        let cursor_x = right_chunks[1].x + 1 + app.chat_input.len() as u16;
        // y: top border (1 row offset)
        let cursor_y = right_chunks[1].y + 1;

        // Ensure cursor doesn't render past the right border of the input box
        let max_x = right_chunks[1].x + right_chunks[1].width.saturating_sub(2);
        frame.set_cursor_position(Position::new(cursor_x.min(max_x), cursor_y));
    }

    // Popup overlay for '/' commands
    if app.chat_input.starts_with('/') {
        render_command_popup(frame, app, right_chunks[1]);
    }
}

fn render_command_popup(frame: &mut Frame, app: &mut App, input_area: Rect) {
    let filtered_cmds = app.filtered_commands();
    if filtered_cmds.is_empty() {
        return;
    }

    let popup_height = (filtered_cmds.len() as u16 + 2).min(8);

    let popup_area = Rect {
        x: input_area.x + 2,
        y: input_area.y.saturating_sub(popup_height),
        width: 45.min(input_area.width.saturating_sub(4)),
        height: popup_height,
    };

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
                .title(" Commands (▲/▼ to select) ")
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
