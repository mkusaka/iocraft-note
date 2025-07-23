use iocraft::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Product {
    id: u32,
    name: String,
    category: String,
    price: u32,
    description: String,
    brand: String,
    #[serde(rename = "inStock")]
    in_stock: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    List,
    Detail(usize),
}

#[component]
fn SearchUI(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let mut should_exit = hooks.use_state(|| false);
    
    // Load products from JSON
    let products = {
        let data = fs::read_to_string("data.json").expect("Failed to read data.json");
        serde_json::from_str::<Vec<Product>>(&data).expect("Failed to parse JSON")
    };
    
    let mut query = hooks.use_state(|| String::new());
    let mut selected_index = hooks.use_state(|| 0);
    let mut view_mode = hooks.use_state(|| ViewMode::List);
    let search_has_focus = hooks.use_state(|| true);
    
    // Filter products based on query
    let filtered_products: Vec<Product> = {
        let query_str = query.to_string();
        if query_str.is_empty() {
            products.clone()
        } else {
            let query_lower = query_str.to_lowercase();
            products
                .iter()
                .filter(|p| {
                    p.name.to_lowercase().contains(&query_lower)
                        || p.category.to_lowercase().contains(&query_lower)
                        || p.brand.to_lowercase().contains(&query_lower)
                })
                .cloned()
                .collect()
        }
    };
    
    // Reset selected index if it's out of bounds
    if selected_index.get() >= filtered_products.len() && !filtered_products.is_empty() {
        selected_index.set(0);
    }
    
    // Handle keyboard events
    hooks.use_terminal_events({
        let filtered_products_len = filtered_products.len();
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
                match code {
                    KeyCode::Esc => {
                        if view_mode.get() != ViewMode::List {
                            view_mode.set(ViewMode::List);
                        } else {
                            should_exit.set(true);
                        }
                    }
                    KeyCode::Up => {
                        if view_mode.get() == ViewMode::List && filtered_products_len > 0 {
                            let current = selected_index.get();
                            selected_index.set(if current > 0 { current - 1 } else { filtered_products_len - 1 });
                        }
                    }
                    KeyCode::Down => {
                        if view_mode.get() == ViewMode::List && filtered_products_len > 0 {
                            let current = selected_index.get();
                            selected_index.set((current + 1) % filtered_products_len);
                        }
                    }
                    KeyCode::Enter => {
                        if view_mode.get() == ViewMode::List && filtered_products_len > 0 {
                            view_mode.set(ViewMode::Detail(selected_index.get()));
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
                                    content: "Product Search",
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
                                            on_change: move |new_value| query.set(new_value),
                                            has_focus: search_has_focus.get(),
                                        )
                                    }
                                }
                            }
                            
                            // Instructions
                            View(margin_bottom: 1) {
                                Text(
                                    content: "Use ↑/↓ to navigate, Enter to view details, ESC to exit",
                                    color: Color::Grey,
                                )
                            }
                            
                            // Product Table
                            #(if filtered_products.is_empty() {
                                element! {
                                    View(padding: 2) {
                                        Text(content: "No products found", color: Color::Yellow)
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
                                            View(width: 35pct) {
                                                Text(content: "Product Name", weight: Weight::Bold, decoration: TextDecoration::Underline)
                                            }
                                            View(width: 20pct) {
                                                Text(content: "Brand", weight: Weight::Bold, decoration: TextDecoration::Underline)
                                            }
                                            View(width: 15pct) {
                                                Text(content: "Category", weight: Weight::Bold, decoration: TextDecoration::Underline)
                                            }
                                            View(width: 15pct, justify_content: JustifyContent::End) {
                                                Text(content: "Price", weight: Weight::Bold, decoration: TextDecoration::Underline)
                                            }
                                            View(width: 15pct, justify_content: JustifyContent::Center) {
                                                Text(content: "Stock", weight: Weight::Bold, decoration: TextDecoration::Underline)
                                            }
                                        }
                                        
                                        // Table Rows
                                        #(filtered_products.iter().enumerate().map(|(i, product)| {
                                            let is_selected = i == selected_index.get();
                                            element! {
                                                View(
                                                    padding: 1,
                                                    background_color: if is_selected { Color::DarkGrey } else if i % 2 == 0 { Color::Reset } else { Color::Black },
                                                    border_style: if is_selected { BorderStyle::Round } else { BorderStyle::None },
                                                    border_color: if is_selected { Color::Green } else { Color::Reset },
                                                ) {
                                                    View(width: 35pct) {
                                                        Text(
                                                            content: &product.name,
                                                            color: if is_selected { Color::White } else { Color::Reset },
                                                            weight: if is_selected { Weight::Bold } else { Weight::Normal },
                                                        )
                                                    }
                                                    View(width: 20pct) {
                                                        Text(
                                                            content: &product.brand,
                                                            color: if is_selected { Color::Yellow } else { Color::DarkYellow },
                                                        )
                                                    }
                                                    View(width: 15pct) {
                                                        Text(
                                                            content: &product.category,
                                                            color: Color::Cyan,
                                                        )
                                                    }
                                                    View(width: 15pct, justify_content: JustifyContent::End) {
                                                        Text(
                                                            content: format!("${}", product.price),
                                                            color: Color::Green,
                                                        )
                                                    }
                                                    View(width: 15pct, justify_content: JustifyContent::Center) {
                                                        Text(
                                                            content: if product.in_stock { "✓ In Stock" } else { "✗ Out" },
                                                            color: if product.in_stock { Color::Green } else { Color::Red },
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
                    if let Some(product) = filtered_products.get(index) {
                        element! {
                            View(
                                border_style: BorderStyle::Round,
                                border_color: Color::Blue,
                                padding: 2,
                            ) {
                                View(flex_direction: FlexDirection::Column) {
                                    View(margin_bottom: 1) {
                                        Text(
                                            content: &product.name,
                                            color: Color::White,
                                            weight: Weight::Bold,
                                        )
                                    }
                                    View(margin_bottom: 1) {
                                        Text(content: format!("Brand: {}", product.brand), color: Color::Yellow)
                                    }
                                    View(margin_bottom: 1) {
                                        Text(content: format!("Category: {}", product.category), color: Color::Cyan)
                                    }
                                    View(margin_bottom: 1) {
                                        Text(content: format!("Price: ${}", product.price), color: Color::Green)
                                    }
                                    View(margin_bottom: 1) {
                                        Text(
                                            content: format!("Status: {}", if product.in_stock { "In Stock" } else { "Out of Stock" }),
                                            color: if product.in_stock { Color::Green } else { Color::Red },
                                        )
                                    }
                                    View(margin_bottom: 2) {
                                        View(margin_bottom: 1) {
                                            Text(content: "Description:", color: Color::Grey)
                                        }
                                        Text(content: &product.description)
                                    }
                                    View {
                                        Text(content: "Press ESC to go back", color: Color::Grey)
                                    }
                                }
                            }
                        }
                    } else {
                        element! {
                            View {
                                Text(content: "Product not found", color: Color::Red)
                            }
                        }
                    }
                }
            })
        }
    }
}

fn main() {
    smol::block_on(element!(SearchUI).fullscreen()).unwrap();
}