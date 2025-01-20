use std::time::{SystemTime, UNIX_EPOCH};

use piston_window::*;
use std::f64::consts::PI;

const WINDOW_SIZE_X: u32 = 840;
const WINDOW_SIZE_Y: u32 = 710;

// const RADIUS_X: f64 = 320.0; // Semi-major axis
// const RADIUS_Y: f64 = 320.0; // Semi-minor axis
const ARC_THICKNESS: f64 = 6.0; // Thickness of the arc

const TEXT_OFFSET_FACTOR: f64 = 0.89;
const TEXT_SIZE: u32 = 29;
// const SECONDS_HAND_LENGTH: f64 = 280.0;

const LONG_LINE_LENGTH: f64 = 13.0; // Length of 5-minute markers
const SHORT_LINE_LENGTH: f64 = 10.0; // Length of regular minute markers
const LONG_LINE_THICKNESS: f64 = 2.6; // minute makers
const SHORT_LINE_THICKNESS: f64 = 1.2;

// const MINUTE_HAND_LENGTH: f64 = 255.10;
const MINUTE_HAND_THICKNESS: f64 = 2.5;
// const HOUR_HAND_LENGTH: f64 = 160.0;
const HOUR_HAND_THICKNESS: f64 = 4.0;
const CLOCK_PIN_CIRCLE: f64 = 8.0;

const FONT_PATH: &str = "assets/BlackRolmer-Oblique.otf";

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Clock", [WINDOW_SIZE_X, WINDOW_SIZE_Y])
        .exit_on_esc(true)
        .transparent(true)
        .build()
        .unwrap();
    let mut glyphs = window.load_font(FONT_PATH).unwrap(); // Load a font file

    while let Some(event) = window.next() {
        let window_size = window.size();
        let center_x = window_size.width / 2.0;
        let center_y = window_size.height / 2.0;

        let mut radius_x = window_size.width / 2.0 - 100.0;
        let mut radius_y = window_size.height / 2.0 - 35.0;
        let mut minute_hand_length = decrease_by_percentage(radius_x, 0.39631);
        let mut hour_hand_length = decrease_by_percentage(radius_x, 0.409524);
        let mut seconds_hand_length = decrease_by_percentage(radius_x, 0.266667);
        if radius_y < 320.0 || radius_x < 320.0 {
            radius_x = 320.0;
            radius_y = 320.0;
            minute_hand_length = 255.10;
            hour_hand_length = 160.0;
            seconds_hand_length = 280.0
        }
        if radius_x > 730.0 {
            radius_x = 730.0;
            minute_hand_length = 585.0;
            hour_hand_length = 500.0;
            seconds_hand_length = 670.0;
        }

        window.draw_2d(&event, |c, g, device| {
            // clear([0.1, 0.1, 0.2, 0.0], g); // transparent background
            clear([0.1, 0.1, 0.2, 1.0], g);
            // Background color
            // Define the bounding rectangle for the elliptical arc
            let rect = [
                center_x - radius_x, // Top-left X
                center_y - radius_y, // Top-left Y
                2.0 * radius_x,      // Width
                2.0 * radius_y,      // Height
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
                    center_x - radius_x,
                    center_y - radius_y,
                    2.0 * radius_x,
                    2.0 * radius_y,
                ],
                c.transform,
                g,
            );
            for hour in 1..=12 {
                let angle = (hour as f64) * PI / 6.0 - PI / 2.0;
                let x = center_x + radius_x * TEXT_OFFSET_FACTOR * angle.cos();
                let y = center_y + radius_y * TEXT_OFFSET_FACTOR * angle.sin();

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
                let start_x = center_x + (radius_x - line_length) * angle.cos();
                let start_y = center_y + (radius_y - line_length) * angle.sin();
                let end_x = center_x + radius_x * angle.cos();
                let end_y = center_y + radius_y * angle.sin();

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
            let minute_x = center_x + minute_hand_length * minute_angle.cos();
            let minute_y = center_y + minute_hand_length * minute_angle.sin();
            line(
                [1.0, 1.0, 1.0, 1.0], // White color
                MINUTE_HAND_THICKNESS,
                [center_x, center_y, minute_x, minute_y],
                c.transform,
                g,
            );

            // Hour hand
            let hour_x = center_x + hour_hand_length * hour_angle.cos();
            let hour_y = center_y + hour_hand_length * hour_angle.sin();
            line(
                [1.0, 1.0, 1.0, 1.0], // White color
                HOUR_HAND_THICKNESS,
                [center_x, center_y, hour_x, hour_y],
                c.transform,
                g,
            );

            // Draw the seconds hand
            // Calculate the endpoint of the seconds hand
            let hand_x = center_x + seconds_hand_length * angle.cos();
            let hand_y = center_y + seconds_hand_length * angle.sin();
            line(
                [1.0, 0.0, 0.0, 1.0],                 // Color (red)
                2.0,                                  // Line thickness
                [center_x, center_y, hand_x, hand_y], // Line from center to calculated endpoint
                c.transform,
                g,
            );

            // Draw the small center dot
            ellipse(
                [0.125, 0.275, 0.631, 1.0], // White color
                [
                    center_x - CLOCK_PIN_CIRCLE, // X-coordinate: Center minus radius
                    center_y - CLOCK_PIN_CIRCLE, // Y-coordinate: Center minus radius
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

fn decrease_by_percentage(og_val: f64, percentage: f64) -> f64 {
    // println!("{}", og_val * (1.0 - percentage));
    og_val * (1.0 - percentage)
}
