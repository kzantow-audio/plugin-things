use crate::{dimensions::LogicalPosition, drag_drop::{DropData, DropOperation}, keyboard::KeyboardModifiers};

#[derive(Clone, Copy, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Debug)]
pub enum Event {
    Draw,

    KeyDown {
        key_code: keyboard_types::Code,
        text: Option<String>,
    },

    KeyUp {
        key_code: keyboard_types::Code,
        text: Option<String>,
    },

    KeyboardModifiers {
        modifiers: KeyboardModifiers,
    },

    MouseButtonDown {
        button: MouseButton,
        position: LogicalPosition,
    },

    MouseButtonUp {
        button: MouseButton,
        position: LogicalPosition,
    },

    MouseExited,

    MouseMoved {
        position: LogicalPosition,
    },

    MouseWheel {
        position: LogicalPosition,
        delta_x: f64,
        delta_y: f64,
    },

    DragEntered {
        position: LogicalPosition,
        data: DropData,
    },

    DragExited,

    DragMoved {
        position: LogicalPosition,
        data: DropData,
    },

    DragDropped {
        position: LogicalPosition,
        data: DropData,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EventResponse {
    Handled,
    Ignored,
    DropAccepted(DropOperation),
}

pub type EventCallback = dyn Fn(Event) -> EventResponse;
