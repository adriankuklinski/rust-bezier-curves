use bevy::{
    prelude::*,
    window::{ Window, WindowMode },
};

use std::collections::HashMap;

#[derive(Resource)]
struct PointMaterial(Handle<ColorMaterial>);

#[derive(Component, Clone, Debug)]
struct Point {
    position: Vec3,
    level: u32,
}

#[derive(Resource)]
struct Line {
    start: Entity,
    end: Entity,
    level: u32,
}

#[derive(Default, Resource)]
struct BezierData {
    points: HashMap<Entity, Point>,
    lines: HashMap<Entity, Line>,
    current_level: u32,
}

fn setup(
    mut commands: Commands,
    mut bezier_data: ResMut<BezierData>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let point_material = materials.add(Color::rgb(1.0, 0.0, 0.0).into());
    commands.insert_resource(PointMaterial(point_material));
    commands.spawn(Camera2dBundle::default());
    bezier_data.current_level = 0;
}

fn mouse_click_system(
    mut commands: Commands,
    bezier_data: ResMut<BezierData>,
    mouse_input: Res<Input<MouseButton>>,
    window: Window,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let cursor_position = window.cursor_position().unwrap();
        let world_position = Vec3::new(cursor_position.x, cursor_position.y, 0.0);

        let point = Point {position: world_position, level: bezier_data.current_level};
        let point_entity = commands.spawn_empty().insert(point.clone()).id();

        bezier_data.points.insert(point_entity, point);
    }
}

fn render_points_system(
    bezier_data: Res<BezierData>,
    mut query: Query<(&Point, &mut Transform)>,
) {
    for (point, mut transform) in query.iter_mut() {
        transform.translation = point.position;
    }
}

fn main() {
    App::new()
        .insert_resource(BezierData::default())
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_click_system)
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
        .run();
}

