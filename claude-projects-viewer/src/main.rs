mod parser;
mod types;

use iocraft::prelude::*;
use parser::ProjectParser;
use types::SessionMessage;

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    List,
    Detail(usize),
}

#[derive(Clone)]
struct SearchResult {
    project_name: String,
    messages: Vec<SessionMessage>,
}

#[component]
fn ClaudeProjectsViewer(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let mut should_exit = hooks.use_state(|| false);
    
    // Load all projects on startup
    let projects = ProjectParser::load_all_projects().unwrap_or_default();
    
    let mut query = hooks.use_state(String::new);
    let mut selected_index = hooks.use_state(|| 0);
    let mut view_mode = hooks.use_state(|| ViewMode::List);
    let mut search_has_focus = hooks.use_state(|| false);
    
    // Search results - only calculate when query changes
    let query_str = query.to_string();
    let search_results: Vec<SearchResult> = if query_str.is_empty() {
        Vec::new()
    } else {
        ProjectParser::search_messages(&projects, &query_str)
            .into_iter()
            .map(|(project_name, messages)| SearchResult {
                project_name,
                messages,
            })
            .collect()
    };
    
    // Reset selected index if it's out of bounds
    if selected_index.get() >= search_results.len() && !search_results.is_empty() {
        selected_index.set(0);
    }
    
    // Handle keyboard events
    hooks.use_terminal_events({
        let results_len = search_results.len();
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, modifiers, .. }) if kind != KeyEventKind::Release => {
                // Only handle special keys if search doesn't have focus or if it's a control key
                if !search_has_focus.get() || modifiers.contains(KeyModifiers::CONTROL) {
                    match code {
                        KeyCode::Esc => {
                            if view_mode.get() != ViewMode::List {
                                view_mode.set(ViewMode::List);
                            } else {
                                should_exit.set(true);
                            }
                        }
                        KeyCode::Up => {
                            if view_mode.get() == ViewMode::List && results_len > 0 {
                                let current = selected_index.get();
                                selected_index.set(if current > 0 { current - 1 } else { results_len - 1 });
                            }
                        }
                        KeyCode::Down => {
                            if view_mode.get() == ViewMode::List && results_len > 0 {
                                let current = selected_index.get();
                                selected_index.set((current + 1) % results_len);
                            }
                        }
                        KeyCode::Enter => {
                            if view_mode.get() == ViewMode::List && results_len > 0 {
                                view_mode.set(ViewMode::Detail(selected_index.get()));
                            }
                        }
                        KeyCode::Tab => {
                            // Toggle focus between search and list
                            search_has_focus.set(!search_has_focus.get());
                        }
                        _ => {}
                    }
                } else if search_has_focus.get() {
                    // Special handling for ESC when search has focus
                    if matches!(code, KeyCode::Esc) {
                        search_has_focus.set(false);
                    }
                }
            }
            _ => {}
        }
    });
    
    // Handle exit
    if should_exit.get() {
        let mut system = hooks.use_context_mut::<SystemContext>();
        system.exit();
    }
    
    element! {
        View(width: width, height: height, padding: 2) {
            #(match view_mode.get() {
                ViewMode::List => {
                    element! {
                        View(flex_direction: FlexDirection::Column, width: 100pct) {
                            // Title
                            View(margin_bottom: 1) {
                                Text(
                                    content: format!("Claude Projects Viewer ({} projects loaded)", projects.len()),
                                    color: Color::White,
                                    weight: Weight::Bold,
                                    align: TextAlign::Center,
                                )
                            }
                            
                            // Search Bar
                            View(
                                border_style: BorderStyle::Round,
                                border_color: if search_has_focus.get() { Color::Blue } else { Color::Grey },
                                padding: 1,
                                margin_bottom: 1,
                                width: 100pct,
                            ) {
                                View(flex_direction: FlexDirection::Row) {
                                    Text(content: "Search: ", color: Color::Yellow)
                                    View(flex_grow: 1.0, background_color: Color::Black, padding_left: 1) {
                                        TextInput(
                                            value: query.to_string(),
                                            on_change: move |new_value| {
                                                query.set(new_value);
                                            },
                                            has_focus: search_has_focus.get(),
                                        )
                                    }
                                }
                            }
                            
                            // Instructions
                            View(margin_bottom: 1) {
                                Text(
                                    content: "Tab: toggle search focus | ↑/↓: navigate | Enter: view | ESC: back/exit",
                                    color: Color::Grey,
                                )
                            }
                            
                            // Search Results
                            #(if query_str.is_empty() {
                                element! {
                                    View(padding: 2) {
                                        Text(
                                            content: "Enter a search query to find messages",
                                            color: Color::DarkYellow,
                                            align: TextAlign::Center,
                                        )
                                    }
                                }
                            } else if search_results.is_empty() {
                                element! {
                                    View(padding: 2) {
                                        Text(content: "No messages found", color: Color::Yellow)
                                    }
                                }
                            } else {
                                element! {
                                    View(
                                        flex_direction: FlexDirection::Column,
                                        border_style: BorderStyle::Round,
                                        border_color: Color::Cyan,
                                        width: 100pct,
                                    ) {
                                        // Table Header
                                        View(
                                            border_style: BorderStyle::Single,
                                            border_edges: Edges::Bottom,
                                            border_color: Color::Grey,
                                            padding: 1,
                                        ) {
                                            View(width: 40pct) {
                                                Text(content: "Project", weight: Weight::Bold, decoration: TextDecoration::Underline)
                                            }
                                            View(width: 20pct) {
                                                Text(content: "Messages", weight: Weight::Bold, decoration: TextDecoration::Underline)
                                            }
                                            View(width: 40pct) {
                                                Text(content: "Preview", weight: Weight::Bold, decoration: TextDecoration::Underline)
                                            }
                                        }
                                        
                                        // Table Rows
                                        #(search_results.iter().enumerate().map(|(i, result)| {
                                            let is_selected = i == selected_index.get();
                                            let preview = result.messages.first()
                                                .map(|msg| {
                                                    let texts = msg.get_text_content();
                                                    texts.first()
                                                        .map(|text| {
                                                            let text = text.chars().take(50).collect::<String>();
                                                            if text.len() == 50 {
                                                                format!("{text}...")
                                                            } else {
                                                                text
                                                            }
                                                        })
                                                        .unwrap_or_else(|| "No preview available".to_string())
                                                })
                                                .unwrap_or_else(|| "No preview available".to_string());
                                            
                                            element! {
                                                View(
                                                    padding: 1,
                                                    background_color: if is_selected { Color::DarkGrey } else if i % 2 == 0 { Color::Reset } else { Color::Black },
                                                    border_style: if is_selected { BorderStyle::Round } else { BorderStyle::None },
                                                    border_color: if is_selected { Color::Green } else { Color::Reset },
                                                ) {
                                                    View(width: 40pct) {
                                                        Text(
                                                            content: &result.project_name,
                                                            color: if is_selected { Color::White } else { Color::Reset },
                                                            weight: if is_selected { Weight::Bold } else { Weight::Normal },
                                                        )
                                                    }
                                                    View(width: 20pct) {
                                                        Text(
                                                            content: format!("{} found", result.messages.len()),
                                                            color: Color::Cyan,
                                                        )
                                                    }
                                                    View(width: 40pct) {
                                                        Text(
                                                            content: &preview,
                                                            color: Color::DarkGrey,
                                                        )
                                                    }
                                                }
                                            }
                                        }))
                                    }
                                }
                            })
                        }
                    }
                }
                ViewMode::Detail(index) => {
                    if let Some(result) = search_results.get(index) {
                        element! {
                            View(
                                flex_direction: FlexDirection::Column,
                                width: 100pct,
                            ) {
                                // Header
                                View(
                                    border_style: BorderStyle::Round,
                                    border_color: Color::Blue,
                                    padding: 1,
                                    margin_bottom: 1,
                                ) {
                                    View(flex_direction: FlexDirection::Column) {
                                        View(margin_bottom: 1) {
                                            Text(
                                                content: &result.project_name,
                                                color: Color::White,
                                                weight: Weight::Bold,
                                            )
                                        }
                                        Text(
                                            content: format!("{} messages found", result.messages.len()),
                                            color: Color::Cyan,
                                        )
                                    }
                                }
                                
                                // Messages list
                                View(
                                    flex_direction: FlexDirection::Column,
                                    height: height.saturating_sub(10),
                                    overflow: Overflow::Scroll,
                                ) {
                                    #(result.messages.iter().enumerate().map(|(i, msg)| {
                                        let msg_type = msg.get_type();
                                        let timestamp = msg.get_timestamp().unwrap_or("N/A");
                                        let content_preview = msg.get_text_content()
                                            .join(" ")
                                            .chars()
                                            .take(200)
                                            .collect::<String>();
                                        
                                        element! {
                                            View(
                                                border_style: BorderStyle::Single,
                                                border_color: match msg_type {
                                                    "user" => Color::Green,
                                                    "assistant" => Color::Blue,
                                                    "system" => Color::Yellow,
                                                    _ => Color::Grey,
                                                },
                                                padding: 1,
                                                margin_bottom: 1,
                                            ) {
                                                View(flex_direction: FlexDirection::Column) {
                                                    View(margin_bottom: 1) {
                                                        Text(
                                                            content: format!("[{}] {}", i + 1, msg_type),
                                                            color: match msg_type {
                                                                "user" => Color::Green,
                                                                "assistant" => Color::Blue,
                                                                "system" => Color::Yellow,
                                                                _ => Color::Grey,
                                                            },
                                                            weight: Weight::Bold,
                                                        )
                                                        Text(
                                                            content: format!(" - {}", timestamp),
                                                            color: Color::DarkGrey,
                                                        )
                                                    }
                                                    Text(
                                                        content: &content_preview,
                                                        wrap: TextWrap::Wrap,
                                                    )
                                                }
                                            }
                                        }
                                    }))
                                }
                                
                                // Footer
                                View(margin_top: 1) {
                                    Text(content: "Press ESC to go back", color: Color::Grey)
                                }
                            }
                        }
                    } else {
                        element! {
                            View {
                                Text(content: "Result not found", color: Color::Red)
                            }
                        }
                    }
                }
            })
        }
    }
}

fn main() {
    // Set up panic handler to capture crash details
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("PANIC: {panic_info}");
        if let Some(location) = panic_info.location() {
            eprintln!("Panic occurred at {}:{}:{}", 
                location.file(), 
                location.line(), 
                location.column()
            );
        }
    }));
    
    smol::block_on(element!(ClaudeProjectsViewer).fullscreen()).unwrap();
}