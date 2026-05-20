mod engine;
use engine::window::Window;

fn main() {
    let mut window = Window::new(1280, 720, "BRIXIT");

    while !window.should_close() {
        window.update();
        window.render();
    }
}
