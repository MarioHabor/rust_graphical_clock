use std::time::{SystemTime, UNIX_EPOCH};

use piston_window::*;
use std::f64::consts::PI;

const WINDOW_SIZE_X: u32 = 840;
const WINDOW_SIZE_Y: u32 = 710;

const CENTER_X: f64 = WINDOW_SIZE_X as f64 / 2.0;
const CENTER_Y: f64 = WINDOW_SIZE_Y as f64 / 2.0;
const RADIUS_X: f64 = 320.0; // Semi-major axis
const RADIUS_Y: f64 = 320.0; // Semi-minor axis
const ARC_THICKNESS: f64 = 6.0; // Thickness of the arc

const TEXT_OFFSET_FACTOR: f64 = 0.89;
const TEXT_SIZE: u32 = 29;
const SECONDS_HAND_LENGTH: f64 = 280.0;

const LONG_LINE_LENGTH: f64 = 13.0; // Length of 5-minute markers
const SHORT_LINE_LENGTH: f64 = 10.0; // Length of regular minute markers
const LONG_LINE_THICKNESS: f64 = 2.6; // minute makers
const SHORT_LINE_THICKNESS: f64 = 1.2;

const MINUTE_HAND_LENGTH: f64 = 255.10;
const MINUTE_HAND_THICKNESS: f64 = 2.5;
const HOUR_HAND_LENGTH: f64 = 160.0;
const HOUR_HAND_THICKNESS: f64 = 4.0;
const CLOCK_PIN_CIRCLE: f64 = 8.0;

const FONT_PATH: &str = "assets/BlackRolmer-Oblique.otf";

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Clock", [WINDOW_SIZE_X, WINDOW_SIZE_Y])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut glyphs = window.load_font(FONT_PATH).unwrap(); // Load a font file

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, device| {
            clear([0.1, 0.1, 0.2, 1.0], g); // Background color
                                            // Define the bounding rectangle for the elliptical arc
            let rect = [
                CENTER_X - RADIUS_X, // Top-left X
                CENTER_Y - RADIUS_Y, // Top-left Y
                2.0 * RADIUS_X,      // Width
                2.0 * RADIUS_Y,      // Height
            ];

            // Draw an arc around the elliptical clock
            circle_arc(
                [1.0, 1.0, 1.0, 1.0], // White color
                ARC_THICKNESS,        // Line thickness
                0.0,                  // Start angle (0 radians)
                PI,                   // End angle (180 degrees, half-circle)
                rect,                 // Bounding rectangle
                c.transform,
                g,
            );

            // Draw the second half of the elliptical arc
            circle_arc(
                [1.0, 1.0, 1.0, 1.0], // White color
                ARC_THICKNESS,        // Line thickness
                PI,                   // Start angle (180 degrees)
                2.0 * PI,             // End angle (360 degrees)
                rect,                 // Bounding rectangle
                c.transform,
                g,
            );

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
                    TEXT_SIZE,            // Font size
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
            let hand_x = CENTER_X + SECONDS_HAND_LENGTH * angle.cos();
            let hand_y = CENTER_Y + SECONDS_HAND_LENGTH * angle.sin();

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

            let total_seconds = current_time.as_secs();
            let hours = (total_seconds / 3600) % 12; // Convert to hours (0-11)
            let minutes = (total_seconds / 60) % 60; // Convert to minutes (0-59)

            // Calculate angles for the hands
            let minute_angle = (minutes as f64) * (2.0 * PI / 60.0) - PI / 2.0;
            let hour_angle = (hours as f64) * (2.0 * PI / 12.0)
                + (minutes as f64) * (2.0 * PI / (12.0 * 60.0))
                - PI / 2.0;

            // Minute hand
            let minute_x = CENTER_X + MINUTE_HAND_LENGTH * minute_angle.cos();
            let minute_y = CENTER_Y + MINUTE_HAND_LENGTH * minute_angle.sin();
            line(
                [1.0, 1.0, 1.0, 1.0], // White color
                MINUTE_HAND_THICKNESS,
                [CENTER_X, CENTER_Y, minute_x, minute_y],
                c.transform,
                g,
            );

            // Hour hand
            let hour_x = CENTER_X + HOUR_HAND_LENGTH * hour_angle.cos();
            let hour_y = CENTER_Y + HOUR_HAND_LENGTH * hour_angle.sin();
            line(
                [1.0, 1.0, 1.0, 1.0], // White color
                HOUR_HAND_THICKNESS,
                [CENTER_X, CENTER_Y, hour_x, hour_y],
                c.transform,
                g,
            );

            // Draw the seconds hand
            line(
                [1.0, 0.0, 0.0, 1.0],                 // Color (red)
                2.0,                                  // Line thickness
                [CENTER_X, CENTER_Y, hand_x, hand_y], // Line from center to calculated endpoint
                c.transform,
                g,
            );

            // Draw the small center dot
            ellipse(
                [0.125, 0.275, 0.631, 1.0], // White color
                [
                    CENTER_X - CLOCK_PIN_CIRCLE, // X-coordinate: Center minus radius
                    CENTER_Y - CLOCK_PIN_CIRCLE, // Y-coordinate: Center minus radius
                    2.0 * CLOCK_PIN_CIRCLE,      // Width of the dot
                    2.0 * CLOCK_PIN_CIRCLE,      // Height of the dot
                ],
                c.transform,
                g,
            );

            glyphs.factory.encoder.flush(device);
        });
    }
}
