//! Shopping Cart Example
//!
//! An e-commerce shopping cart demonstrating list management,
//! dynamic UI updates, and calculations.
//!
//! Run with: cargo run --example shopping_cart

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

/// Product information
#[derive(Clone)]
struct Product {
    id: usize,
    name: String,
    description: String,
    price: f64,
    category: String,
    rating: f32,
}

impl Product {
    fn new(id: usize, name: &str, description: &str, price: f64, category: &str, rating: f32) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            price,
            category: category.to_string(),
            rating,
        }
    }
}

/// Cart item
#[derive(Clone)]
struct CartItem {
    product: Product,
    quantity: u32,
}

impl CartItem {
    fn total_price(&self) -> f64 {
        self.product.price * self.quantity as f64
    }
}

/// Shopping cart state
struct ShoppingCartState {
    available_products: Vec<Product>,
    cart_items: Vec<CartItem>,
    discount_code: String,
    discount_percent: f64,
}

impl ShoppingCartState {
    fn new() -> Self {
        Self {
            available_products: vec![
                Product::new(1, "Wireless Headphones", "Premium noise-cancelling headphones", 199.99, "Electronics", 4.5),
                Product::new(2, "Smart Watch", "Fitness tracking and notifications", 299.99, "Electronics", 4.7),
                Product::new(3, "Laptop Backpack", "Durable and water-resistant", 49.99, "Accessories", 4.3),
                Product::new(4, "USB-C Hub", "7-in-1 connectivity solution", 39.99, "Accessories", 4.6),
                Product::new(5, "Mechanical Keyboard", "RGB backlit gaming keyboard", 129.99, "Electronics", 4.8),
                Product::new(6, "Wireless Mouse", "Ergonomic design with precision tracking", 59.99, "Electronics", 4.4),
                Product::new(7, "Phone Stand", "Adjustable aluminum phone holder", 24.99, "Accessories", 4.2),
                Product::new(8, "Screen Protector", "Tempered glass protection", 14.99, "Accessories", 4.0),
            ],
            cart_items: Vec::new(),
            discount_code: String::new(),
            discount_percent: 0.0,
        }
    }

    fn add_to_cart(&mut self, product: Product) {
        if let Some(item) = self.cart_items.iter_mut().find(|i| i.product.id == product.id) {
            item.quantity += 1;
        } else {
            self.cart_items.push(CartItem {
                product,
                quantity: 1,
            });
        }
    }

    fn remove_from_cart(&mut self, product_id: usize) {
        self.cart_items.retain(|item| item.product.id != product_id);
    }

    fn update_quantity(&mut self, product_id: usize, quantity: u32) {
        if let Some(item) = self.cart_items.iter_mut().find(|i| i.product.id == product_id) {
            if quantity == 0 {
                self.remove_from_cart(product_id);
            } else {
                item.quantity = quantity;
            }
        }
    }

    fn subtotal(&self) -> f64 {
        self.cart_items.iter().map(|item| item.total_price()).sum()
    }

    fn tax(&self) -> f64 {
        self.subtotal() * 0.08 // 8% tax
    }

    fn discount_amount(&self) -> f64 {
        self.subtotal() * (self.discount_percent / 100.0)
    }

    fn total(&self) -> f64 {
        self.subtotal() + self.tax() - self.discount_amount()
    }

    fn apply_discount(&mut self, code: &str) {
        self.discount_code = code.to_string();
        self.discount_percent = match code.to_uppercase().as_str() {
            "SAVE10" => 10.0,
            "SAVE20" => 20.0,
            "WELCOME" => 15.0,
            _ => 0.0,
        };
    }

