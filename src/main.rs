extern crate sfml;

use std::time::Duration;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;
use tracing::{debug, error, info, trace, warn};

// Arbitrary. Needs tweaking.
const TARGET_RENDER_TIME: Duration = Duration::from_millis(20);

fn main() {
    tracing_subscriber::fmt::init();

    dragonfly::test_logging();

    let mut window = RenderWindow::new(
        (800, 600),
        "SFML VertexArray accessors Example",
        Style::CLOSE,
        &Default::default(),
    )
    .unwrap();
    window.set_vertical_sync_enabled(true);

    let mut rectangle = RectangleShape::new();
    rectangle.set_size(Vector2f::new(100.0, 100.0));
    rectangle.set_fill_color(Color::rgb(255, 0, 100));
    rectangle.set_position(Vector2f::new(100.0, 100.0));

    let mut clock = dragonfly::Clock::new();
    let mut adjust_time = Duration::ZERO;
    loop {
        let _elapsed = clock.delta();
        // events
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => {
                    window.close();
                    return;
                }
                _ => {}
            }
        }

        // drawing
        window.clear(Color::BLACK);
        window.draw(&rectangle);
        window.display();
        let render_time = clock.split();
        if let Some(intended_sleep_time) = TARGET_RENDER_TIME
            .checked_sub(render_time)
            .and_then(|sleep_time| sleep_time.checked_sub(adjust_time))
        {
            std::thread::sleep(intended_sleep_time);
            // let adjust_time = actual_sleep_time - intended_sleep_time
        }
    }
}
