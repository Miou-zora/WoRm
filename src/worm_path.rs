use bevy::prelude::*;
use bevy::window::PrimaryWindow;


#[derive(Component)]
pub struct Head;

#[derive(Default)]
pub struct Path {
    points: Vec<Vec2>, // 2D because 3D is GWA GWA
}

#[derive(Component, Default)]
pub struct Worm {
    path: Path,
    head: Option<Entity>,
}

#[derive(Component)]
struct TailPart;

pub fn setup_head(mut commands: Commands) {
    let head = commands.spawn(SpriteBundle { ..default() })
        .insert(Head)
        .insert(Name::new("WormHead")).id();

    commands.spawn(
        Worm {
            head: Some(head),
            ..default()
        })
        .insert(Name::new("Worm"))
        .push_children(&[head]);

}

pub fn update_head(
    mut q_head: Query<(&Head, &mut Transform)>,
    q_window: Query<&Window, With<PrimaryWindow>>
)
{
    let window = q_window.iter().next().unwrap();
    if window.cursor_position().is_none() { return; }
    let mouse_pos = window.cursor_position().unwrap();
    let cursor_pos = Vec2::new(mouse_pos.x - window.width() / 2.0, -mouse_pos.y + window.height() / 2.0);

    q_head.iter_mut().next().unwrap().1.translation = Vec3::new(cursor_pos.x, cursor_pos.y, 0.0);
}

pub fn update_worm(
    mut q_worm: Query<&mut Worm>,
    q_head: Query<&Transform, With<Head>>
)
{
    let head_pos = q_head.iter().next().unwrap().translation;
    let mut worm = q_worm.iter_mut().next().unwrap();

    if worm.path.points.is_empty() {
        spawn_path_point(head_pos.xy(), &mut worm);
        spawn_path_point(head_pos.xy(), &mut worm);
        return;
    }
    let worm_path_len = worm.path.points.len();
    if worm_path_len >= 2 {
        worm.path.points[worm_path_len - 1] = head_pos.xy();
    }

    let mut last_path_pos_entity = *worm.path.points.last().unwrap();
    if worm_path_len >= 2 {
        last_path_pos_entity = worm.path.points[worm_path_len - 2];
    }
    // last
    if last_path_pos_entity.distance(head_pos.xy()) > 30.0 {
        spawn_path_point(head_pos.xy(), &mut worm);
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
        for i in 0..worm.path.points.len() {
            gizmos.circle_2d(worm.path.points[i], 5.0, Color::WHITE);
        }
    }
}


pub fn debug_draw_head(
    mut gizmos: Gizmos,
    q_head: Query<&Transform, With<Head>>
) {
    for head in q_head.iter() {
        gizmos.circle_2d(head.translation.xy(), 7.0, Color::RED);
    }
    // TODO: draw bezier curve or spline
}