    fn item_count(&self) -> u32 {
        self.cart_items.iter().map(|item| item.quantity).sum()
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Shopping Cart")
        .size(1100, 750)
        .build()?;

    let state = Arc::new(RwLock::new(ShoppingCartState::new()));

    let content = build_shop_ui(state)?;

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}

fn build_shop_ui(state: Arc<RwLock<ShoppingCartState>>) -> Result<UIElement> {
    Ok(Grid::new()
        .columns(vec![
            ColumnDefinition::star(2.0), // Products
            ColumnDefinition::star(1.0), // Cart
        ])
        .child_at(build_products_panel(state.clone()), 0, 0)
        .child_at(build_cart_panel(state), 0, 1)
        .into())
}

fn build_products_panel(state: Arc<RwLock<ShoppingCartState>>) -> UIElement {
    let products = state.read().available_products.clone();

    Border::new()
        .background(&Brush::from_color(Color::rgb(245, 245, 245)))
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .child(
                    Border::new()
                        .background(&Brush::from_color(Color::rgb(0, 120, 215)))
                        .padding(Thickness::symmetric(30.0, 20.0))
                        .child(
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(15.0)
                                .child(
                                    TextBlock::new()
                                        .text("🛒")
                                        .font_size(28.0),
                                )
                                .child(
                                    TextBlock::new()
                                        .text("Tech Store")
                                        .font_size(28.0)
                                        .font_weight(FontWeight::Bold)
                                        .foreground(&Brush::white()),
                                ),
                        ),
                )
                .child(
                    ScrollViewer::new()
                        .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
                        .padding_uniform(30.0)
                        .content({
                            let mut grid = Grid::new()
                                .columns(vec![
                                    ColumnDefinition::star(1.0),
                                    ColumnDefinition::star(1.0),
                                ])
                                .column_spacing(20.0)
                                .row_spacing(20.0);

                            let rows = (products.len() + 1) / 2;
                            grid = grid.rows((0..rows).map(|_| RowDefinition::auto()).collect());

                            for (idx, product) in products.iter().enumerate() {
                                let row = idx / 2;
                                let col = idx % 2;
                                grid = grid.child_at(build_product_card(product, state.clone()), row, col);
                            }

                            grid
                        }),
                ),
        )
        .into()
}

fn build_product_card(product: &Product, state: Arc<RwLock<ShoppingCartState>>) -> UIElement {
    let product_clone = product.clone();

    Border::new()
        .background(&Brush::white())
        .corner_radius_uniform(12.0)
        .border_thickness_uniform(1.0)
        .border_brush(&Brush::from_color(Color::rgb(220, 220, 220)))
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .child(
                    Border::new()
                        .background(&Brush::from_color(Color::rgb(240, 240, 240)))
                        .height(150.0)
                        .corner_radius(CornerRadius::new(12.0, 12.0, 0.0, 0.0))
                        .child(
                            StackPanel::new()
                                .orientation(Orientation::Vertical)
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .child(
                                    TextBlock::new()
                                        .text("📦")
                                        .font_size(64.0),
                                ),
                        ),
                )
                .child(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .padding_uniform(15.0)
                        .spacing(10.0)
                        // Category badge
                        .child(
                            Border::new()
                                .background(&Brush::from_color(Color::rgb(230, 240, 255)))
                                .corner_radius_uniform(4.0)
                                .padding(Thickness::symmetric(8.0, 4.0))
                                .horizontal_alignment(HorizontalAlignment::Left)
                                .child(
                                    TextBlock::new()
                                        .text(&product.category)
                                        .font_size(10.0)
                                        .foreground(&Brush::from_color(Color::rgb(0, 100, 180))),
                                ),
                        )
                        // Title
                        .child(
                            TextBlock::new()
                                .text(&product.name)
                                .font_size(16.0)
                                .font_weight(FontWeight::SemiBold)
                                .text_wrapping(TextWrapping::Wrap),
                        )
                        // Description
                        .child(
                            TextBlock::new()
                                .text(&product.description)
                                .font_size(12.0)
                                .foreground(&Brush::from_color(Color::rgb(100, 100, 100)))
                                .text_wrapping(TextWrapping::Wrap)
                                .max_lines(2),
                        )
                        // Rating
                        .child(
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(5.0)
                                .child(
                                    TextBlock::new()
                                        .text("⭐")
                                        .font_size(14.0),
                                )
                                .child(
                                    TextBlock::new()
                                        .text(&format!("{:.1}", product.rating))
                                        .font_size(13.0)
                                        .font_weight(FontWeight::Medium),
                                ),
                        )
                        // Price and button
                        .child(
                            Grid::new()
                                .columns(vec![
                                    ColumnDefinition::star(1.0),
                                    ColumnDefinition::auto(),
                                ])
                                .margin(Thickness::new(0.0, 5.0, 0.0, 0.0))
                                .child_at(
                                    TextBlock::new()
                                        .text(&format!("${:.2}", product.price))
                                        .font_size(20.0)
                                        .font_weight(FontWeight::Bold)
                                        .foreground(&Brush::from_color(Color::rgb(0, 120, 215)))
                                        .vertical_alignment(VerticalAlignment::Center),
                                    0, 0
                                )
                                .child_at(
                                    Button::new()
                                        .content("Add to Cart")
                                        .padding(Thickness::symmetric(15.0, 8.0))
                                        .font_size(12.0)
                                        .on_click(move |_| {
                                            state.write().add_to_cart(product_clone.clone());
                                            println!("Added {} to cart", product_clone.name);
                                        }),
                                    0, 1
                                ),
                        ),
                ),
        )
        .into()
}

