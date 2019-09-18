extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate image as im;
extern crate vecmath;

use piston_window::*;
use piston::window::WindowSettings;

const WIDTH: u32 = 700;
const HEIGHT: u32 = 400;
const X_BOUND: u32 = WIDTH - 1;
const Y_BOUND: u32 = HEIGHT - 1;
const X_BOUND_F: f64 = X_BOUND as f64;
const Y_BOUND_F: f64 = Y_BOUND as f64;

// const MAX_ITERATIONS: u32 = 128;

fn draw(canvas: &mut im::ImageBuffer<im::Rgba<u8>, Vec<u8>>,
        pos: [f64; 2], scale: f64, iterations: u32) {
    let iter_color_scale_fac: f64 = 255. / iterations as f64;

    for yp in 0..Y_BOUND {
        for xp in 0..X_BOUND {
            let x0 = ((xp as f64) * 3.5 / X_BOUND_F) * scale + pos[0] * 3.5 / X_BOUND_F - 2.5;
            let y0 = ((yp as f64) * 2. / Y_BOUND_F) * scale + pos[1] * 2. / Y_BOUND_F - 1.;

            let mut x = 0.;
            let mut y = 0.;

            let mut rsquare = 0.;
            let mut isquare = 0.;
            let mut zsquare = 0.;

            let mut iteration: u32 = 0;

            while rsquare + isquare <= 4. && iteration < iterations {
                x = rsquare - isquare + x0;
                y = zsquare - rsquare - isquare + y0;
                rsquare = x*x;
                isquare = y*y;
                zsquare = (x + y)*(x + y);
                iteration += 1;
            }

            let iteration = (iteration as f64 * iter_color_scale_fac) as u8;

            canvas.put_pixel(xp, yp, im::Rgba([iteration, iteration, iteration, 255]));
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(
            "Mandelbrot",
            [WIDTH, HEIGHT]
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut canvas: im::ImageBuffer<im::Rgba<u8>, Vec<u8>> = im::ImageBuffer::new(WIDTH, HEIGHT);

    let mut texture_ctx = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer()
            .into()
    };
    let mut texture: G2dTexture = Texture::from_image(
            &mut texture_ctx,
            &canvas,
            &TextureSettings::new()
        ).unwrap();

    let mut pos = [0., 0.];
    let mut scale = 1.;

    let mut prev_mouse_pos: Option<[f64; 2]> = None;

    let mut pressed = false;
    let mut renderable = true;

    draw(&mut canvas, pos, scale, 128);
    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            texture.update(&mut texture_ctx, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                texture_ctx.encoder.flush(device);
    
                //clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });

            renderable = true;
        }

        if let Some(_) = e.press_args() {
            pressed = true;
            prev_mouse_pos = None;

        } else if let Some(_) = e.release_args() {
            pressed = false;
        }

        if renderable {
            if let Some(p) = e.mouse_cursor_args() {
                if pressed {
                    if let Some(prev_p) = prev_mouse_pos {
                        pos[0] += (prev_p[0] - p[0]) * scale;
                        pos[1] += (prev_p[1] - p[1]) * scale;

                        draw(&mut canvas, pos, scale, 128);
                        renderable = false;
                    }
 
                }

                prev_mouse_pos = Some(p);
            }
        }

        if let Some(s) = e.mouse_scroll_args() {
            scale *= 1. - s[1] * 0.1;

            if renderable {
                draw(&mut canvas, pos, scale, 128);
                renderable = false;
            }
        }
    }
}
