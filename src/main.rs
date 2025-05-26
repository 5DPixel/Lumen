use vulkan_engine::{ecs::{components::{SerializableVector3, Transform}, world::World}, renderer::window};
use vulkan_engine::ecs::entity::Entity;

fn main() {
    //pollster::block_on(window::run());

    let mut world = World::new();

    world.register_component::<Transform>();

    //Write scene

    // world.register_component::<Transform>();
    // let entity = world.create_entity();

    // let transform = Transform {
    //     position: SerializableVector3::new(0.0, 0.0, 0.0),
    //     rotation: SerializableVector3::new(0.0, 0.0, 0.0),
    //     scale: SerializableVector3::new(1.0, 1.0, 1.0),
    // };

    //world.add_component(entity, transform);
    //world.save_to_file("scene1.lscn");

    world.load_from_file("scene1.lscn");

    let entity = Entity::from_id(0);

    if let Some(transform) = world.get_component::<Transform>(entity) {
        println!("Transform for entity {:?}: {:?}", entity, transform);
    } else {
        println!("Entity {:?} has no Transform component.", entity);
    }
}
