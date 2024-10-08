use sdl2::keyboard::{Keycode, Mod, Scancode};

#[derive(Debug)]
pub struct KeyEventInfo {
    pub timestamp: u32,
    pub window_id: u32,
    pub keycode: Option<Keycode>,
    pub scancode: Option<Scancode>,
    pub keymod: Mod,
    pub repeat: bool,
}

type Handler<T> = Box<dyn Fn(T) -> () + Send + Sync>;

type KeyFn = Handler<KeyEventInfo>;

pub struct HandlerFns {
    pub keydowns: Vec<KeyFn>,
    pub keyups: Vec<KeyFn>,
}

impl Default for HandlerFns {
    fn default() -> Self {
        Self {
            keydowns: Vec::new(),
            keyups: Vec::new(),
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
    pub fn register_handler_keyup(&mut self, callback: KeyFn) {
        self.inner_handlers.keyups.push(callback);
    }
    pub fn register_handler_keydown(&mut self, callback: KeyFn) {
        self.inner_handlers.keydowns.push(callback);
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
        for callback in self.inner_handlers.keydowns.iter() {
            callback(KeyEventInfo {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
            })
        }
    }
    pub fn handle_key_up(
        &self,
        timestamp: u32,
        window_id: u32,
        keycode: Option<Keycode>,
        scancode: Option<Scancode>,
        keymod: Mod,
        repeat: bool,
    ) {
        log::info!("key down");
        for callback in self.inner_handlers.keyups.iter() {
            callback(KeyEventInfo {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
            })
        }
    }
}
