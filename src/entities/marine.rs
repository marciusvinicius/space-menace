use amethyst::{
    assets::{Handle, Prefab},
    core::{math::{Vector2, Vector3}, Transform},
    ecs::prelude::World,
    prelude::Builder,
    renderer::transparent::Transparent,
};

use crate::{
    components::{
        Animation,
        AnimationId,
        AnimationPrefabData,
        Orientation,
        Orientations,
        Marine,
        Motion,
        TwoDimObject
    },
    resources::Context,
};

pub fn load_marine(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>, ctx: &Context) {
    let mut transform = Transform::default();
    let scale = ctx.scale;

    transform.set_scale(Vector3::new(scale, scale, scale));

    let mut two_dim_object = TwoDimObject::new(32. * scale, 36. * scale);
    two_dim_object.set_position(384., 176.);
    two_dim_object.update_transform_position(&mut transform);

    world
        .create_entity()
        .with(Marine::new())
        .with(two_dim_object)
        .with(transform)
        .with(Motion::new(Vector2::new(384., 176.)))
        .with(Animation {
            current: AnimationId::Idle,
            types: vec![
                AnimationId::Die,
                AnimationId::Idle,
                AnimationId::Jump,
                AnimationId::Move,
                AnimationId::Shoot,
            ],
        })
        .with(prefab)
        .with(Orientation::new(Orientations::Normal, Orientations::Normal))
        .with(Transparent) // Necessary for ordered layering
        .build();
}
