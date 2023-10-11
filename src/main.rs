use std::collections::HashMap;

use bevy::{
    prelude::*,
    window::{ Window, WindowMode },
};

#[derive(Component, Clone, Debug)]
struct Point {
    position: Vec3,
    level: usize,
}

#[derive(Resource)]
struct Line {
    start: Entity,
    end: Entity,
    level: u32,
}

#[derive(Default, Resource)]
struct BezierData {
    current_level: usize,
    lines: HashMap<Entity, Line>,
    points: HashMap<Entity, Point>,
    point_mesh: Handle<Mesh>,
    point_material: Handle<StandardMaterial>,
}

fn setup(
    mut commands: Commands,
    mut bezier_data: ResMut<BezierData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            viewport_origin: Vec2::new(0.0, 0.0),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 10.0),  // Set Z position to 10.0
        ..Default::default()
    });

    let point_mesh = meshes.add(Mesh::from(shape::Circle { radius: 100.0, ..Default::default() }));
    let point_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.0, 0.0).into(),
        ..Default::default()
    });

    bezier_data.point_mesh = point_mesh;
    bezier_data.point_material = point_material;
    bezier_data.current_level = 0;
}

fn mouse_click_system(
    mut commands: Commands,
    mut bezier_data: ResMut<BezierData>,
    query: Query<&Window>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for window in query.iter() {
            let cursor_position = window.cursor_position().unwrap();
            let world_position = Vec3::new(cursor_position.x, cursor_position.y, 0.0);

            let point = Point { position: world_position, level: bezier_data.current_level };
            let transform = Transform::from_xyz(world_position.x, world_position.y, 0.0);

            let point_entity = commands.spawn((point.clone(), transform)).id();

            bezier_data.points.insert(point_entity, point);
        }
    }
}

fn render_points_system(
    bezier_data: Res<BezierData>,
    mut commands: Commands,
) {
    for (entity, point) in bezier_data.points.iter() {
        let transform = Transform::from_xyz(point.position.x, point.position.y, 0.0);

        commands.entity(*entity)
            .insert(PbrBundle {
                mesh: bezier_data.point_mesh.clone(),
                material: bezier_data.point_material.clone(),
                transform,
                ..Default::default()
            });
    }
}

fn log_system(
    query: Query<&Window>,
    bezier_data: Res<BezierData>,
) {
    for window in query.iter() {
        if let Some(cursor_position) = window.cursor_position() {
            println!("Mouse Position: {:?}", cursor_position);
        }
    }

    println!("Points:");
    for (entity, point) in bezier_data.points.iter() {
        println!("{:?} - {:?}", entity, point);
    }
}

fn log_entity_contents_system(
    query: Query<(Entity, &Point, &Handle<Mesh>, &Handle<StandardMaterial>)>,
) {
    for (entity, point, mesh_handle, material_handle) in query.iter() {
        println!("Entity {:?}: Point: {:?}, Mesh Handle: {:?}, Material Handle: {:?}", entity, point, mesh_handle, material_handle);
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
        )
        .insert_resource(BezierData::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (mouse_click_system, render_points_system, log_entity_contents_system))
        .run();
}

