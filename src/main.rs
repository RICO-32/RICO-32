use rico_32::engine::rico::RicoEngine;

fn main() {
    let engine = RicoEngine::default();
    engine.start().expect("Couldn't start the RICO-32 Engine!");
}
