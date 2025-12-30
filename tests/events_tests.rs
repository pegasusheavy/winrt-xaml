//! Unit tests for event handling system.

use winrt_xaml::events::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
fn test_event_handler_creation() {
    let handler: EventHandler<ClickEventArgs> = EventHandler::new();
    assert_eq!(handler.subscriber_count(), 0);
}

#[test]
fn test_event_subscription() {
    let handler: EventHandler<ClickEventArgs> = EventHandler::new();

    handler.subscribe(|_args| {
        println!("Event fired!");
    });

    assert_eq!(handler.subscriber_count(), 1);
}

#[test]
fn test_event_invocation() {
    let handler: EventHandler<ClickEventArgs> = EventHandler::new();
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    handler.subscribe(move |_args| {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    });

    let args = ClickEventArgs::new();
    handler.invoke(&args);

    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn test_multiple_subscribers() {
    let handler: EventHandler<ClickEventArgs> = EventHandler::new();
    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..5 {
        let counter_clone = counter.clone();
        handler.subscribe(move |_args| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
    }

    assert_eq!(handler.subscriber_count(), 5);

    let args = ClickEventArgs::new();
    handler.invoke(&args);

    assert_eq!(counter.load(Ordering::SeqCst), 5);
}

#[test]
fn test_event_clear() {
    let handler: EventHandler<ClickEventArgs> = EventHandler::new();

    handler.subscribe(|_| {});
    handler.subscribe(|_| {});
    handler.subscribe(|_| {});

    assert_eq!(handler.subscriber_count(), 3);

    handler.clear();

    assert_eq!(handler.subscriber_count(), 0);
}

#[test]
fn test_routed_event_args() {
    let args = RoutedEventArgs::new();
    assert!(!args.handled);
    assert!(args.source.is_none());

    let args = RoutedEventArgs::with_source("TestButton");
    assert!(!args.handled);
    assert_eq!(args.source.as_ref().unwrap(), "TestButton");
}

#[test]
fn test_click_event_args() {
    let args = ClickEventArgs::new();
    assert!(!args.routed.handled);
}

#[test]
fn test_text_changed_event_args() {
    let args = TextChangedEventArgs::new("Hello");
    assert_eq!(args.text, "Hello");
    assert!(!args.routed.handled);
}

#[test]
fn test_selection_changed_event_args() {
    let args = SelectionChangedEventArgs::new(5);
    assert_eq!(args.selected_index, 5);
    assert!(!args.routed.handled);
}

#[test]
fn test_value_changed_event_args() {
    let args = ValueChangedEventArgs::new(10.0, 20.0);
    assert_eq!(args.old_value, 10.0);
    assert_eq!(args.new_value, 20.0);
    assert!(!args.routed.handled);
}

#[test]
fn test_checked_event_args() {
    let args = CheckedEventArgs::new(true);
    assert!(args.is_checked);
    assert!(!args.routed.handled);
}

#[test]
fn test_key_event_args() {
    let args = KeyEventArgs::new(0x41, true); // 'A' key
    assert_eq!(args.key_code, 0x41);
    assert!(args.is_down);
}

#[test]
fn test_mouse_event_args() {
    let args = MouseEventArgs::new(100, 200);
    assert_eq!(args.x, 100);
    assert_eq!(args.y, 200);
}

#[test]
fn test_focus_event_args() {
    let args = FocusEventArgs::new(true);
    assert!(args.got_focus);

    let args = FocusEventArgs::new(false);
    assert!(!args.got_focus);
}

#[test]
fn test_event_handler_clone() {
    let handler: EventHandler<ClickEventArgs> = EventHandler::new();
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    handler.subscribe(move |_| {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    });

    let handler_clone = handler.clone();

    let args = ClickEventArgs::new();
    handler_clone.invoke(&args);

    assert_eq!(counter.load(Ordering::SeqCst), 1);
    assert_eq!(handler.subscriber_count(), handler_clone.subscriber_count());
}

