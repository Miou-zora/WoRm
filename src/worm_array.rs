use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Default)]
struct WormParts(Vec<Entity>);

#[derive(Component, Default)]
pub struct Worm {
    parts: WormParts,
}

const FIRST_SEGMENT_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);
const SECOND_SEGMENT_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);
const FIRST_POS_WORM: Vec3 = Vec3::new(50.0, 50.0, 1.0);
const SECOND_POS_WORM: Vec3 = Vec3::new(10.0, 10.0, 1.0);

pub fn spawn_worm(mut commands: Commands) {
    let first_segment = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: FIRST_SEGMENT_COLOR,
                ..Default::default()
            },
            transform: Transform {
                translation: FIRST_POS_WORM,
                scale: Vec3::new(100.0, 100.0, 100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
    let mut segments: Vec<Entity> = vec![first_segment];
    for _ in 0..10 {
        let segment = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: SECOND_SEGMENT_COLOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: SECOND_POS_WORM,
                    scale: Vec3::new(60.0, 60.0, 60.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();
        segments.push(segment);
    }
    commands.spawn(Worm {
        parts: WormParts(segments),
    });
}

pub fn update_worm_pos(
    mut query: Query<&mut Worm>,
    mut positions: Query<&mut Transform>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_windows.iter().next().unwrap();
    let window_size = Vec2::new(window.width(), window.height());
    if !window.cursor_position().is_some() {
        return;
    }
    let mouse_position_from_window = window.cursor_position().unwrap();
    let mouse_position = Vec2::new(
        mouse_position_from_window.x - window_size.x / 2.0,
        -mouse_position_from_window.y + window_size.y / 2.0,
    );

    for worm in query.iter_mut() {
        let segment_positions = worm
            .parts
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Transform>>();
        positions.get_mut(worm.parts.0[0]).unwrap().translation = mouse_position.extend(1.0);
        segment_positions
            .iter()
            .zip(worm.parts.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                let actual_pos = positions.get_mut(*segment).unwrap().translation;
                let distance = pos.translation.distance(actual_pos);
                if distance > 41.0 {
                    let direction = pos.translation - actual_pos;
                    if direction == Vec3::ZERO { return; }
                    let direction = direction.normalize();
                    let new_pos = actual_pos + direction * (distance - 40.0);
                    positions.get_mut(*segment).unwrap().translation = new_pos;
                } else if distance < 39.0 {
                    let direction = pos.translation - actual_pos;
                    if direction == Vec3::ZERO { return; }
                    let direction = direction.normalize();
                    let new_pos = actual_pos - direction * (40.0 - distance);
                    positions.get_mut(*segment).unwrap().translation = new_pos;
                }
            });
    }
}
