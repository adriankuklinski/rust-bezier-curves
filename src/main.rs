use bevy::{
    prelude::*,
    window::{Window, WindowMode},
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
        .run();
}

