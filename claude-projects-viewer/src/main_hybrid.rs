mod parser;
mod types;

use iocraft::prelude::*;
use parser::ProjectParser;
use types::SessionMessage;
use std::env;

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
    let mut search_has_focus = hooks.use_state(|| true);
    
    // Search results - only calculate when query changes
    let query_str = query.to_string();
    let search_results: Vec<SearchResult> = if query_str.is_empty() {
        // クエリがない場合は全プロジェクトを表示
        projects.iter()
            .map(|project| SearchResult {
                project_name: project.project_name.clone(),
                messages: project.messages.clone(),
            })
            .collect()
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
    
    // Create a state for results count that updates with search results
    let mut results_count_state = hooks.use_state(|| search_results.len());
    results_count_state.set(search_results.len());
    
    // Store query and results length for keyboard handler
    let query_for_handler = query_str.clone();
    let results_len_for_handler = search_results.len();
    
    // Handle keyboard events
    hooks.use_terminal_events({
        let results_count = results_count_state.get();
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, modifiers, .. }) if kind != KeyEventKind::Release => {
                match code {
                    KeyCode::Esc => {
                        if search_has_focus.get() {
                            // 検索フォーカス時はフォーカスを外す
                            search_has_focus.set(false);
                        } else if view_mode.get() != ViewMode::List {
                            view_mode.set(ViewMode::List);
                        } else {
                            should_exit.set(true);
                        }
                    }
                    KeyCode::Tab => {
                        // Toggle focus between search and list
                        search_has_focus.set(!search_has_focus.get());
                    }
                    KeyCode::Up => {
                        // リストフォーカス時のみ動作
                        if !search_has_focus.get() && view_mode.get() == ViewMode::List && results_count > 0 {
                            let current = selected_index.get();
                            selected_index.set(if current > 0 { current - 1 } else { results_count - 1 });
                        }
                    }
                    KeyCode::Down => {
                        // リストフォーカス時のみ動作
                        if !search_has_focus.get() && view_mode.get() == ViewMode::List && results_count > 0 {
                            let current = selected_index.get();
                            selected_index.set((current + 1) % results_count);
                        }
                    }
                    KeyCode::Enter => {
                        // リストフォーカス時のみ動作
                        if !search_has_focus.get() && view_mode.get() == ViewMode::List && results_count > 0 {
                            view_mode.set(ViewMode::Detail(selected_index.get()));
                        }
                    }
                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        // Ctrl+P で現在の画面を静的出力
                        if modifiers.contains(KeyModifiers::CONTROL) {
                            // 現在の画面を標準出力に出力して終了
                            println!("\n=== Claude Projects Viewer Output ===\n");
                            
                            if let ViewMode::List = view_mode.get() {
                                // List view
                                println!("Search: {}", query_for_handler);
                                println!("Results: {} projects\n", results_len_for_handler);
                                
                                // Note: Can't iterate over search_results here, just show count
                                println!("(Use arrow keys to navigate and Enter to view details)")
                            } else if let ViewMode::Detail(_) = view_mode.get() {
                                // Detail view
                                println!("Detail view selected");
                                println!("(Cannot access detail content in export mode)")
                            }
                            
                            println!("\n=== End of Output ===");
                            should_exit.set(true);
                        }
                    }
                    _ => {}
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
                                    content: format!("Claude Projects Viewer (Latest {} projects)", projects.len()),
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
                                background_color: Color::Black,
                            ) {
                                View(flex_direction: FlexDirection::Column) {
                                    View(flex_direction: FlexDirection::Row) {
                                        Text(content: "Search: ", color: Color::Yellow)
                                        View(flex_grow: 1.0) {
                                            TextInput(
                                                value: query.to_string(),
                                                on_change: move |new_value| {
                                                    query.set(new_value);
                                                },
                                                has_focus: search_has_focus.get(),
                                            )
                                        }
                                    }
                                    #(if !query_str.is_empty() {
                                        element! {
                                            View(margin_top: 1) {
                                                Text(
                                                    content: format!("Current query: {}", query_str),
                                                    color: Color::Cyan,
                                                )
                                            }
                                        }
                                    } else {
                                        element! {
                                            View {}
                                        }
                                    })
                                }
                            }
                            
                            // Instructions
                            View(margin_bottom: 1) {
                                Text(
                                    content: if search_has_focus.get() {
                                        "Type to search | ESC: unfocus | Tab: toggle | Ctrl+P: export & exit"
                                    } else {
                                        "Tab: focus search | ↑/↓: navigate | Enter: view | ESC: exit | Ctrl+P: export"
                                    },
                                    color: Color::Grey,
                                )
                            }
                            
                            // Search Results
                            #(if search_results.is_empty() {
                                element! {
                                    View(padding: 2) {
                                        Text(
                                            content: if query_str.is_empty() {
                                                "No projects found"
                                            } else {
                                                "No matching projects found"
                                            },
                                            color: Color::Yellow
                                        )
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
                                                Text(
                                                    content: if query_str.is_empty() { "Total Messages" } else { "Found" },
                                                    weight: Weight::Bold,
                                                    decoration: TextDecoration::Underline
                                                )
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
                                                            content: if query_str.is_empty() {
                                                                format!("{} messages", result.messages.len())
                                                            } else {
                                                                format!("{} found", result.messages.len())
                                                            },
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
                                    Text(content: "Press ESC to go back | Ctrl+P to export", color: Color::Grey)
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
    // コマンドライン引数をチェック
    let args: Vec<String> = env::args().collect();
    let print_mode = args.get(1).map(|s| s == "--print").unwrap_or(false);
    
    if print_mode {
        // 静的出力モード
        println!("Claude Projects Viewer - Print Mode");
        println!("Run without --print for interactive mode\n");
        
        // プロジェクトを読み込んで表示
        match ProjectParser::load_all_projects() {
            Ok(projects) => {
                println!("Loaded {} projects\n", projects.len());
                
                for (i, project) in projects.iter().enumerate() {
                    println!("{}. {} ({} messages)", 
                        i + 1, 
                        project.project_name, 
                        project.messages.len()
                    );
                    
                    if let Some(first_msg) = project.messages.first() {
                        let preview = first_msg.get_text_content()
                            .first()
                            .map(|text| text.chars().take(80).collect::<String>())
                            .unwrap_or_else(|| "No preview".to_string());
                        println!("   {}", preview);
                    }
                    
                    if i < projects.len() - 1 {
                        println!();
                    }
                }
            }
            Err(e) => {
                eprintln!("Error loading projects: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // インタラクティブモード
        smol::block_on(element!(ClaudeProjectsViewer).fullscreen()).unwrap();
    }
}