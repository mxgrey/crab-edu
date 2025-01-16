use crab_edu::*;

fn main() -> AppExit {
    let mut sketch = Sketch::new();
    let mut pen = sketch.spawn_pen(Color::srgb(1.0, 0., 0.));
    pen.move_pen(Movement::relative(0.3, Direction::Forward));
    pen.move_pen(Movement::relative(0.2, Direction::Right));
    sketch.run()
}
