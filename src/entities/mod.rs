/* Usage

// create entities and containers

#[gen_containers]
pub struct EntityContainers {
    pub my_entity_container: EntityContainer<MyEntity>,
    pub my_animated_entity_container: EntityContainer<MyAnimatedEntity>,
}

// Static entity
#[gen_entity]
pub struct MyEntity {
    pub k: i32,
}


// Animated entity
#[gen_entity(my_animated_entity_container)]
pub struct MyAnimatedEntity {
    pub k: i32,
}



// changes in State
let mut entity_containers = EntityContainers::new();

let entity_id = entity_containers.create::<MyEntity>(
    Transform {
        pos: Vec2 { x: 100., y: 400. },
        rot: 0.,
        layer: 0,
    },
    Sprite {
        texture,
        texture_flip: TextureFlip::NO,
        uvs: (Vec2i { x: 0, y: 0 }, Vec2i { x: 32, y: 32 }),
        pivot: Vec2 { x: 16., y: 16. },
        size: Vec2 { x: 32., y: 32. },
    },
);

let animated_entity_id = entity_containers.create_animated::<MyEntity>(
    Transform {
        pos: Vec2 { x: 100., y: 200. },
        rot: 0.,
        layer: 0,
    },
    animation_set
);

let animated_entity = entity_containers.get_mut(animated_entity_id).unwrap();
animated_entity.play_animation(app);



// insert
self.entity_id = self.entity_containers.create::<MyEntity>(
    Transform {
        pos: Vec2 { x: 100., y: 400. },
        rot: 0.,
        layer: 0,
    },
    Sprite {
        texture: self.texture,
        texture_flip: TextureFlip::NO,
        uvs: (Vec2i { x: 0, y: 0 }, Vec2i { x: 32, y: 32 }),
        pivot: Vec2 { x: 16., y: 16. },
        size: Vec2 { x: 32., y: 32. },
    },
);

// remove
self.entity_containers.destroy(self.entity_id);

// update
if let Some(my_entity) = self.entity_containers.get_mut(self.entity_id) {
    my_entity.entity_mut().transform.pos +=
        100.0 * app.last_frame_duration() * move_direction;
}

// render
self.entity_containers.render(app);

*/

pub mod entity;
pub mod container;

pub use entity::*;
pub use container::*;

use crate::State;
use crate::app::{
    App,
    animation_system::{Animator, AnimationSet},
    id_manager::Id,
    imgui::ImDraw,
    transform::Transform,
};

use entity_macros::*;
