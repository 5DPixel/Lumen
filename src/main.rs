use vulkan_engine::ecs::{components::{MeshData, ModelRenderer, SerializableVector3, Transform}, world::World};
use vulkan_engine::ecs::entity::Entity;
use vulkan_engine::renderer::window;

fn main() {
    pollster::block_on(window::run());

    let mut world = World::new();

    world.register_component::<Transform>();
    world.register_component::<ModelRenderer>();

    //Write scene

    let entity = world.create_entity();

    let transform = Transform {
        position: SerializableVector3::new(0.0, 0.0, 0.0),
        rotation: SerializableVector3::new(0.0, 0.0, 0.0),
        scale: SerializableVector3::new(1.0, 1.0, 1.0),
    };

    let mesh_data = MeshData {
        name: String::from("cube.obj"),
    };

    let model_renderer = ModelRenderer {
        meshes: vec![mesh_data],
    };

    world.add_component(entity, transform);
    world.add_component(entity, model_renderer);
    world.save_to_file("scene1.lscn");

    world.load_from_file("scene1.lscn");

    let entity = Entity::from_id(0);

    if let Some(transform) = world.get_component::<ModelRenderer>(entity) {
        println!("ModelRenderer for entity {:?}: {:?}", entity, transform);
    } else {
        println!("Entity {:?} has no ModelRenderer component.", entity);
    }
}
