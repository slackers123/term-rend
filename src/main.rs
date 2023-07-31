use std::usize;

const SCREEN_HEIGHT: usize = 200;
const SCREEN_WIDTH: usize = 200;
const MSAA_COUNT: usize = 2;
const MSAA_MULTIPLYER: f32 = 1.0 / MSAA_COUNT.pow(2) as f32;

type FrameBuffer = [[Color; SCREEN_WIDTH]; SCREEN_HEIGHT];

#[derive(Debug, Default, Clone, Copy)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Vec2 {
    x: f32,
    y: f32,
}

#[derive(Debug, Default, Clone)]
struct Tri {
    a: Vec2,
    b: Vec2,
    c: Vec2,
    color: Color,
}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fb = [[Color::default(); SCREEN_WIDTH]; SCREEN_HEIGHT];
    draw_tri(
        Tri {
            a: Vec2 { x: 0.5, y: 0.5 },
            b: Vec2 { x: 1.0, y: 1.0 },
            c: Vec2 { x: 0.0, y: 1.0 },
            color: Color {
                r: 0.0,
                g: 0.5,
                b: 0.5,
            },
        },
        &mut fb,
    );
    draw_tri(
        Tri {
            a: Vec2 { x: 0.0, y: 0.0 },
            b: Vec2 { x: 1.0, y: 0.0 },
            c: Vec2 { x: 0.5, y: 0.5 },
            color: Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
            },
        },
        &mut fb,
    );

    use image::{Rgb, RgbImage};

    let mut img = RgbImage::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);

    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let pix = fb[y][x];
            img.put_pixel(
                x as u32,
                y as u32,
                Rgb([
                    (255.0 * pix.r) as u8,
                    (255.0 * pix.g) as u8,
                    (255.0 * pix.b) as u8,
                ]),
            );
        }
    }

    let window = show_image::create_window("show res", Default::default())?;
    window.set_image("res image", img)?;

    for event in window.event_channel()? {
        if let show_image::event::WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(show_image::event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
        }
    }

    Ok(())
}

/// Draws a triangle inputs have to be clockwise
fn draw_tri(tri: Tri, fb: &mut FrameBuffer) {
    let pixel_width = 1.0 / SCREEN_WIDTH as f32;
    let subpixel_width = pixel_width / MSAA_COUNT as f32;
    let pixel_height = 1.0 / SCREEN_HEIGHT as f32;
    let subpixel_height = pixel_height / MSAA_COUNT as f32;

    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let r_x = x as f32 / SCREEN_WIDTH as f32;
            let r_y = y as f32 / SCREEN_HEIGHT as f32;

            let mut pixel_res = 0.0;

            for m_x in 0..MSAA_COUNT {
                for m_y in 0..MSAA_COUNT {
                    if is_inside(
                        &tri,
                        Vec2 {
                            x: r_x + (m_x as f32 * subpixel_width),
                            y: r_y + (m_y as f32 * subpixel_height),
                        },
                    ) {
                        pixel_res += MSAA_MULTIPLYER;
                    }
                }
            }
            fb[y][x] = tri.color * pixel_res + fb[y][x] * (1.0 - pixel_res);
        }
    }
}

fn is_inside(tri: &Tri, pixel_pos: Vec2) -> bool {
    if slope_height_at(tri.a, tri.b, pixel_pos.x) >= pixel_pos.y {
        false
    } else if slope_height_at(tri.b, tri.c, pixel_pos.x) <= pixel_pos.y {
        false
    } else if slope_height_at(tri.c, tri.a, pixel_pos.x) <= pixel_pos.y {
        false
    } else {
        true
    }
}

fn slope_height_at(p1: Vec2, p2: Vec2, x: f32) -> f32 {
    let k = (p2.y - p1.y) / (p2.x - p1.x);
    let d = p1.y - (k * p1.x);
    k * x + d
}
