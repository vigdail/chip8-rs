use chip8::Chip8;
use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const BLACK: u32 = 0x00000000;
const WHITE: u32 = 0xffffffff;

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
        0x60, 0xaa, // 0x0200: LD v0, 0xaa
        0xf0, 0x55, // 0x0202: LD [I], v0
        0x60, 0x55, // 0x0204: LD v0, 0x55
        0xf0, 0x55, // 0x0206: LD [I], v0
        0x60, 0x00, // 0x0208: LD v0, 0x00
        0xa0, 0x00, // 0x020a: LD I, 0x000
        0xd0, 0x02, // 0x020c: DRW vx, vy, 0x02
    ];
    chip8.set_program(program);
    for _ in 0..program.len() / 2 {
        chip8.run();
    }

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    println!("{:?}", &chip8.get_framebuffer()[0..0xf]);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, b) in chip8.get_framebuffer().iter().enumerate() {
            buffer[i] = if *b > 0 { WHITE } else { BLACK };
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
