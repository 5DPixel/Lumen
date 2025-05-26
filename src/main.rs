use vulkan_engine::renderer::window;

fn main() {
    pollster::block_on(window::run());
}