fn build_cart_panel(state: Arc<RwLock<ShoppingCartState>>) -> UIElement {
    let s = state.read();
    let items = s.cart_items.clone();
    let subtotal = s.subtotal();
    let tax = s.tax();
    let discount = s.discount_amount();
    let total = s.total();
    let item_count = s.item_count();

    Border::new()
        .background(&Brush::white())
        .border_thickness(Thickness::new(1.0, 0.0, 0.0, 0.0))
        .border_brush(&Brush::from_color(Color::rgb(220, 220, 220)))
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                // Header
                .child(
                    Border::new()
                        .background(&Brush::from_color(Color::rgb(250, 250, 250)))
                        .padding(Thickness::symmetric(20.0, 15.0))
                        .border_thickness(Thickness::new(0.0, 0.0, 0.0, 1.0))
                        .border_brush(&Brush::from_color(Color::rgb(230, 230, 230)))
                        .child(
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(10.0)
                                .child(
                                    TextBlock::new()
                                        .text("🛒")
                                        .font_size(22.0),
                                )
                                .child(
                                    TextBlock::new()
                                        .text("Shopping Cart")
                                        .font_size(18.0)
                                        .font_weight(FontWeight::SemiBold),
                                )
                                .child(
                                    Border::new()
                                        .background(&Brush::from_color(Color::rgb(0, 120, 215)))
                                        .corner_radius_uniform(10.0)
                                        .padding(Thickness::symmetric(8.0, 4.0))
                                        .child(
                                            TextBlock::new()
                                                .text(&item_count.to_string())
                                                .font_size(12.0)
                                                .foreground(&Brush::white()),
                                        ),
                                ),
                        ),
                )
                // Cart items
                .child(
                    ScrollViewer::new()
                        .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
                        .height(300.0)
                        .content(build_cart_items_list(items, state.clone())),
                )
                // Discount code
                .child(build_discount_section(state.clone()))
                // Summary
                .child(
                    Border::new()
                        .background(&Brush::from_color(Color::rgb(250, 250, 250)))
                        .padding_uniform(20.0)
                        .border_thickness(Thickness::new(0.0, 1.0, 0.0, 1.0))
                        .border_brush(&Brush::from_color(Color::rgb(230, 230, 230)))
                        .child(
                            StackPanel::new()
                                .orientation(Orientation::Vertical)
                                .spacing(12.0)
                                .child(build_summary_row("Subtotal:", &format!("${:.2}", subtotal)))
                                .child(build_summary_row("Tax (8%):", &format!("${:.2}", tax)))
                                .child(if discount > 0.0 {
                                    build_summary_row("Discount:", &format!("-${:.2}", discount))
                                } else {
                                    Border::new().into()
                                })
                                .child(
                                    Border::new()
                                        .height(1.0)
                                        .background(&Brush::from_color(Color::rgb(220, 220, 220)))
                                        .margin(Thickness::symmetric(0.0, 5.0)),
                                )
                                .child(
                                    Grid::new()
                                        .columns(vec![
                                            ColumnDefinition::star(1.0),
                                            ColumnDefinition::auto(),
                                        ])
                                        .child_at(
                                            TextBlock::new()
                                                .text("Total:")
                                                .font_size(18.0)
                                                .font_weight(FontWeight::Bold),
                                            0, 0
                                        )
                                        .child_at(
                                            TextBlock::new()
                                                .text(&format!("${:.2}", total))
                                                .font_size(24.0)
                                                .font_weight(FontWeight::Bold)
                                                .foreground(&Brush::from_color(Color::rgb(0, 120, 215))),
                                            0, 1
                                        ),
                                ),
                        ),
                )
                // Checkout button
                .child(
                    Border::new()
                        .padding_uniform(20.0)
                        .child(
                            Button::new()
                                .content("Proceed to Checkout")
                                .padding(Thickness::symmetric(30.0, 15.0))
                                .font_size(16.0)
                                .horizontal_alignment(HorizontalAlignment::Stretch)
                                .enabled(!items.is_empty())
                                .on_click(move |_| {
                                    println!("=== Checkout ===");
                                    println!("Total: ${:.2}", total);
                                    println!("================");
                                }),
                        ),
                ),
        )
        .into()
}

fn build_cart_items_list(items: Vec<CartItem>, state: Arc<RwLock<ShoppingCartState>>) -> UIElement {
    let mut panel = StackPanel::new()
        .orientation(Orientation::Vertical);

    if items.is_empty() {
        panel = panel.child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(15.0)
                .padding_uniform(40.0)
                .child(
                    TextBlock::new()
                        .text("🛒")
                        .font_size(64.0)
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                .child(
                    TextBlock::new()
                        .text("Your cart is empty")
                        .font_size(16.0)
                        .foreground(&Brush::from_color(Color::GRAY))
                        .horizontal_alignment(HorizontalAlignment::Center),
                ),
        );
    } else {
        for item in items {
            panel = panel.child(build_cart_item(&item, state.clone()));
        }
    }

    panel.into()
}

