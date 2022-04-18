use surge;
fn main() {
    
    let window = surge::Window::new();

    pollster::block_on(surge::Window::run());
}
