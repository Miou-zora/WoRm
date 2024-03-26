use bevy::prelude::*;
use bevy::window::PrimaryWindow;


#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct PathPoint;

#[derive(Component)]
pub struct WormPath {
    path: Vec<Entity>, // 2D because 3D is GWA GWA
    head: Entity,
}

pub fn setup_head(mut commands: Commands) {
    let head = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Head).id();

    commands.spawn(WormPath {
        path: Vec::new(),
        head
    });
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

pub fn update_path(
    mut commands: Commands,
    mut q_path: Query<(&mut WormPath)>,
    q_head: Query<&Transform, With<Head>>,
    q_tail: Query<&Transform, With<PathPoint>>
)
{
    let head_pos = q_head.iter().next().unwrap().translation;
    let mut worm_path = q_path.iter_mut().next().unwrap();

    if worm_path.path.is_empty() {
        println!("Path is empty");
        worm_path.path.push(commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                ..Default::default()
            },
            transform: Transform {
                translation: head_pos,
                scale: Vec3::new(5.0, 5.0, 5.0),
                ..Default::default()
            },
            ..Default::default()
        })
            .insert(PathPoint)
            .id());
        return;
    }
    let last_tail_entity = worm_path.path.last().unwrap();
    let last_tail = q_tail.get(*last_tail_entity).unwrap();
    // if distance between head and last tail is greater than 10, add a new tail
    if head_pos.distance(last_tail.translation) > 30.0 {
        worm_path.path.push(commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                ..Default::default()
            },
            transform: Transform {
                translation: head_pos,
                scale: Vec3::new(5.0, 5.0, 5.0),
                ..Default::default()
            },
            ..Default::default()
        })
            .insert(PathPoint)
            .id());
    }
    if worm_path.path.len() > 10 {
        commands.entity(worm_path.path.remove(0)).despawn();
    }
}