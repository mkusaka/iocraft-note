mod parser;
mod types;

use iocraft::prelude::*;
use parser::ProjectParser;
use types::SessionMessage;
use std::env;

#[derive(Clone)]
struct SearchResult {
    project_name: String,
    messages: Vec<SessionMessage>,
}

fn main() {
    // コマンドライン引数から検索クエリを取得
    let args: Vec<String> = env::args().collect();
    let query = args.get(1).map(|s| s.as_str()).unwrap_or("");
    
    // Load all projects
    let projects = match ProjectParser::load_all_projects() {
        Ok(projects) => projects,
        Err(e) => {
            eprintln!("Error loading projects: {}", e);
            std::process::exit(1);
        }
    };
    
    // Search results
    let search_results: Vec<SearchResult> = if query.is_empty() {
        // クエリがない場合は全プロジェクトを表示
        projects.iter()
            .map(|project| SearchResult {
                project_name: project.project_name.clone(),
                messages: project.messages.clone(),
            })
            .collect()
    } else {
        ProjectParser::search_messages(&projects, query)
            .into_iter()
            .map(|(project_name, messages)| SearchResult {
                project_name,
                messages,
            })
            .collect()
    };
    
    // Create UI
    let mut ui = element! {
        View(flex_direction: FlexDirection::Column, padding: 1) {
            // Title
            View(
                border_style: BorderStyle::Double,
                border_color: Color::Blue,
                padding: 1,
                margin_bottom: 1,
            ) {
                View(flex_direction: FlexDirection::Column) {
                    Text(
                        content: format!("Claude Projects Viewer (Latest {} projects)", projects.len()),
                        color: Color::White,
                        weight: Weight::Bold,
                        align: TextAlign::Center,
                    )
                    #(if !query.is_empty() {
                        element! {
                            View(margin_top: 1) {
                                Text(
                                    content: format!("Search: {}", query),
                                    color: Color::Yellow,
                                    align: TextAlign::Center,
                                )
                            }
                        }
                    } else {
                        element! { View {} }
                    })
                }
            }
            
            // Results
            #(if search_results.is_empty() {
                element! {
                    View(padding: 2) {
                        Text(
                            content: if query.is_empty() {
                                "No projects found"
                            } else {
                                "No matching projects found"
                            },
                            color: Color::Yellow,
                            align: TextAlign::Center,
                        )
                    }
                }
            } else {
                element! {
                    View(flex_direction: FlexDirection::Column) {
                        // Summary
                        View(margin_bottom: 1) {
                            Text(
                                content: if query.is_empty() {
                                    format!("Showing all {} projects", search_results.len())
                                } else {
                                    format!("Found {} matching projects", search_results.len())
                                },
                                color: Color::Cyan,
                            )
                        }
                        
                        // Results list
                        View(flex_direction: FlexDirection::Column) {
                            #(search_results.iter().enumerate().map(|(i, result)| {
                                let total_messages = result.messages.len();
                                let preview = result.messages.first()
                                    .map(|msg| {
                                        let texts = msg.get_text_content();
                                        texts.first()
                                            .map(|text| {
                                                let text = text.chars().take(60).collect::<String>();
                                                if text.len() == 60 {
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
                                        border_style: BorderStyle::Single,
                                        border_color: Color::DarkGrey,
                                        padding: 1,
                                        margin_bottom: 1,
                                    ) {
                                        View(flex_direction: FlexDirection::Column) {
                                            // Project name and message count
                                            View(flex_direction: FlexDirection::Row, margin_bottom: 1) {
                                                Text(
                                                    content: format!("{}. ", i + 1),
                                                    color: Color::DarkGrey,
                                                )
                                                Text(
                                                    content: &result.project_name,
                                                    color: Color::White,
                                                    weight: Weight::Bold,
                                                )
                                                Text(
                                                    content: format!(
                                                        " ({} {})",
                                                        total_messages,
                                                        if query.is_empty() { "messages" } else { "matches" }
                                                    ),
                                                    color: Color::Cyan,
                                                )
                                            }
                                            // Preview
                                            Text(
                                                content: format!("  {}", preview),
                                                color: Color::DarkGrey,
                                            )
                                            // Show first few matches if searching
                                            #(if !query.is_empty() && result.messages.len() > 0 {
                                                element! {
                                                    View(margin_top: 1, padding_left: 2) {
                                                        View(flex_direction: FlexDirection::Column) {
                                                            #(result.messages.iter().take(3).enumerate().map(|(j, msg)| {
                                                                let msg_type = msg.get_type();
                                                                let content = msg.get_text_content()
                                                                    .join(" ")
                                                                    .chars()
                                                                    .take(100)
                                                                    .collect::<String>();
                                                                
                                                                element! {
                                                                    View(margin_bottom: if j < 2 { 1 } else { 0 }) {
                                                                        Text(
                                                                            content: format!("• [{}] {}", msg_type, content),
                                                                            color: match msg_type {
                                                                                "user" => Color::Green,
                                                                                "assistant" => Color::Blue,
                                                                                "system" => Color::Yellow,
                                                                                _ => Color::Grey,
                                                                            },
                                                                        )
                                                                    }
                                                                }
                                                            }))
                                                            #(if result.messages.len() > 3 {
                                                                element! {
                                                                    View(margin_top: 1) {
                                                                        Text(
                                                                            content: format!("  ... and {} more matches", result.messages.len() - 3),
                                                                            color: Color::DarkGrey,
                                                                        )
                                                                    }
                                                                }
                                                            } else {
                                                                element! { View {} }
                                                            })
                                                        }
                                                    }
                                                }
                                            } else {
                                                element! { View {} }
                                            })
                                        }
                                    }
                                }
                            }))
                        }
                    }
                }
            })
            
            // Usage help
            View(
                margin_top: 1,
                border_style: BorderStyle::Single,
                border_color: Color::DarkGrey,
                padding: 1,
            ) {
                View(flex_direction: FlexDirection::Column) {
                    Text(content: "Usage:", color: Color::Yellow, weight: Weight::Bold)
                    Text(content: "  claude-projects-viewer-print [search_query]", color: Color::DarkGrey)
                    Text(content: "", color: Color::DarkGrey)
                    Text(content: "Examples:", color: Color::Yellow)
                    Text(content: "  claude-projects-viewer-print          # Show all projects", color: Color::DarkGrey)
                    Text(content: "  claude-projects-viewer-print rust     # Search for 'rust'", color: Color::DarkGrey)
                }
            }
        }
    };
    
    // 標準出力に描画（選択・コピー可能）
    ui.print();
}