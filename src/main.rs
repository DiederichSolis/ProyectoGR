use std::time::Instant;

use glam::Vec2;
use raycasting::RayCasting;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

mod raycasting;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("raycasting", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let mut raycasting = RayCasting {
        world: WORLD_MAP.to_vec().iter().map(|v| v.to_vec()).collect(),
        pos: Vec2::new(22.0, 12.0),
        dir: Vec2::new(-1.0, 0.0),
        plane: Vec2::new(0.0, 0.66),
    };

    let mut last_frame_time = Instant::now();
    let mut delta_time;

    let mut events = sdl_context.event_pump()?;

    'mainloop: loop {
        let now = Instant::now();
        delta_time = now.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = now;

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }

        let mut move_factor = 0.0;
        let mut rotation_factor = 0.0;

        let keyboard_state = events.keyboard_state();
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::W)
            || keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Up)
        {
            move_factor += 3.0 * delta_time;
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::S)
            || keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Down)
        {
            move_factor -= 3.0 * delta_time;
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::A)
            || keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Left)
        {
            rotation_factor += 0.9 * delta_time;
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::D)
            || keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right)
        {
            rotation_factor -= 0.9 * delta_time;
        }

        raycasting.transform_cam(move_factor, rotation_factor);
        let lines: Vec<(glam::U16Vec2, glam::U16Vec2, u8, bool)> =
            raycasting.lines(WINDOW_WIDTH as u16, WINDOW_HEIGHT as u16);

        // clear the screen
        canvas.set_draw_color(Color::RGB(43, 44, 47));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(52, 134, 235));
        canvas.fill_rect(Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT / 2))?;

        // render the world
        for line in lines {
            let mut color = map_index_to_color(line.2);
            if line.3 {
                color = fade(color);
            }
            canvas.set_draw_color(color);
            canvas.draw_line(
                Point::new(line.0.x as i32, line.0.y as i32),
                Point::new(line.1.x as i32, line.1.y as i32),
            )?;
        }

        // render the map
        let inc_x = (WINDOW_WIDTH - MAP_WIDTH as u32 * 5 - 10) as i32;
        let inc_y = (WINDOW_HEIGHT - MAP_HEIGHT as u32 * 5 - 10) as i32;
        for (y, row) in raycasting.world.iter().enumerate() {
            for (x, index) in row.iter().enumerate() {
                let color = map_index_to_color(*index);
                canvas.set_draw_color(color);
                let rect = Rect::new(inc_x + x as i32 * 5, inc_y + y as i32 * 5, 5, 5);
                canvas.fill_rect(rect)?;
            }
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(
            inc_x + (raycasting.pos.x * 5.0) as i32 - 1,
            inc_y + (raycasting.pos.y * 5.0) as i32 - 1,
            3,
            3,
        ))?;
        canvas.draw_line(
            Point::new(
                inc_x + (raycasting.pos.x * 5.0) as i32,
                inc_y + (raycasting.pos.y * 5.0) as i32,
            ),
            Point::new(
                inc_x + (raycasting.pos * 5.0 + raycasting.dir.normalize() * 5.0).x as i32,
                inc_y + (raycasting.pos * 5.0 + raycasting.dir.normalize() * 5.0).y as i32,
            ),
        )?;

        canvas.present();
    }

    Ok(())
}

const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;

const WORLD_MAP: [[u8; MAP_WIDTH]; MAP_HEIGHT] = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
];

fn map_index_to_color(i: u8) -> Color {
    match i {
        0 => Color::RGB(43, 44, 47),
        1 => Color::BLACK,
        2 => Color::GRAY,
        3 => Color::RED,
        4 => Color::WHITE,
        _ => Color::GREEN,
    }
}

fn fade(c: Color) -> Color {
    Color::RGB(c.r / 2, c.g / 2, c.b / 2)
}
