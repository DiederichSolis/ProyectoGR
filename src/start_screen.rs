use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Pantalla de Bienvenida", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().accelerated().build()?;
    let mut event_pump = sdl_context.event_pump()?;

    // Main loop
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        // Render
        canvas.set_draw_color(Color::RGB(0, 0, 0)); // Black background
        canvas.clear();

        // Draw welcome text
        draw_text(&mut canvas, "Bienvenido a la aplicación", 100, 100, Color::RGB(255, 255, 255))?;

        // Draw button
        draw_button(&mut canvas, 300, 400, 200, 50, "Hacer clic")?;

        // Present
        canvas.present();

        // Sleep for a short while to reduce CPU usage
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}

fn draw_text(canvas: &mut Canvas<Window>, text: &str, x: i32, y: i32, color: Color) -> Result<(), String> {
    // Here, you would use an SDL2 text rendering library like `sdl2_ttf` to render the text.
    // For simplicity, we are just drawing a rectangle instead of text.
    canvas.set_draw_color(color);
    canvas.fill_rect(sdl2::rect::Rect::new(x, y, 300, 50))?;
    Ok(())
}

fn draw_button(canvas: &mut Canvas<Window>, x: i32, y: i32, width: u32, height: u32, label: &str) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 255)); // Blue button
    canvas.fill_rect(sdl2::rect::Rect::new(x, y, width, height))?;

    canvas.set_draw_color(Color::RGB(255, 255, 255)); // White border
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, width, height))?;

    // Here, you would render the button label as well, which requires text rendering.
    // For simplicity, the label is not displayed.
    Ok(())
}
