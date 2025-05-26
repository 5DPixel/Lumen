use vulkan_engine::window;

fn main() {
    pollster::block_on(window::run());
}
