#![allow(unused)]
extern crate sdl2;
extern crate vector3;

use eventhandler::EventHandler;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::GLProfile;
use simple_logger::SimpleLogger;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

use vector3::Vector3;

pub mod debug;
pub mod eventhandler;

#[derive(Clone, Debug)]
struct AppState {
    example_interthread: u64,
    should_shutdown: bool,
    current_iter: u64,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            example_interthread: 0,
            should_shutdown: false,
            current_iter: 0,
        }
    }
}

fn update(state: &AppState) -> AppState {
    log::trace!("Current update iteration: {0}", state.current_iter);
    AppState {
        example_interthread: state.example_interthread,
        should_shutdown: false,
        current_iter: (state.current_iter + 1) % u64::MAX,
    }
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .expect("Logger should create");
    log::info!("Logger initialized");
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();
    let font_ctx = sdl2::ttf::init().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Window", 800, 600)
        .opengl()
        .build()
        .unwrap();

    // Unlike the other example above, nobody created a context for your window, so you need to create one.
    let ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));

    let default_state = AppState::default();
    let state = Arc::new(RwLock::new(default_state));

    let event_handler = Arc::new(RwLock::new(eventhandler::EventHandler::new()));

    let extra_state = state.clone();
    let extra_handler = event_handler.clone();

    let game_thread = thread::spawn(move || {
        let state_rwlock = extra_state;
        let handler_rwlock = extra_handler;

        let handler_state = Arc::new(Mutex::new(0u64));

        let copied_state = handler_state.clone();
        let init_event_handler = |handler: &mut EventHandler| {
            let ref1 = Arc::clone(&copied_state);
            let ref2 = Arc::clone(&ref1);

            handler.register_handler_keydown(Box::new(move |event| {
                if event.repeat {
                    return;
                }
                let mut numb = ref1.lock().expect("should be possible to grab state");
                *numb += 1;
                // whatever
                dbg!(event);
            }));
            handler.register_handler_keyup(Box::new(move |event| {
                if event.repeat {
                    return;
                }
                let mut numb = ref2.lock().expect("should be possible to grab state");
                *numb -= 1;
                // whatever
                dbg!(event);
            }));
        };

        init_event_handler(
            &mut handler_rwlock
                .write()
                .expect("Should be able to get handler write lock"),
        );
        'thread_inner: loop {
            thread::sleep(Duration::from_nanos(16_666_667));
            let mut state = {
                let state = state_rwlock.read().unwrap();
                if state.should_shutdown {
                    break 'thread_inner;
                }
                update(&state)
            };
            state.example_interthread = *handler_state.lock().unwrap();
            *state_rwlock.write().unwrap() = state;
        }
    });

    let caskaydia_font = font_ctx
        .load_font(
            "/usr/share/fonts/cascadiacode/CaskaydiaCoveNerdFont-SemiBold.ttf",
            14,
        )
        .unwrap();

    // the render thread loop

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut now = timer_subsystem.performance_counter();
    let mut last;
    let mut loose_i: f64 = 0.0;
    let mut frame_count: u128 = 0;

    let mut debug = debug::DebugRenderer::new(&caskaydia_font);
    let mut debug_items: BTreeMap<&'static str, &dyn Debug> = BTreeMap::new();
    'running: loop {
        frame_count += 1;
        last = now;
        now = timer_subsystem.performance_counter();

        let delta_time =
            ((now - last) * 1000) as f64 / timer_subsystem.performance_frequency() as f64;

        loose_i = loose_i + (delta_time / 10.0);
        let i = (loose_i % 255.0).floor() as u8;
        //canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        unsafe {
            gl::ClearColor(f32::from(i) / 255.0, 0.3, 1.0 - (f32::from(i) / 255.0), 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.gl_swap_window();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { timestamp }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    timestamp,
                    ..
                } => {
                    log::info!("Received message to quit at timestamp: {timestamp}, shutting down gracefully, waiting for write lock");
                    let mut state = state.write().unwrap();
                    log::debug!("Write lock acquired");
                    state.should_shutdown = true;
                    drop(state);
                    game_thread.join().unwrap();
                    break 'running;
                }
                Event::KeyDown {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                } => event_handler
                    .read()
                    .expect("should be able to get read lock")
                    .handle_key_down(timestamp, window_id, keycode, scancode, keymod, repeat),
                Event::KeyUp {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                } => event_handler
                    .read()
                    .expect("should be able to get read lock")
                    .handle_key_up(timestamp, window_id, keycode, scancode, keymod, repeat),
                _ => {}
            }
        }

        let state_lock = state.read().unwrap();

        debug_items.insert("Current game tick", &state_lock.current_iter);
        debug_items.insert("Current render frame", &frame_count);
        debug_items.insert("delta_time", &delta_time);

        //        debug.render_to_canvas(&debug_items, &mut canvas);
        debug_items.clear();
        // ugly: signify to compiler that debug_items is clear
        debug_items = debug_items.into_iter().map(|_| unreachable!()).collect();
        drop(state_lock);
        // canvas.present();
    }
    log::info!("Bye, world!");
}
