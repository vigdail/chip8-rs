use chip8::Chip8;
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use std::time::{Duration, Instant};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const PIXEL_OFF_COLOR: u32 = 0x00000000;
const PIXEL_ON_COLOR: u32 = 0xffffffff;

fn main() {
    let mut window = Window::new(
        "Chip-8",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X16,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let mut buffer = [0; WIDTH * HEIGHT];

    let mut chip8 = Chip8::new();
    let program = &[
        0x60, 0x81, // 0x0200: LD v0, 0xaa
        0xf0, 0x55, // 0x0202: LD [I], v0
        0x60, 0x7e, // 0x0204: LD v0, 0x55
        0xf0, 0x55, // 0x0206: LD [I], v0
        0x60, 0x00, // 0x0208: LD v0, 0x00
        0xa0, 0x00, // 0x020a: LD I, 0x000
        0xd0, 0x02, // 0x020c: DRW vx, vy, 0x02
        0x12, 0x0e, // 0x020e: JMP 0x020e ; infinite loop
    ];
    chip8.set_program(program);

    let mut last_draw = Instant::now();
    let mut last_run = Instant::now();
    let mut last_key_update = Instant::now();
    let mut last_timer = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time = Instant::now();
        if time - last_key_update >= Duration::from_millis(200) {
            let key_pressed = window
                .get_keys_pressed(KeyRepeat::Yes)
                .get(0)
                .and_then(map_keycode);
            chip8.set_key_pressed(key_pressed);

            last_key_update = Instant::now();
        }

        if time - last_timer >= Duration::from_micros(16667) {
            chip8.tick_timers();
            last_timer = Instant::now();
        }

        if Instant::now() - last_run > Duration::from_millis(2) {
            chip8.run();
            last_run = Instant::now();
        }

        if time - last_draw >= Duration::from_millis(10) {
            for (i, b) in chip8.get_framebuffer().iter().enumerate() {
                buffer[i] = if *b > 0 {
                    PIXEL_ON_COLOR
                } else {
                    PIXEL_OFF_COLOR
                };
            }

            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
            last_draw = Instant::now();
        }
    }
}

fn map_keycode(key: &Key) -> Option<u8> {
    match key {
        Key::Key1 => Some(0x1),
        Key::Key2 => Some(0x2),
        Key::Key3 => Some(0x3),
        Key::Key4 => Some(0xC),

        Key::Q => Some(0x4),
        Key::W => Some(0x5),
        Key::E => Some(0x6),
        Key::R => Some(0xD),

        Key::A => Some(0x7),
        Key::S => Some(0x8),
        Key::D => Some(0x9),
        Key::F => Some(0xE),

        Key::Z => Some(0xA),
        Key::X => Some(0x0),
        Key::C => Some(0xB),
        Key::V => Some(0xF),
        _ => None,
    }
}
