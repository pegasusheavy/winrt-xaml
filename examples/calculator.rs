//! Calculator example demonstrating button grid and state management.

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

fn main() -> Result<()> {
    env_logger::init();

    println!("Creating calculator...");

    let app = Application::new()?;

    let window = Window::builder()
        .title("Calculator")
        .size(350, 500)
        .build()?;

    // Calculator state
    let calc_state = Arc::new(RwLock::new(CalculatorState::new()));

    // Display
    let display = TextBlock::new()?
        .with_text("0")?;
    display.set_position(20, 20);
    display.set_size(310, 50);

    // Number buttons (0-9)
    let button_positions = [
        (20, 350, "0"), (20, 290, "1"), (90, 290, "2"), (160, 290, "3"),
        (20, 230, "4"), (90, 230, "5"), (160, 230, "6"),
        (20, 170, "7"), (90, 170, "8"), (160, 170, "9"),
    ];

    for (x, y, label) in button_positions {
        let button = Button::new()?
            .with_content(label)?;
        button.set_position(x, y);
        button.set_size(60, 50);

        let state_clone = calc_state.clone();
        let display_clone = display.clone();
        let digit = label.to_string();

        button.click().subscribe(move |_| {
            let mut state = state_clone.write();
            state.append_digit(&digit);
            let _ = display_clone.set_text(&state.display);
            println!("Digit: {}", digit);
        });
    }

    // Operation buttons
    let ops = [
        (230, 170, "+"), (230, 230, "-"),
        (230, 290, "×"), (230, 350, "÷"),
    ];

    for (x, y, op) in ops {
        let button = Button::new()?
            .with_content(op)?;
        button.set_position(x, y);
        button.set_size(60, 50);

        let state_clone = calc_state.clone();
        let display_clone = display.clone();
        let operation = op.to_string();

        button.click().subscribe(move |_| {
            let mut state = state_clone.write();
            state.set_operation(&operation);
            let _ = display_clone.set_text(&state.display);
            println!("Operation: {}", operation);
        });
    }

    // Equals button
    let equals = Button::new()?
        .with_content("=")?;
    equals.set_position(90, 350);
    equals.set_size(130, 50);

    let state_clone = calc_state.clone();
    let display_clone = display.clone();
    equals.click().subscribe(move |_| {
        let mut state = state_clone.write();
        state.calculate();
        let _ = display_clone.set_text(&state.display);
        println!("Result: {}", state.display);
    });

    // Clear button
    let clear = Button::new()?
        .with_content("C")?;
    clear.set_position(20, 110);
    clear.set_size(130, 50);

    let state_clone = calc_state.clone();
    let display_clone = display.clone();
    clear.click().subscribe(move |_| {
        let mut state = state_clone.write();
        state.clear();
        let _ = display_clone.set_text("0");
        println!("Cleared");
    });

    // Decimal button
    let decimal = Button::new()?
        .with_content(".")?;
    decimal.set_position(160, 350);
    decimal.set_size(60, 50);

    let state_clone = calc_state.clone();
    let display_clone = display.clone();
    decimal.click().subscribe(move |_| {
        let mut state = state_clone.write();
        state.append_decimal();
        let _ = display_clone.set_text(&state.display);
    });

    window.set_content(equals)?;

    // Show the window
    window.show()?;

    println!("Starting application...");
    app.run()
}

struct CalculatorState {
    display: String,
    current_value: f64,
    stored_value: f64,
    operation: Option<String>,
    new_number: bool,
}

impl CalculatorState {
    fn new() -> Self {
        Self {
            display: "0".to_string(),
            current_value: 0.0,
            stored_value: 0.0,
            operation: None,
            new_number: true,
        }
    }

    fn append_digit(&mut self, digit: &str) {
        if self.new_number {
            self.display = digit.to_string();
            self.new_number = false;
        } else {
            if self.display == "0" {
                self.display = digit.to_string();
            } else {
                self.display.push_str(digit);
            }
        }
        self.current_value = self.display.parse().unwrap_or(0.0);
    }

    fn append_decimal(&mut self) {
        if !self.display.contains('.') {
            self.display.push('.');
        }
    }

    fn set_operation(&mut self, op: &str) {
        self.stored_value = self.current_value;
        self.operation = Some(op.to_string());
        self.new_number = true;
    }

    fn calculate(&mut self) {
        if let Some(op) = &self.operation {
            let result = match op.as_str() {
                "+" => self.stored_value + self.current_value,
                "-" => self.stored_value - self.current_value,
                "×" => self.stored_value * self.current_value,
                "÷" => {
                    if self.current_value != 0.0 {
                        self.stored_value / self.current_value
                    } else {
                        0.0
                    }
                }
                _ => self.current_value,
            };
            self.current_value = result;
            self.display = format!("{}", result);
            self.operation = None;
            self.new_number = true;
        }
    }

    fn clear(&mut self) {
        self.display = "0".to_string();
        self.current_value = 0.0;
        self.stored_value = 0.0;
        self.operation = None;
        self.new_number = true;
    }
}
