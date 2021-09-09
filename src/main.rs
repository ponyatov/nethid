//! # Virtual HID daemon (network remote control)

// \ config
// \ game
// default screen window width
pub const W: u16 = 640;
// default screen window height
pub const H: u16 = 480;
// / game
// / config

// \ mod
mod test;
// / mod

// \ extern
extern crate sdl2;
extern crate tracing;
// / extern

// \ use
use tracing::{info, instrument};
// / use

#[instrument]
/// program entry point
fn main() {
    tracing_subscriber::fmt().compact().init();
    // \ args
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();
    let program = std::path::Path::new(&argv[0]);
    let module = program.file_stem().unwrap();
    info!("start {:?} as #{:?} {:?}", module, argc, argv);
    // / args
    game_loop(argv[0].clone());
    // \ atexit
    info!("stop");
    // / atexit
}

// \ game

#[allow(dead_code)]
pub struct Game {
    pub argv: String,
    pub w: u16,
    pub h: u16,
    pub sdl: sdl2::Sdl,
    pub video: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,
    pub icon: sdl2::surface::Surface<'static>,
    pub events: sdl2::EventPump,
    // pub canvas: sdl2::render::WindowCanvas,
}

impl Game {
    #[instrument]
    fn new(argv: String) -> Self {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let window = video
            .window(argv.as_str(), W as u32, H as u32)
            .build()
            .unwrap();
        let icon = sdl2::image::LoadSurface::from_file("doc/logo.png").unwrap();
        // let canvas = window.into_canvas().build().unwrap();
        let event_pump = context.event_pump().unwrap();
        Game {
            argv: argv,
            w: W,
            h: H,
            sdl: context,
            video: video,
            window: window,
            icon: icon,
            events: event_pump,
        }
    }
}

fn game_loop(argv: String) {
    let mut game = Game::new(argv);
    'event: loop {
        for event in game.events.poll_iter() {
            info!("{:?}", event);
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'event,
                _ => (),
            }
        }
    }
}

// / game
