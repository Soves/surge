use surge;
fn main() {
    
    let game_instance = surge::GameInstance::new();

    pollster::block_on(game_instance.run());
}
