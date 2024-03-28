use std::collections::HashMap;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;


// #[derive(Component)]
// pub struct Head;

#[derive(Default)]
pub struct Path {
    points: Vec<Vec2>, // 2D because 3D is GWA GWA
}

#[derive(Component, Default)]
pub struct Worm {
    path: Path,
    map_entity_index: HashMap<Entity, usize>,
}

#[derive(Component)]
struct WormPart;

pub fn setup_head(mut commands: Commands) {
    // let head = commands.spawn(SpriteBundle { ..default() })
    //     // .insert(Head)
    //     .insert(Name::new(" WormHead")).id();

    commands.spawn(
        Worm {
            map_entity_index: HashMap::new(),
            ..default()
        })
        .insert(Name::new("Worm"));
        // .push_children(&[head]);

}

pub fn update_head(
    mut q_worm: Query<&mut Worm>,
    q_window: Query<&Window, With<PrimaryWindow>>
)
{
    let window = q_window.iter().next().unwrap();
    if window.cursor_position().is_none() { return; }
    let mouse_pos = window.cursor_position().unwrap();
    let cursor_pos = Vec2::new(mouse_pos.x - window.width() / 2.0, -mouse_pos.y + window.height() / 2.0);

    let path_len = q_worm.iter().next().unwrap().path.points.len();
    if path_len >= 1 {
        q_worm.iter_mut().next().unwrap().path.points[path_len - 1] = cursor_pos;
    }
}

pub fn update_worm(
    mut q_worm: Query<&mut Worm>,
    // q_head: Query<&Transform, With<Head>>
)
{
    // let head_pos = q_head.iter().next().unwrap().translation;
    let mut worm = q_worm.iter_mut().next().unwrap();

    // Create 2 paths point if it doesn't exist
    if worm.path.points.is_empty() {
        let default_head_pos = Vec2::new(0.0, 0.0);
        spawn_path_point(default_head_pos, &mut worm);
        spawn_path_point(default_head_pos, &mut worm);
        return;
    }

    // update head pos
    // worm_path_len must be >= 2 (it's always true because we spawn 2 points)
    // worm.path.points[worm_path_len - 1] = head_pos.xy();

    // get head - 1 pos
    let worm_path_len = worm.path.points.len();
    let before_last_path_pos_entity = worm.path.points[worm_path_len - 2];
    let last_path_pos_entity = worm.path.points[worm_path_len - 1];
    let distance_between_last_and_head = before_last_path_pos_entity.distance(last_path_pos_entity);

    // make first point follow second with
    if worm.path.points.len() == 10 {
        let vector_between_first_and_second = worm.path.points[0] - worm.path.points[1];
        let normalized_vector = vector_between_first_and_second.normalize();
        let new_point = worm.path.points[1] + normalized_vector * (30.0 - distance_between_last_and_head);
        worm.path.points[0] = new_point;
    }

    // spawn new path point if distance between last path point and head is > 30
    if before_last_path_pos_entity.distance(last_path_pos_entity) > 30.0 {
        spawn_path_point(last_path_pos_entity, &mut worm);
        if worm.path.points.len() > 10 {
            // remove last point
            worm.path.points.remove(0);
            // worm.path.points.remove();
        }
    }
}

fn spawn_path_point(head_pos: Vec2, worm: &mut Mut<Worm>) {
    worm.path.points.push(head_pos);
}

pub fn debug_draw_path(
    mut gizmos: Gizmos,
    q_worm: Query<&Worm>,
) {
    for worm in q_worm.iter() {
        if worm.path.points.len() < 1 { continue; }
        if worm.path.points.len() >= 3 {
            for i in 1..worm.path.points.len() - 1 {
                gizmos.circle_2d(worm.path.points[i], 5.0, Color::WHITE);
            }
        }
        if worm.path.points.len() >= 2 {
            gizmos.circle_2d(worm.path.points[0], 7.0, Color::GREEN);
        }
        gizmos.circle_2d(worm.path.points[worm.path.points.len() - 1], 7.0, Color::RED);
    }
}


// pub fn debug_draw_head(
//     mut gizmos: Gizmos,
//     q_head: Query<&Transform, With<Head>>
// ) {
//     for head in q_head.iter() {
//         gizmos.circle_2d(head.translation.xy(), 7.0, Color::RED);
//     }
//     // TODO: draw bezier curve or spline
// }

