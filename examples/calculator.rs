//! Calculator Application Example
//!
//! A functional calculator demonstrating Grid layout and event handling.
//!
//! Run with: cargo run --example calculator

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

/// Calculator state
struct CalculatorState {
    display: String,
    first_operand: Option<f64>,
    operator: Option<char>,
    waiting_for_second: bool,
}

impl CalculatorState {
    fn new() -> Self {
        Self {
            display: "0".to_string(),
            first_operand: None,
            operator: None,
            waiting_for_second: false,
        }
    }

    fn input_digit(&mut self, digit: char) {
        if self.waiting_for_second {
            self.display = digit.to_string();
            self.waiting_for_second = false;
        } else if self.display == "0" {
            self.display = digit.to_string();
        } else {
            self.display.push(digit);
        }
    }

    fn input_decimal(&mut self) {
        if self.waiting_for_second {
            self.display = "0.".to_string();
            self.waiting_for_second = false;
        } else if !self.display.contains('.') {
            self.display.push('.');
        }
    }

    fn input_operator(&mut self, op: char) {
        let current: f64 = self.display.parse().unwrap_or(0.0);

        if let Some(first) = self.first_operand {
            if let Some(prev_op) = self.operator {
                let result = Self::calculate(first, current, prev_op);
                self.display = Self::format_result(result);
                self.first_operand = Some(result);
            }
        } else {
            self.first_operand = Some(current);
        }

        self.operator = Some(op);
        self.waiting_for_second = true;
    }

    fn calculate_result(&mut self) {
        if let (Some(first), Some(op)) = (self.first_operand, self.operator) {
            let second: f64 = self.display.parse().unwrap_or(0.0);
            let result = Self::calculate(first, second, op);
            self.display = Self::format_result(result);
            self.first_operand = None;
            self.operator = None;
            self.waiting_for_second = true;
        }
    }

    fn calculate(first: f64, second: f64, op: char) -> f64 {
        match op {
            '+' => first + second,
            '-' => first - second,
            '*' => first * second,
            '/' => {
                if second != 0.0 {
                    first / second
                } else {
                    f64::NAN
                }
            }
            _ => second,
        }
    }

