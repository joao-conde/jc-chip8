use jc_chip8::chip8::Chip8;
use sdl2::{keyboard::Keycode, pixels::PixelFormatEnum};
use std::io::Read;
use std::{collections::HashMap, fs::File};

const ROMS: &str = "../roms/";
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALING: usize = 10;
const PIXEL_SET: (u8, u8, u8) = (0xFF, 0xFF, 0xFF);
const PIXEL_UNSET: (u8, u8, u8) = (0x00, 0x00, 0x00);
const KEY_MAPPINGS: [(Keycode, u8); 16] = [
    (Keycode::Num1, 0x01),
    (Keycode::Num2, 0x02),
    (Keycode::Num3, 0x03),
    (Keycode::Num4, 0x0C),
    (Keycode::Q, 0x04),
    (Keycode::W, 0x05),
    (Keycode::E, 0x06),
    (Keycode::R, 0x0D),
    (Keycode::A, 0x07),
    (Keycode::S, 0x08),
    (Keycode::D, 0x09),
    (Keycode::F, 0x0E),
    (Keycode::Z, 0x0A),
    (Keycode::X, 0x00),
    (Keycode::C, 0x0B),
    (Keycode::V, 0x0F),
];

fn main() {
    let key_mapper = KEY_MAPPINGS
        .iter()
        .cloned()
        .collect::<HashMap<Keycode, u8>>();

    // read ROM bytes
    let mut file = File::open(&format!("{}/Pong.ch8", ROMS)).unwrap();
    let mut rom = Vec::new();
    file.read_to_end(&mut rom).unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom);

    // SDL graphics
    let sdl = sdl2::init().expect("failed to init SDL");
    let video_subsystem = sdl.video().expect("failed to get video context");

    let main_window = video_subsystem
        .window(
            "Chip-8 Emulator",
            (SCALING * WIDTH) as u32,
            (SCALING * HEIGHT) as u32,
        )
        .resizable()
        .build()
        .expect("failed to build window");
    let mut canvas = main_window
        .into_canvas()
        .build()
        .expect("failed to build window's canvas");
    canvas
        .set_scale(SCALING as f32, SCALING as f32)
        .expect("failed setting window scale");
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH as u32, HEIGHT as u32)
        .unwrap();

    // emulate clock ticks
    let mut timer_subsystem = sdl.timer().expect("failed to get timer system");
    let tick_interval = 1000 / 120; // frequency in Hz to period in ms
    let mut last_update_time = 0;

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        while let Some(event) = event_pump.poll_event() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,

                sdl2::event::Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    key_mapper.get(&key).map(|byte| chip8.key_press(*byte));
                }

                sdl2::event::Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    key_mapper.get(&key).map(|byte| chip8.key_lift(*byte));
                }

                _ => {}
            }
        }

        chip8.clock();
        chip8.clock_dt();
        chip8.clock_st();

        let current_time = timer_subsystem.ticks();
        let delta_t = current_time - last_update_time;

        if tick_interval > delta_t {
            timer_subsystem.delay(tick_interval - delta_t);

            let mut pixels = [0u8; WIDTH * HEIGHT * 3];
            for (i, p) in chip8.pixels().iter().enumerate() {
                let (r, g, b) = if *p == 1 { PIXEL_SET } else { PIXEL_UNSET };
                pixels[i * 3] = r;
                pixels[i * 3 + 1] = g;
                pixels[i * 3 + 2] = b;
            }

            texture
                .update(None, &pixels, WIDTH * 3)
                .expect("failure updating texture");

            canvas
                .copy(&texture, None, None)
                .expect("failure copying texture to canvas");

            canvas.present();
        }

        last_update_time = current_time;
    }
}
