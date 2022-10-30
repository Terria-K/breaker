use std::collections::HashMap;

use bevy_ecs::{
    prelude::Entity,
    query::{With, Without},
    system::{Query, Res, ResMut},
};
use macroquad::{prelude::Rect, audio::{Sound, play_sound_once}};

use crate::{
    component::{resource::ColliderResource, Aabb, BallType, Lives, Position, Tags, Velocity},
    game::States,
};

pub fn query_all_colliders(
    mut colliders: ResMut<ColliderResource>,
    query: Query<(Entity, &Aabb, &Position), Without<Velocity>>,
) {
    colliders.0 = query
        .iter()
        .map(|(e, x, pos)| (e, Rect::new(pos.0.x + x.0.x, pos.0.y + x.0.y, x.0.w, x.0.h)))
        .collect::<Vec<(Entity, Rect)>>();
}

pub fn system(
    mut query: Query<(&mut Position, &mut Velocity, &Aabb, &Tags)>,
    mut block_query: Query<&mut Lives, With<Tags>>,
    colliders: Res<ColliderResource>,
    sound: Res<HashMap<String, Sound>>,
    mut state: ResMut<States>,
) {
    let colliders = &colliders.0;
    if colliders.len() <= 1 {
        *state = States::Restarting;
    }

    for (mut pos, mut vel, aabb, tags) in query.iter_mut() {
        let rect = Rect::new(pos.0.x + aabb.0.x, pos.0.y + aabb.0.y, aabb.0.w, aabb.0.h);

        let on_collide = |e: Entity, aabb: &Rect, intersection: Rect| {
            let a_center = aabb.point() + aabb.size() * 0.5f32;
            let b_center = rect.point() + rect.size() * 0.5f32;
            let to = a_center - b_center;
            let to_signum = to.signum();
            if intersection.w > intersection.h {
                pos.0.y -= to_signum.y * intersection.h;
                vel.0.y = -to_signum.y * vel.0.y.abs();
            } else {
                pos.0.x -= to_signum.x * intersection.w;
                vel.0.x = -to_signum.x * vel.0.x.abs();
            }
            if let Ok(mut lives) = block_query.get_component_mut::<Lives>(e) {
                if let Tags::Ball(BallType::Special) = tags {
                    lives.0 = 0;
                } else {
                    lives.0 -= 1;
                }
                let sound = sound["HitBlock"];
                play_sound_once(sound);
            }
        };

        collide(&rect, colliders, on_collide);
    }
}

fn collide<F>(a: &Rect, b: &[(Entity, Rect)], f: F)
where
    F: FnOnce(Entity, &Rect, Rect),
{
    for (entity, collider) in b {
        if let Some(intersection) = collider.intersect(*a) {
            f(*entity, collider, intersection);
            return;
        }
    }
}
