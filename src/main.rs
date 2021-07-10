use circle_collision::balls::Drawable;
use circle_collision::grfx::render::Render2D;

fn main() {
    let window = Drawable::new(
        String::from("Events and Update Test - Moving Circles"),
        800,
        600,
    );
    window.start();
}
