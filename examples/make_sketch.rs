use crab_edu::{Sketch, AppExit, Color};

fn main() -> AppExit {
    let mut sketch = Sketch::new();
    sketch.spawn_pen(Color::srgb(1.0, 0., 0.));
    sketch.run()
}
