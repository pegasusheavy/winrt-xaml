//! Shopping cart example.

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Shopping Cart")
        .size(550, 550)
        .build()?;

    let cart = Arc::new(RwLock::new(Vec::<CartItem>::new()));
    let total = Arc::new(RwLock::new(0.0f64));

    // Title
    let title = TextBlock::new()?
        .with_text("Shopping Cart")?;
    title.set_position(190, 20);
    title.set_size(170, 30);

    // Product 1
    let product1 = TextBlock::new()?
        .with_text("Widget A - $19.99")?;
    product1.set_position(50, 80);
    product1.set_size(200, 25);

    let add1 = Button::new()?
        .with_content("Add to Cart")?;
    add1.set_position(270, 75);
    add1.set_size(120, 30);

    let cart_clone = cart.clone();
    let total_clone = total.clone();
    add1.click().subscribe(move |_| {
        cart_clone.write().push(CartItem {
            name: "Widget A".to_string(),
            price: 19.99,
        });
        *total_clone.write() += 19.99;
        println!("Added Widget A to cart");
    });

    // Product 2
    let product2 = TextBlock::new()?
        .with_text("Gadget B - $29.99")?;
    product2.set_position(50, 130);
    product2.set_size(200, 25);

    let add2 = Button::new()?
        .with_content("Add to Cart")?;
    add2.set_position(270, 125);
    add2.set_size(120, 30);

    let cart_clone = cart.clone();
    let total_clone = total.clone();
    add2.click().subscribe(move |_| {
        cart_clone.write().push(CartItem {
            name: "Gadget B".to_string(),
            price: 29.99,
        });
        *total_clone.write() += 29.99;
        println!("Added Gadget B to cart");
    });

    // Product 3
    let product3 = TextBlock::new()?
        .with_text("Doohickey C - $39.99")?;
    product3.set_position(50, 180);
    product3.set_size(200, 25);

    let add3 = Button::new()?
        .with_content("Add to Cart")?;
    add3.set_position(270, 175);
    add3.set_size(120, 30);

    let cart_clone = cart.clone();
    let total_clone = total.clone();
    add3.click().subscribe(move |_| {
        cart_clone.write().push(CartItem {
            name: "Doohickey C".to_string(),
            price: 39.99,
        });
        *total_clone.write() += 39.99;
        println!("Added Doohickey C to cart");
    });

    // Cart summary
    let summary = TextBlock::new()?
        .with_text("Cart: 0 items")?;
    summary.set_position(50, 250);
    summary.set_size(300, 25);

    // Total
    let total_display = TextBlock::new()?
        .with_text("Total: $0.00")?;
    total_display.set_position(50, 290);
    total_display.set_size(300, 30);

    // View cart button
    let view_cart = Button::new()?
        .with_content("View Cart")?;
    view_cart.set_position(50, 350);
    view_cart.set_size(120, 40);

    let cart_clone = cart.clone();
    let total_clone = total.clone();
    let summary_clone = summary.clone();
    let total_display_clone = total_display.clone();
    view_cart.click().subscribe(move |_| {
        let items = cart_clone.read();
        let total_val = *total_clone.read();
        let _ = summary_clone.set_text(&format!("Cart: {} items", items.len()));
        let _ = total_display_clone.set_text(&format!("Total: ${:.2}", total_val));

        println!("\n=== Shopping Cart ===");
        for item in items.iter() {
            println!("{} - ${:.2}", item.name, item.price);
        }
        println!("Total: ${:.2}", total_val);
        println!("====================\n");
    });

    // Clear cart button
    let clear_cart = Button::new()?
        .with_content("Clear Cart")?;
    clear_cart.set_position(190, 350);
    clear_cart.set_size(120, 40);

    let cart_clone = cart.clone();
    let total_clone = total.clone();
    let summary_clone = summary.clone();
    let total_display_clone = total_display.clone();
    clear_cart.click().subscribe(move |_| {
        cart_clone.write().clear();
        *total_clone.write() = 0.0;
        let _ = summary_clone.set_text("Cart: 0 items");
        let _ = total_display_clone.set_text("Total: $0.00");
        println!("Cart cleared");
    });

    // Checkout button
    let checkout = Button::new()?
        .with_content("Checkout")?;
    checkout.set_position(330, 350);
    checkout.set_size(120, 40);

    let cart_clone = cart.clone();
    checkout.click().subscribe(move |_| {
        let items = cart_clone.read();
        if items.is_empty() {
            println!("Cart is empty!");
        } else {
            println!("Proceeding to checkout with {} items", items.len());
        }
    });

    window.set_content(view_cart)?;

    // Show the window
    window.show()?;

    println!("Starting application...");
    app.run()
}

struct CartItem {
    name: String,
    price: f64,
}
