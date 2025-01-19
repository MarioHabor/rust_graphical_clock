// use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use std::time::{SystemTime, UNIX_EPOCH};

use piston_window::*;
use std::f64::consts::PI;

const CENTER_X: f64 = 420.0;
const CENTER_Y: f64 = 360.0;
const RADIUS_X: f64 = 320.0; // Semi-major axis
const RADIUS_Y: f64 = 320.0; // Semi-minor axis
const TEXT_OFFSET_FACTOR: f64 = 0.89;
const HAND_LENGTH: f64 = 220.0;
const LONG_LINE_LENGTH: f64 = 13.0; // Length of 5-minute markers
const SHORT_LINE_LENGTH: f64 = 10.0; // Length of regular minute markers
const LONG_LINE_THICKNESS: f64 = 2.6;
const SHORT_LINE_THICKNESS: f64 = 1.2;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Clock", [840, 710])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut glyphs = window.load_font("assets/FiraSans-Black.ttf").unwrap(); // Load a font file

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, device| {
            clear([0.1, 0.1, 0.2, 1.0], g); // Background color

            // Draw the ellipse (clock face)
            ellipse(
                [0.263, 0.431, 0.839, 1.0], // Color (blue)
                [
                    CENTER_X - RADIUS_X,
                    CENTER_Y - RADIUS_Y,
                    2.0 * RADIUS_X,
                    2.0 * RADIUS_Y,
                ],
                c.transform,
                g,
            );
            for hour in 1..=12 {
                let angle = (hour as f64) * PI / 6.0 - PI / 2.0;
                let x = CENTER_X + RADIUS_X * TEXT_OFFSET_FACTOR * angle.cos();
                let y = CENTER_Y + RADIUS_Y * TEXT_OFFSET_FACTOR * angle.sin();

                // Draw the hour number at calculated position
                text(
                    [1.0, 1.0, 1.0, 1.0], // White color
                    25,                   // Font size
                    &hour.to_string(),    // Hour number as text
                    &mut glyphs,
                    c.transform.trans(x - 10.0, y + 10.0), // Adjust position slightly
                    g,
                )
                .unwrap();
            }

            // Get the current time in seconds
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            let seconds = current_time.as_secs() % 60;
            let angle = (seconds as f64) * (2.0 * PI / 60.0) - PI / 2.0;
            // Calculate the endpoint of the seconds hand
            let hand_x = CENTER_X + HAND_LENGTH * angle.cos();
            let hand_y = CENTER_Y + HAND_LENGTH * angle.sin();

            // Draw the seconds hand
            line(
                [1.0, 0.0, 0.0, 1.0],                 // Color (red)
                2.0,                                  // Line thickness
                [CENTER_X, CENTER_Y, hand_x, hand_y], // Line from center to calculated endpoint
                c.transform,
                g,
            );

            // Draw minute markers
            for minute in 0..60 {
                let angle = (minute as f64) * (2.0 * PI / 60.0) - PI / 2.0;

                // Determine line length (longer for 5th minute)
                let (line_length, line_thickness) = if minute % 5 == 0 {
                    (LONG_LINE_LENGTH, LONG_LINE_THICKNESS) // Long and bold for 5-minute markers
                } else {
                    (SHORT_LINE_LENGTH, SHORT_LINE_THICKNESS) // Short and thin for regular markers
                };
                // Calculate start and end points of the line
                let start_x = CENTER_X + (RADIUS_X - line_length) * angle.cos();
                let start_y = CENTER_Y + (RADIUS_Y - line_length) * angle.sin();
                let end_x = CENTER_X + RADIUS_X * angle.cos();
                let end_y = CENTER_Y + RADIUS_Y * angle.sin();

                // Draw the line
                line(
                    [1.0, 1.0, 1.0, 1.0],             // Color (white)
                    line_thickness,                   // Line thickness
                    [start_x, start_y, end_x, end_y], // Line from start to end
                    c.transform,
                    g,
                );
            }

            glyphs.factory.encoder.flush(device);
        });
    }
}