    fn format_result(value: f64) -> String {
        if value.is_nan() {
            "Error".to_string()
        } else if value == value.trunc() {
            format!("{:.0}", value)
        } else {
            let formatted = format!("{:.10}", value);
            formatted.trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }

    fn clear(&mut self) {
        self.display = "0".to_string();
        self.first_operand = None;
        self.operator = None;
        self.waiting_for_second = false;
    }

    fn clear_entry(&mut self) {
        self.display = "0".to_string();
    }

    fn negate(&mut self) {
        if self.display != "0" {
            if self.display.starts_with('-') {
                self.display = self.display[1..].to_string();
            } else {
                self.display = format!("-{}", self.display);
            }
        }
    }

    fn percentage(&mut self) {
        if let Ok(value) = self.display.parse::<f64>() {
            self.display = Self::format_result(value / 100.0);
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Calculator")
        .size(320, 480)
        .resizable(false)
        .build()?;

    let state = Arc::new(RwLock::new(CalculatorState::new()));

    let content = build_calculator_ui(state)?;

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}

fn build_calculator_ui(state: Arc<RwLock<CalculatorState>>) -> Result<UIElement> {
    let display_state = state.clone();

    Ok(Border::new()
        .background(&Brush::from_color(Color::rgb(32, 32, 32)))
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .padding_uniform(10.0)
                // Display
                .child(build_display(display_state))
                // Button grid
                .child(build_button_grid(state)),
        )
        .into())
}

fn build_display(state: Arc<RwLock<CalculatorState>>) -> UIElement {
    let display_text = state.read().display.clone();

    Border::new()
        .background(&Brush::from_color(Color::rgb(48, 48, 48)))
        .corner_radius_uniform(5.0)
        .padding(Thickness::new(15.0, 20.0, 15.0, 20.0))
        .margin(Thickness::new(0.0, 0.0, 0.0, 10.0))
        .child(
            TextBlock::new()
                .text(&display_text)
                .font_size(42.0)
                .font_weight(FontWeight::Light)
                .foreground(&Brush::white())
                .horizontal_alignment(HorizontalAlignment::Right)
                .text_alignment(TextAlignment::Right),
        )
        .into()
}

fn build_button_grid(state: Arc<RwLock<CalculatorState>>) -> UIElement {
    // Button definitions: (label, row, col, is_operator, is_wide)
    let buttons = [
        // Row 0
        ("C", 0, 0, false, false),
        ("CE", 0, 1, false, false),
        ("%", 0, 2, false, false),
        ("/", 0, 3, true, false),
        // Row 1
        ("7", 1, 0, false, false),
        ("8", 1, 1, false, false),
        ("9", 1, 2, false, false),
        ("*", 1, 3, true, false),
        // Row 2
        ("4", 2, 0, false, false),
        ("5", 2, 1, false, false),
        ("6", 2, 2, false, false),
        ("-", 2, 3, true, false),
        // Row 3
        ("1", 3, 0, false, false),
        ("2", 3, 1, false, false),
        ("3", 3, 2, false, false),
        ("+", 3, 3, true, false),
        // Row 4
        ("+/-", 4, 0, false, false),
        ("0", 4, 1, false, false),
        (".", 4, 2, false, false),
        ("=", 4, 3, true, false),
    ];

    let mut grid = Grid::new()
        .rows(vec![
            RowDefinition::star(1.0),
            RowDefinition::star(1.0),
            RowDefinition::star(1.0),
            RowDefinition::star(1.0),
            RowDefinition::star(1.0),
        ])
        .columns(vec![
            ColumnDefinition::star(1.0),
            ColumnDefinition::star(1.0),
            ColumnDefinition::star(1.0),
            ColumnDefinition::star(1.0),
        ])
        .row_spacing(5.0)
        .column_spacing(5.0);

    for (label, row, col, is_operator, _is_wide) in buttons {
        let button = create_calc_button(label, is_operator, state.clone());
        grid = grid.child_at(button, row, col);
    }

    grid.into()
}

fn create_calc_button(label: &str, is_operator: bool, state: Arc<RwLock<CalculatorState>>) -> UIElement {
    let label_owned = label.to_string();
    let label_for_click = label.to_string();

    let bg_color = if is_operator {
        Color::rgb(255, 152, 0) // Orange for operators
    } else if label == "C" || label == "CE" || label == "%" || label == "+/-" {
        Color::rgb(80, 80, 80) // Gray for functions
    } else {
        Color::rgb(64, 64, 64) // Dark gray for numbers
    };

    let text_color = if is_operator {
        Color::WHITE
    } else {
        Color::WHITE
    };

    Border::new()
        .background(&Brush::from_color(bg_color))
        .corner_radius_uniform(5.0)
        .child(
            Button::new()
                .content(&label_owned)
                .font_size(20.0)
                .horizontal_alignment(HorizontalAlignment::Stretch)
                .vertical_alignment(VerticalAlignment::Stretch)
                .on_click(move |_| {
                    let mut calc = state.write();
                    match label_for_click.as_str() {
                        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                            calc.input_digit(label_for_click.chars().next().unwrap());
                        }
                        "." => calc.input_decimal(),
                        "+" | "-" | "*" | "/" => {
                            calc.input_operator(label_for_click.chars().next().unwrap());
                        }
                        "=" => calc.calculate_result(),
                        "C" => calc.clear(),
                        "CE" => calc.clear_entry(),
                        "+/-" => calc.negate(),
                        "%" => calc.percentage(),
                        _ => {}
                    }
                    println!("Display: {}", calc.display);
                }),
        )
        .into()
}
