use renderer::init;

fn main() {
    pollster::block_on(init());
}