fn build_cart_item(item: &CartItem, state: Arc<RwLock<ShoppingCartState>>) -> UIElement {
    let product_id = item.product.id;
    let quantity = item.quantity;

    Border::new()
        .padding_uniform(15.0)
        .border_thickness(Thickness::new(0.0, 0.0, 0.0, 1.0))
        .border_brush(&Brush::from_color(Color::rgb(240, 240, 240)))
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(10.0)
                // Title and price
                .child(
                    Grid::new()
                        .columns(vec![
                            ColumnDefinition::star(1.0),
                            ColumnDefinition::auto(),
                        ])
                        .child_at(
                            TextBlock::new()
                                .text(&item.product.name)
                                .font_size(14.0)
                                .font_weight(FontWeight::Medium)
                                .text_wrapping(TextWrapping::Wrap),
                            0, 0
                        )
                        .child_at(
                            Button::new()
                                .content("🗑️")
                                .padding(Thickness::symmetric(8.0, 4.0))
                                .on_click(move |_| {
                                    state.write().remove_from_cart(product_id);
                                    println!("Removed item from cart");
                                }),
                            0, 1
                        ),
                )
                // Quantity and total
                .child(
                    Grid::new()
                        .columns(vec![
                            ColumnDefinition::auto(),
                            ColumnDefinition::star(1.0),
                            ColumnDefinition::auto(),
                        ])
                        .column_spacing(10.0)
                        // Quantity controls
                        .child_at(
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(8.0)
                                .child(
                                    Button::new()
                                        .content("-")
                                        .width(30.0)
                                        .padding(Thickness::symmetric(5.0, 2.0))
                                        .on_click(move |_| {
                                            let state_dec = state.clone();
                                            let new_qty = quantity.saturating_sub(1);
                                            state_dec.write().update_quantity(product_id, new_qty);
                                        }),
                                )
                                .child(
                                    TextBlock::new()
                                        .text(&quantity.to_string())
                                        .font_size(14.0)
                                        .width(30.0)
                                        .horizontal_alignment(HorizontalAlignment::Center),
                                )
                                .child(
                                    Button::new()
                                        .content("+")
                                        .width(30.0)
                                        .padding(Thickness::symmetric(5.0, 2.0))
                                        .on_click(move |_| {
                                            let state_inc = state.clone();
                                            state_inc.write().update_quantity(product_id, quantity + 1);
                                        }),
                                ),
                            0, 0
                        )
                        // Price
                        .child_at(
                            TextBlock::new()
                                .text(&format!("${:.2}", item.total_price()))
                                .font_size(16.0)
                                .font_weight(FontWeight::SemiBold)
                                .foreground(&Brush::from_color(Color::rgb(0, 120, 215)))
                                .horizontal_alignment(HorizontalAlignment::Right),
                            0, 2
                        ),
                ),
        )
        .into()
}

fn build_discount_section(state: Arc<RwLock<ShoppingCartState>>) -> UIElement {
    let state_apply = state.clone();

    Border::new()
        .padding_uniform(20.0)
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(10.0)
                .child(
                    TextBlock::new()
                        .text("Have a discount code?")
                        .font_size(13.0)
                        .font_weight(FontWeight::Medium),
                )
                .child(
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(8.0)
                        .child(
                            TextBox::new()
                                .placeholder("Enter code")
                                .font_size(13.0)
                                .width(180.0)
                                .on_text_changed(move |text| {
                                    // Store the code for later
                                }),
                        )
                        .child(
                            Button::new()
                                .content("Apply")
                                .padding(Thickness::symmetric(15.0, 8.0))
                                .font_size(13.0)
                                .on_click(move |_| {
                                    state_apply.write().apply_discount("SAVE10");
                                    println!("Discount applied!");
                                }),
                        ),
                )
                .child(
                    TextBlock::new()
                        .text("Try: SAVE10, SAVE20, or WELCOME")
                        .font_size(10.0)
                        .foreground(&Brush::from_color(Color::GRAY)),
                ),
        )
        .into()
}

fn build_summary_row(label: &str, value: &str) -> UIElement {
    Grid::new()
        .columns(vec![
            ColumnDefinition::star(1.0),
            ColumnDefinition::auto(),
        ])
        .child_at(
            TextBlock::new()
                .text(label)
                .font_size(14.0),
            0, 0
        )
        .child_at(
            TextBlock::new()
                .text(value)
                .font_size(14.0)
                .font_weight(FontWeight::Medium),
            0, 1
        )
        .into()
}

