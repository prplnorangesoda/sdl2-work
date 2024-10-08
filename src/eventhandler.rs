use sdl2::keyboard::{Keycode, Mod, Scancode};

#[derive(Debug)]
pub struct KeyDownEventInfo {}

type KeyDownFn = Box<dyn Fn(KeyDownEventInfo) -> () + Send + Sync>;

pub enum HandlerFn {
    KeyDown(KeyDownFn),
}
pub struct EventHandler {
    inner_handlers: Vec<HandlerFn>,
}

// STATIC
impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
            inner_handlers: Vec::new(),
        }
    }
}
impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

// METHODS
impl EventHandler {
    pub fn register_handler_keydown(&mut self, callback: KeyDownFn) {}
    pub fn handle_key_down(
        &self,
        timestamp: u32,
        window_id: u32,
        keycode: Option<Keycode>,
        scancode: Option<Scancode>,
        keymod: Mod,
        repeat: bool,
    ) {
        log::info!("key down");
        dbg!(timestamp, window_id, keycode, scancode, keymod, repeat);
    }
}
