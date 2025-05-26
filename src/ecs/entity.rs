#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entity(u32);

impl Entity {
    pub fn id(&self) -> u32 {
        self.0
    }

    pub fn from_id(id: u32) -> Self {
        Entity(id)
    }
}

pub struct EntityManager {
    next_id: u32,
    recycled: Vec<u32>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            recycled: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        if let Some(id) = self.recycled.pop() {
            Entity(id)
        } else {
            let id = self.next_id;
            self.next_id += 1;
            Entity(id)
        }
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        if entity.id() < self.next_id {
            self.recycled.push(entity.id());
        }
    }
}
