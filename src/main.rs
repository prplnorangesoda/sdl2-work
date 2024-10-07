extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::sync::Arc;
use std::sync::Mutex;
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
    println!("Current iteration: {0}", state.current_iter);
    AppState {
        should_shutdown: false,
        current_iter: (state.current_iter + 1) % u64::MAX,
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

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
    let state = Arc::new(Mutex::new(default_state));

    let extra_state = state.clone();

    let game_thread = thread::spawn(move || {
        let inner_state = extra_state;
        'thread_inner: loop {
            println!("whatever");
            let mut state = inner_state.lock().unwrap();
            println!("{state:?}");
            if state.should_shutdown {
                println!("State should shutdown");
                break 'thread_inner;
            }
            *state = update(&state);
            drop(state);
            thread::sleep(Duration::from_millis(100));
        }
    });
    // the render thread loop

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut now = timer_subsystem.performance_counter();
    let mut last;
    let mut loose_i: f64 = 0.0;

    'running: loop {
        last = now;
        now = timer_subsystem.performance_counter();

        let delta_time =
            ((now - last) * 1000) as f64 / timer_subsystem.performance_frequency() as f64;

        println!("delta_time: {delta_time}");
        println!("loose_i: {loose_i}");

        loose_i = (loose_i + (delta_time / 10.0));
        let i = (loose_i.floor() % 255.0) as u8;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { timestamp } => {
                    println!("Received message to quit at timestamp: {timestamp}");
                    let mut state = state.lock().unwrap();
                    println!("Write lock acquired");
                    state.should_shutdown = true;
                    drop(state);
                    game_thread.join().unwrap();
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
    }
    println!("Bye, world!");
}
