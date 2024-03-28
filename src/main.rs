use bevy::prelude::*;

use crate::worm_path::{debug_draw_path, setup_head, update_head, update_worm};

mod worm_path;

// use crate::worm_linked_list::{Tail, update_worm, Worm, WormHeadBundle};

// mod worm_linked_list;

// #[derive(Component, Default)]
// struct WormTail(Option<Entity>);
//
// // Tag
// #[derive(Component, Default)]
// struct HeadWorm;
//
// #[derive(Bundle, Default)]
// struct WormBundle<M: Material2d> {
//     material_mesh2d_bundle: MaterialMesh2dBundle<M>,
//     tail: WormTail,
// }

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn(Camera2dBundle::default());
//
//     let tail = commands
//         .spawn(Worm {
//             tail: Default::default(),
//             material_mesh2d: MaterialMesh2dBundle {
//                 mesh: meshes.add(Circle::new(35.0)).into(),
//                 material: materials.add(Color::PURPLE),
//                 ..default()
//             },
//         })
//         .id();
//
//     commands.spawn(WormHeadBundle {
//         head: Default::default(),
//         worm: Worm {
//             tail: Tail(Option::from(tail)),
//             material_mesh2d: MaterialMesh2dBundle {
//                 mesh: meshes.add(Circle::new(50.0)).into(),
//                 material: materials.add(Color::CRIMSON),
//                 ..default()
//             },
//         },
//     });
// }

// fn move_head_worm(
//     mut meshs: Query<&mut Transform, (With<Mesh2dHandle>, With<HeadWorm>)>,
//     q_windows: Query<&Window, With<PrimaryWindow>>,
// ) {
//     let window = q_windows.iter().next().unwrap();
//     let window_size = Vec2::new(window.width(), window.height());
//
//     for mut transform in meshs.iter_mut() {
//         if window.cursor_position().is_some() {
//             let mouse_position_from_window = window.cursor_position().unwrap();
//             let mouse_position = Vec2::new(
//                 mouse_position_from_window.x - window_size.x / 2.0,
//                 -mouse_position_from_window.y + window_size.y / 2.0,
//             );
//             transform.translation = Vec3::new(mouse_position.x, mouse_position.y, 0.0);
//         }
//     }
// }

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_head))
        .add_systems(Update, ((update_head, update_worm).chain(),
                              (debug_draw_path).chain()))
        .run();
}
