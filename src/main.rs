use std::time::{SystemTime, UNIX_EPOCH};

use piston_window::*;
use std::f64::consts::PI;

const WINDOW_SIZE_X: u32 = 840;
const WINDOW_SIZE_Y: u32 = 710;

const ARC_THICKNESS: f64 = 6.0; // Thickness of the arc

const TEXT_OFFSET_FACTOR: f64 = 0.89;
const TEXT_SIZE: u32 = 29;

const LONG_LINE_LENGTH: f64 = 13.0; // Length of 5-minute markers
const SHORT_LINE_LENGTH: f64 = 10.0; // Length of regular minute markers
const LONG_LINE_THICKNESS: f64 = 2.6; // minute makers
const SHORT_LINE_THICKNESS: f64 = 1.2;

const MINUTE_HAND_THICKNESS: f64 = 2.5;
const HOUR_HAND_THICKNESS: f64 = 4.0;
const CLOCK_PIN_CIRCLE: f64 = 8.0;

const FONT_PATH: &str = "assets/BlackRolmer-Oblique.otf";

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Clock", [WINDOW_SIZE_X, WINDOW_SIZE_Y])
        .exit_on_esc(true)
        // .transparent(true)
        .build()
        .unwrap();
    let mut glyphs = window.load_font(FONT_PATH).unwrap(); // Load a font file
    let mut mouse_x = 400.0; // Initialize mouse position at center
    let mut mouse_y = 300.0;

    while let Some(event) = window.next() {
        let window_size = window.size();
        let center_x = window_size.width / 2.0;
        let center_y = window_size.height / 2.0;

        let mut radius_x = window_size.width / 2.0 - 100.0;
        let mut radius_y = window_size.height / 2.0 - 35.0;
        let mut minute_hand_length = decrease_by_percentage(radius_x, 0.24631);
        let mut hour_hand_length = decrease_by_percentage(radius_x, 0.409524);
        let mut seconds_hand_length = decrease_by_percentage(radius_x, 0.1446);
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

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            // Play sound on click
        }

        if let Some(pos) = event.mouse_cursor_args() {
            mouse_x = pos[0];
            mouse_y = pos[1];
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
            // let angle = (seconds as f64) * (2.0 * PI / 60.0) - PI / 2.0;

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
            let hour_angle = calculate_angle(hours, 12, std::f64::consts::PI / 2.0)
                + calculate_angle(minutes, 60, 0.0) / 12.0; // Adjust for minute offset
            let minute_angle = calculate_angle(minutes, 60, std::f64::consts::PI / 2.0);
            let second_angle = calculate_angle(seconds, 60, std::f64::consts::PI / 2.0);
            // Draw the hands
            draw_hand(
                center_x,
                center_y,
                hour_hand_length,
                hour_angle,
                HOUR_HAND_THICKNESS,
                [1.0, 1.0, 1.0, 1.0], // White
                c,
                g,
            );

            draw_hand(
                center_x,
                center_y,
                minute_hand_length,
                minute_angle,
                MINUTE_HAND_THICKNESS,
                [1.0, 1.0, 1.0, 1.0], // White
                c,
                g,
            );

            draw_hand(
                center_x,
                center_y,
                seconds_hand_length,
                second_angle,
                2.0,                  // Thickness for seconds hand
                [1.0, 0.0, 0.0, 1.0], // Red
                c,
                g,
            );
            // Draw the small center dot
            ellipse(
                [0.125, 0.275, 0.631, 1.0],
                [
                    center_x - CLOCK_PIN_CIRCLE, // X-coordinate: Center minus radius
                    center_y - CLOCK_PIN_CIRCLE, // Y-coordinate: Center minus radius
                    2.0 * CLOCK_PIN_CIRCLE,      // Width of the dot
                    2.0 * CLOCK_PIN_CIRCLE,      // Height of the dot
                ],
                c.transform,
                g,
            );

            // Draw the left eye
            draw_eye(
                center_x - 100.0, // Left eye X offset
                center_y - 150.0,
                mouse_x,
                mouse_y,
                50.0, // Eye horizontal radius
                30.0, // Eye vertical radius
                10.0, // Pupil radius
                c,
                g,
            );

            // Draw the right eye
            draw_eye(
                center_x + 100.0, // Right eye X offset
                center_y - 150.0,
                mouse_x,
                mouse_y,
                50.0, // Eye horizontal radius
                30.0, // Eye vertical radius
                10.0, // Pupil radius
                c,
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

fn calculate_angle(value: u64, max_value: u64, offset: f64) -> f64 {
    (value as f64) * (2.0 * std::f64::consts::PI / max_value as f64) - offset
}

fn draw_hand(
    center_x: f64,
    center_y: f64,
    length: f64,
    angle: f64,
    thickness: f64,
    color: [f32; 4],
    c: Context,
    g: &mut G2d,
) {
    let hand_x = center_x + length * angle.cos();
    let hand_y = center_y + length * angle.sin();
    line(
        color,
        thickness,
        [center_x, center_y, hand_x, hand_y], // Line from center to endpoint
        c.transform,
        g,
    );
}

// Function to draw an eye with a pupil
fn draw_eye(
    eye_x: f64,
    eye_y: f64,
    mouse_x: f64,
    mouse_y: f64,
    eye_radius_x: f64,
    eye_radius_y: f64,
    pupil_radius: f64,
    c: Context,
    g: &mut G2d,
) {
    // Draw the eye (oval)
    ellipse(
        [0.0, 0.0, 0.0, 1.0], // Black color
        [
            eye_x - eye_radius_x,
            eye_y - eye_radius_y,
            2.0 * eye_radius_x,
            2.0 * eye_radius_y,
        ],
        c.transform,
        g,
    );

    // Calculate pupil position based on mouse
    let mut dx = mouse_x - eye_x;
    let mut dy = mouse_y - eye_y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance > 0.0 {
        dx /= distance; // Normalize direction vector
        dy /= distance;
    }

    // Constrain pupil within the eye
    let pupil_x = eye_x + eye_radius_x * 0.6 * dx;
    let pupil_y = eye_y + eye_radius_y * 0.6 * dy;

    // Draw the pupil
    ellipse(
        [1.0, 1.0, 1.0, 1.0], // White color
        [
            pupil_x - pupil_radius,
            pupil_y - pupil_radius,
            2.0 * pupil_radius,
            2.0 * pupil_radius,
        ],
        c.transform,
        g,
    );
}
