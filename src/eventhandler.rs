use sdl2::keyboard::{Keycode, Mod, Scancode};

#[derive(Debug)]
pub struct KeyDownEventInfo {}

type KeyDownFn = Box<dyn Fn(KeyDownEventInfo) -> () + Send + Sync>;

pub struct HandlerFns {
    pub inner_keydowns: Vec<KeyDownFn>,
}

impl Default for HandlerFns {
    fn default() -> Self {
        Self {
            inner_keydowns: Vec::new(),
        }
    }
}
pub struct EventHandler {
    inner_handlers: HandlerFns,
}

// STATIC
impl EventHandler {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for EventHandler {
    fn default() -> Self {
        EventHandler {
            inner_handlers: HandlerFns::default(),
        }
    }
}

// METHODS
impl EventHandler {
    pub fn register_handler_keydown(&mut self, callback: KeyDownFn) {
        self.inner_handlers.inner_keydowns.push(callback);
    }
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
        for callback in self.inner_handlers.inner_keydowns.iter() {
            callback(KeyDownEventInfo {})
        }
    }
}
