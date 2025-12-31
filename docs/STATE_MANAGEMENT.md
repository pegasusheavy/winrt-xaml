# State Management in WinRT-XAML

This document discusses approaches to state management and UI updates in the WinRT-XAML library.

## Why Not Traditional Data Binding?

Traditional XAML data binding (like in C#/WPF/UWP) relies on:

1. **Reflection** - Not available in Rust
2. **INotifyPropertyChanged** - COM interface requiring complex FFI
3. **Automatic property tracking** - Conflicts with Rust's ownership model
4. **Runtime type inspection** - Not idiomatic in Rust

**Bottom Line**: Traditional binding is possible but adds significant complexity for minimal benefit in Rust.

## Recommended Approaches

### ✅ Option 1: Manual Updates (Current - Simple)

**Best for**: Simple apps, learning, prototypes

```rust
use std::sync::{Arc, Mutex};
use winrt_xaml::prelude::*;

fn create_counter() -> Result<()> {
    // Shared state
    let count = Arc::new(Mutex::new(0));
    
    let text = XamlTextBlock::new()?;
    text.set_text("Count: 0")?;
    
    let button = XamlButton::new()?;
    button.set_content("Increment")?;
    
    // Manual update on click
    button.on_click({
        let count = count.clone();
        let text = text.clone();
        move || {
            let mut c = count.lock().unwrap();
            *c += 1;
            text.set_text(&format!("Count: {}", c))?;
            Ok(())
        }
    })?;
    
    Ok(())
}
```

**Pros:**
- ✅ Simple and explicit
- ✅ No dependencies
- ✅ Works now
- ✅ Easy to understand

**Cons:**
- ❌ Manual updates everywhere
- ❌ Verbose for complex UIs
- ❌ Easy to forget to update UI

### ✅ Option 2: Reactive Signals (Recommended)

**Best for**: Complex apps, multiple dependent UI elements

Using a reactive library like `signals`, `leptos_reactive`, or custom solution:

```rust
use signals::Signal;

fn create_counter() -> Result<()> {
    // Reactive state
    let count = Signal::new(0);
    
    // Text automatically updates when count changes
    let text = XamlTextBlock::new()?;
    text.bind_text(count.map(|c| format!("Count: {}", c)));
    
    let button = XamlButton::new()?;
    button.on_click({
        let count = count.clone();
        move || {
            count.update(|c| c + 1);
            Ok(())
        }
    })?;
    
    Ok(())
}
```

**Pros:**
- ✅ Automatic UI updates
- ✅ Declarative
- ✅ Scales well
- ✅ Rust-idiomatic

**Cons:**
- ❌ Requires additional library
- ❌ Learning curve

### 🤔 Option 3: Traditional Binding (Not Recommended)

**Best for**: Exact .NET/XAML compatibility (rare)

Implementing INotifyPropertyChanged via COM:

```rust
// This would require:
// - COM interface implementation
// - Property change events
// - Binding expressions
// - Complex FFI

// Example (complex, not recommended):
struct ViewModel {
    count: i32,
    property_changed: Event<PropertyChanged>,
}

impl INotifyPropertyChanged for ViewModel {
    // Requires extensive COM boilerplate
}
```

**Pros:**
- ✅ Familiar to XAML developers

**Cons:**
- ❌ Extremely complex
- ❌ Requires COM interfaces
- ❌ Not Rust-idiomatic
- ❌ Performance overhead
- ❌ Hard to maintain

## Comparison

| Feature | Manual Updates | Reactive Signals | Traditional Binding |
|---------|---------------|------------------|---------------------|
| Complexity | Low | Medium | Very High |
| Learning Curve | Easy | Medium | Hard |
| Performance | Excellent | Excellent | Good |
| Maintainability | Medium | High | Low |
| Rust Idiomatic | Yes | Yes | No |
| Dependencies | None | 1 library | Extensive FFI |
| **Recommended?** | ✅ Yes | ✅ **Yes** | ❌ No |

## Example: Todo App with Reactive State

```rust
use signals::{Signal, SignalVec};

struct TodoApp {
    todos: SignalVec<Todo>,
    input: Signal<String>,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            todos: SignalVec::new(),
            input: Signal::new(String::new()),
        }
    }
    
    fn create_ui(&self) -> Result<()> {
        let input = XamlTextBox::new()?;
        
        // Bind input changes
        input.on_text_changed({
            let input_signal = self.input.clone();
            move |text| {
                input_signal.set(text);
                Ok(())
            }
        })?;
        
        let button = XamlButton::new()?;
        button.on_click({
            let todos = self.todos.clone();
            let input = self.input.clone();
            move || {
                let text = input.get();
                if !text.is_empty() {
                    todos.push(Todo::new(text));
                    input.set(String::new());
                }
                Ok(())
            }
        })?;
        
        // List automatically updates when todos change
        let list = XamlStackPanel::new()?;
        self.todos.for_each({
            let list = list.clone();
            move |todo| {
                let item = XamlTextBlock::new()?;
                item.set_text(&todo.text)?;
                list.add_child(&item)?;
                Ok(())
            }
        })?;
        
        Ok(())
    }
}
```

## Recommendation

**For most applications, use Option 1 (Manual Updates) or Option 2 (Reactive Signals).**

- **Start with Option 1** - Simple, works now, easy to learn
- **Migrate to Option 2** - When your UI becomes complex
- **Avoid Option 3** - Unless you absolutely need .NET compatibility

## Reactive Libraries for Rust

Consider these libraries for reactive state management:

1. **`signals`** - Simple reactive signals
   ```toml
   signals = "0.3"
   ```

2. **`leptos_reactive`** - From the Leptos framework
   ```toml
   leptos_reactive = "0.5"
   ```

3. **`futures-signals`** - Async reactive streams
   ```toml
   futures-signals = "0.3"
   ```

4. **Custom solution** - Implement your own using `Arc<Mutex<T>>` and callbacks

## Future Work

If there's strong demand for traditional binding, we could add:

- [ ] Simple property change notification system
- [ ] Macro-based binding helpers
- [ ] Collection change notifications
- [ ] Computed properties

But for now, **manual updates and reactive signals are the recommended approaches** as they're simpler, more Rust-idiomatic, and work better with Rust's ownership model.

## Questions?

Open an issue on GitHub to discuss state management patterns!
