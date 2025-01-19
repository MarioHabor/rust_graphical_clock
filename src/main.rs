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
    let text_offset_factor = 0.92;

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

            glyphs.factory.encoder.flush(device);
        });
    }
}
