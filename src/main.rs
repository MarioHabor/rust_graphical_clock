// use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use std::time::{SystemTime, UNIX_EPOCH};

use piston_window::*;
use std::f64::consts::PI;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Clock", [840, 710])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut glyphs = window.load_font("assets/FiraSans-Black.ttf").unwrap(); // Load a font file

    let center_x = 420.0;
    let center_y = 360.0;
    let radius_x = 320.0; // Semi-major axis
    let radius_y = 320.0; // Semi-minor axis
    let text_offset_factor = 0.89;
    let hand_length = 220.0;
    let long_line_length = 13.0; // Length of 5-minute markers
    let short_line_length = 10.0; // Length of regular minute markers
    let long_line_thickness = 2.6;
    let short_line_thickness = 1.2;

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, device| {
            clear([0.1, 0.1, 0.2, 1.0], g); // Background color

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

            // Draw clock numbers
            for hour in 1..=12 {
                let angle = (hour as f64) * PI / 6.0 - PI / 2.0;
                let x = center_x + radius_x * text_offset_factor * angle.cos();
                let y = center_y + radius_y * text_offset_factor * angle.sin();

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
            // Draw the hour number at calculated position
            text(
                [1.0, 1.0, 1.0, 1.0], // White color
                25,                   // Font size
                "For Ally",           // Hour number as text
                &mut glyphs,
                c.transform.trans(390.0, 440.0), // Adjust position slightly
                g,
            )
            .unwrap();

            // Get the current time in seconds
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            let seconds = current_time.as_secs() % 60;
            let angle = (seconds as f64) * (2.0 * PI / 60.0) - PI / 2.0;
            // Calculate the endpoint of the seconds hand
            let hand_x = center_x + hand_length * angle.cos();
            let hand_y = center_y + hand_length * angle.sin();

            // Draw the seconds hand
            line(
                [1.0, 0.0, 0.0, 1.0],                 // Color (red)
                2.0,                                  // Line thickness
                [center_x, center_y, hand_x, hand_y], // Line from center to calculated endpoint
                c.transform,
                g,
            );

            // Draw minute markers
            for minute in 0..60 {
                let angle = (minute as f64) * (2.0 * PI / 60.0) - PI / 2.0;

                // Determine line length (longer for 5th minute)
                let (line_length, line_thickness) = if minute % 5 == 0 {
                    (long_line_length, long_line_thickness) // Long and bold for 5-minute markers
                } else {
                    (short_line_length, short_line_thickness) // Short and thin for regular markers
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

            glyphs.factory.encoder.flush(device);
        });
    }
}
