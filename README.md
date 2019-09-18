Simple Mandelbrot Viewer
========================
This is a simple CPU mandelbrot renderer. It's a project for me to add to as I
learn more about Rust.

## Current features
- Grayscale rendering
- Pan
- Zoom (needs work)

## Limitations
- Can't zoom past limit of 64 bit float precision
- Drawing to individual pixels with CPU is laggy

## Planned features
- GPU rendering
..* Render in different thread?
- Dynamic precision
- Colour, colour interpolation
