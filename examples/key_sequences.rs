use std::sync::{Arc, Mutex};
use inputbot::{KeySequence, KeybdKey::*, MouseButton::*, MouseCursor};

/// This example demonstrates sending sequences of key presses / characters via a KeySequence.
/// This can be used, for example, to create a macro which types a specific string.

fn main() {
    // If you are on Linux, you may wish to call this function first to avoid a startup delay when
    // the fake device is created. Otherwise, your first input event - if it is a key sequence - may
    // have missing characters.
    //     inputbot::init_device();

    // Bind our Backquote key (`, ~) to a function that types out the string "Hello, world!".
    // You must remember to call the `.send()` method on the KeySequence after creating it.
    // You could explicitly define the KeySequence ahead of time and send it later like so:
    //      let seq: KeySequence = KeySequence("Hello, world!");
    //      seq.send();
    BackquoteKey.bind(|| {
        KeySequence("Hello, world!").send();
    });

    // LKey.block_bind(|| KeySequence("j").send());
    // JKey.block_bind(|| {});

    // MouseMove.block_bind(|| {
    //     // println!("X1: {}, Y1: {}", MouseCursor::pos().0, MouseCursor::pos().1);
    // });

    MouseMove.mouse_block_bind( move |x, y| {
        let current_x = MouseCursor::pos().0;
        let current_y = MouseCursor::pos().1;

        let x_diff = x - current_x;
        let y_diff = y - current_y;

        // MouseCursor::move_rel(x_diff * 20, y_diff * 20);
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}
