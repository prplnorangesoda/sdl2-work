extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use simple_logger::SimpleLogger;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug)]
struct AppState {
    should_shutdown: bool,
    current_iter: u64,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            should_shutdown: false,
            current_iter: 0,
        }
    }
}

fn update(state: &AppState) -> AppState {
    log::trace!("Current update iteration: {0}", state.current_iter);
    AppState {
        should_shutdown: false,
        current_iter: (state.current_iter + 1) % u64::MAX,
    }
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

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let default_state = AppState::default();
    let state = Arc::new(RwLock::new(default_state));

    let extra_state = state.clone();

    let caskaydia_font = font_ctx
        .load_font(
            "/usr/share/fonts/cascadiacode/CaskaydiaCoveNerdFont-SemiBold.ttf",
            14,
        )
        .unwrap();
    let game_thread = thread::spawn(move || {
        let state_rwlock = extra_state;
        'thread_inner: loop {
            thread::sleep(Duration::from_nanos(16_666_667));
            let state = {
                let state = state_rwlock.read().unwrap();
                if state.should_shutdown {
                    break 'thread_inner;
                }
                update(&state)
            };
            *state_rwlock.write().unwrap() = state;
        }
    });
    // the render thread loop

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut now = timer_subsystem.performance_counter();
    let mut last;
    let mut loose_i: f64 = 0.0;

    let texture_creator = canvas.texture_creator();

    let mut frame_count: u128 = 0;
    'running: loop {
        frame_count += 1;
        last = now;
        now = timer_subsystem.performance_counter();

        let delta_time =
            ((now - last) * 1000) as f64 / timer_subsystem.performance_frequency() as f64;

        log::debug!("delta_time: {delta_time}");
        log::debug!("loose_i: {loose_i}");

        loose_i = loose_i + (delta_time / 10.0);
        let i = (loose_i % 255.0).floor() as u8;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

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
                _ => {}
            }
        }

        let state = state.read().unwrap();
        let font_surf = caskaydia_font
            .render(&format!(
                "Current game thread iteration: {0}",
                state.current_iter
            ))
            .shaded(Color::RGB(255, 255, 255), Color::RGBA(0, 0, 0, 50))
            .unwrap();

        let font_surf2 = caskaydia_font
            .render(&format!(
                "Current render thread iteration: {0}",
                frame_count
            ))
            .shaded(Color::RGB(255, 255, 255), Color::RGBA(0, 0, 0, 50))
            .unwrap();
        let font_surf3 = caskaydia_font
            .render(&format!("delta_time: {0}", delta_time))
            .shaded(Color::RGB(255, 255, 255), Color::RGBA(0, 0, 0, 50))
            .unwrap();
        let (wide1, tall1) = (font_surf.width(), font_surf.height());
        let (wide2, tall2) = (font_surf2.width(), font_surf2.height());
        let (wide3, tall3) = (font_surf3.width(), font_surf3.height());
        log::trace!("Wide1: {wide1}, tall1: {tall1}");
        log::trace!("Wide2: {wide2}, tall2: {tall2}");
        log::trace!("Wide3: {wide3}, tall3: {tall3}");

        let tex = texture_creator
            .create_texture_from_surface(font_surf)
            .unwrap();
        let tex2 = texture_creator
            .create_texture_from_surface(font_surf2)
            .unwrap();
        let tex3 = texture_creator
            .create_texture_from_surface(font_surf3)
            .unwrap();

        let texture_rect = Rect::new(50, 50, wide1, tall1);
        let texture_rect2 = Rect::new(50, 50 + i32::try_from(tall1).unwrap(), wide2, tall2);
        let texture_rect3 = Rect::new(50, 50 + i32::try_from(tall1 + tall2).unwrap(), wide3, tall3);

        canvas.copy(&tex, None, texture_rect).unwrap();
        canvas.copy(&tex2, None, texture_rect2).unwrap();
        canvas.copy(&tex3, None, texture_rect3).unwrap();

        canvas.present();
    }
    log::info!("Bye, world!");
}
