/**
 * This file is part of the "worm" project.
 * It doesn't work at all.
 */
use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use bevy::window::PrimaryWindow;

#[derive(Component, Default)]
pub struct Tail(pub Option<Entity>);

#[derive(Bundle, Default)]
pub struct Worm<M: Material2d> {
    pub material_mesh2d: MaterialMesh2dBundle<M>,
    pub tail: Tail,
}

pub fn update_pos(child: &mut Mut<Transform>, parent: &mut Mut<Transform>) {
    const DISTANCE_WANTED: f32 = 50.0;
    let parent_pos = parent.translation;
    let actual_pos = child.translation;
    let distance = parent_pos.distance(actual_pos);

    if distance > DISTANCE_WANTED {
        let direction = parent_pos - actual_pos;
        let direction = direction.normalize();
        let new_pos = actual_pos + direction * (distance - DISTANCE_WANTED);
        child.translation = new_pos;
    } else if distance < DISTANCE_WANTED {
        let direction = parent_pos - actual_pos;
        let direction = direction.normalize();
        let new_pos = actual_pos + direction * (DISTANCE_WANTED - distance);
        child.translation = new_pos;
    }
}

#[derive(Component, Default)]
pub struct WormHead;

#[derive(Bundle, Default)]
pub struct WormHeadBundle<M: Material2d> {
    pub worm: Worm<M>,
    pub head: WormHead,
}

pub fn update_worm(
    mut query: Query<(&mut Transform, &mut Tail), With<WormHead>>,
    mut useful_query: Query<(&mut Transform, &mut Tail), Without<WormHead>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_windows.iter().next().unwrap();
    let window_size = Vec2::new(window.width(), window.height());
    if !window.cursor_position().is_some() {
        return;
    }
    for (mut transform, mut tail) in &mut query {
        let mouse_position_from_window = window.cursor_position().unwrap();
        let mouse_position = Vec2::new(
            mouse_position_from_window.x - window_size.x / 2.0,
            -mouse_position_from_window.y + window_size.y / 2.0,
        );
        transform.translation = Vec3::new(mouse_position.x, mouse_position.y, 0.0);
        // let tail_transform = useful_query.get_mut(tail_entity).unwrap();
        // update_pos(tail_transform.0, child, tail_transform.1, useful_query);
        // if tail.0.is_none() {
        //     continue;
        // }
        // let mut child_query_result = useful_query.get_mut(tail.0.unwrap());
        // let mut parent_transform: Mut<Transform> = transform;
        // while child_query_result.is_some() {
        //     let mut child_tail = child_query_result.unwrap();
        //     let mut child_transform = &child_tail.0;
        //     let mut child = child_tail.1.unwrap();
        //     println!("child: {:?}", child);
        //     update_pos(&mut child_transform, &mut parent_transform);
        //     if child_tail.0.is_none() {
        //         break;
        //     }
        //     child = child_tail.0.unwrap();
        //     parent_transform = child_transform;
        //     child_query_result = useful_query.get_mut(child);
        // }
    }
}
