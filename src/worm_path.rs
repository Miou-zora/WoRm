use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct Path {
    points: Vec<Vec2>, // 2D because 3D is GWA GWA
    size: f32,
    number_of_points: usize,
}

impl Default for Path {
    fn default() -> Self {
        Path {
            points: Vec::new(),
            size: 400.0,
            number_of_points: 10,
        }
    }
}

#[derive(Component, Default)]
pub struct Worm {
    path: Path,
}

#[derive(Component)]
pub struct WormPart {
    index_pos: usize,
}

pub fn setup_head(mut commands: Commands) {
    commands.spawn(
        Worm {
            path: Path {
                points: Vec::new(),
                size: 1000.0,
                number_of_points: 50,
            },
            ..default()
        })
        .insert(Name::new("Worm"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .with_children(|parent| {
            for entity in 0..30 {
                parent.spawn((WormPart { index_pos: entity }, SpriteBundle {
                    sprite: Sprite {
                        color: Color::CRIMSON,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(30.0, 30.0, 0.0),
                        scale: Vec3::new(20.0, 20.0, 20.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }))
                    .insert(Name::new(format!("Tail_{}", entity)));
            }
        });
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
    mut q_worm: Query<(&mut Worm, &Children)>,
    q_childs: Query<(&GlobalTransform, &Name)>,
)
{
    let mut all_worm = q_worm.iter_mut().next().unwrap();
    let mut worm = all_worm.0;

    // Create 2 paths point if it doesn't exist
    if worm.path.points.is_empty() {
        for &children in all_worm.1 {
            let child = q_childs.get(children).unwrap();
            println!("{}: {}", child.1, child.0.translation());
        }
        let default_head_pos = Vec2::new(0.0, 0.0);
        spawn_path_point(default_head_pos, &mut worm);
        spawn_path_point(default_head_pos, &mut worm);
        return;
    }

    // update head pos
    let mut worm_path_len = worm.path.points.len();
    let before_last_path_pos_entity = worm.path.points[worm_path_len - 2];
    let last_path_pos_entity = worm.path.points[worm_path_len - 1];
    let distance_between_last_and_head = before_last_path_pos_entity.distance(last_path_pos_entity);

    // make first point follow second
    if worm.path.points.len() == worm.path.number_of_points {
        let vector_between_first_and_second = worm.path.points[0] - worm.path.points[1];
        let normalized_vector = vector_between_first_and_second.normalize();
        let new_point = worm.path.points[1] + normalized_vector * (worm.path.size as f32 / worm.path.number_of_points as f32 - distance_between_last_and_head);
        worm.path.points[0] = new_point;
    }

    // spawn new path point if distance between last path point and head is > 30
    if before_last_path_pos_entity.distance(last_path_pos_entity) > worm.path.size as f32 / (worm.path.number_of_points + 1) as f32 {

        spawn_path_point(last_path_pos_entity, &mut worm);
        let vec_bl_to_l = last_path_pos_entity - before_last_path_pos_entity;
        let next_spawn_point = before_last_path_pos_entity + vec_bl_to_l.normalize() * (worm.path.size as f32 / worm.path.number_of_points as f32);
        worm_path_len = worm.path.points.len();
        worm.path.points[worm_path_len - 2] = next_spawn_point;
        if worm.path.points.len() > worm.path.number_of_points {
            // remove last point
            worm.path.points.remove(0);
            // worm.path.points.remove();
        }
    }
}

fn find_pos_in_path(path: &Path, index: usize) -> Vec2
{
    if index > (path.number_of_points - 1) {
        return Vec2::new(1.0, 1.0);
    }

    let mut pos_in_path = path.size / path.number_of_points as f32 * index as f32;
    let mut reversed_vec = path.points.to_vec();
    reversed_vec.reverse();
    for i in 0..(path.points.len() - 2) {
        if (*reversed_vec.get(i).unwrap()).distance(*reversed_vec.get(i+1).unwrap()) >= pos_in_path {
            let vector_i_to_i1 = (*reversed_vec.get(i+1).unwrap()) - (*reversed_vec.get(i).unwrap());
            return *reversed_vec.get(i).unwrap() + pos_in_path * vector_i_to_i1.normalize();
        }
        pos_in_path -= (*reversed_vec.get(i).unwrap()).distance(*reversed_vec.get(i+1).unwrap());
    }
    return *path.points.get(0).unwrap();
}

pub fn update_worm_parts(
    mut q_parts: Query<(&mut Transform, &Parent, &WormPart)>,
    q_parents: Query<&Worm>,
) {
    for (mut transform, parent_entity, worm_part) in q_parts.iter_mut() {
        let parent = q_parents.get(parent_entity.get()).unwrap();
        transform.translation = find_pos_in_path(&parent.path, worm_part.index_pos).extend(1.0);
    }
}

fn spawn_path_point(next_pos: Vec2, worm: &mut Mut<Worm>) {
    worm.path.points.push(next_pos);
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
        gizmos.circle_2d(worm.path.points[worm.path.points.len() - 1], 7.0, Color::BLUE);
    }
    // TODO: draw bezier curve or spline
}
