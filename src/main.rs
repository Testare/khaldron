use std::collections::HashSet;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::{WindowResized, WindowResolution, PrimaryWindow},
};
use bevy_editor_pls::prelude::*;
use bevy_ninepatch::NinePatchPlugin;
use bevy_rapier2d::prelude::*;

const ASPECT_RATIO: (f32, f32) = (800.0, 450.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    /*window: WindowDescriptor {
                        scale_factor_override: Some(2.),
                        title: "Khaldron".to_string(),
                        width: 1600.,
                        height: 900.,
                        ..Default::default()
                    },*/
                    primary_window: Some(Window {
                        title: String::from("Khaldron"),
                        resolution: WindowResolution::new(1600.0, 900.0).with_scale_factor_override(2.),
                        ..default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
                .set(LogPlugin {
                    level: Level::INFO,
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // .add_plugin(EditorPlugin)
        // .add_plugin(NinePatchPlugin::<()>::default())
        .add_startup_system(general_game_startup)
        .add_startup_system(game_startup)
        .add_startup_system(ui_setup_from_scene)
        .add_system(node_heirarchy_report)
        .add_system(scale_for_window)
        // .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.))
        .run();
}

fn general_game_startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn game_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraCode-Bold.ttf");
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Hello my bbs!\n\nI love you muchly!",
            TextStyle {
                color: Color::RED,
                font_size: 30.0,
                font,
            },
        )
        .with_alignment(TextAlignment::Center),
        ..Default::default()
    });
}

fn ui_setup_from_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene: Handle<DynamicScene> = asset_server.load("ui/ui.scn.ron");
    commands
        .spawn(DynamicSceneBundle { scene, ..default() })
        .insert(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Window UI Root"));
}

fn node_heirarchy_report(
    mut events: EventReader<WindowResized>,
    nodes: Query<(Entity, Option<&Parent>, Option<&Children>, Option<&Name>), With<Node>>,
) {
    for _ in events.iter() {
        for (entity, parent_opt, children_opt, name) in nodes.iter() {
            println!(
                "{:?}: {:?} -> {:?} -> {:?}",
                entity,
                parent_opt,
                name.map(|n| n.as_str()).unwrap_or("<nameless>"),
                children_opt
            );
        }
        println!("----")
    }
}

fn scale_for_window(mut windows: Query<(Entity, &mut Window), With<PrimaryWindow>>, mut resize_events: EventReader<WindowResized>) {
    // let window_ids: HashSet<WindowId> = resize_events.iter().map(|e| e.id).collect();

    let resized_windows: HashSet<Entity> = resize_events.iter().map(|e| {
        e.window
    }).collect();
    
    // for window_id in window_ids {

        // TODO iter_many_mut?
        // if let Some(window) = windows.get_mut(window_id) {
        for (_, mut window) in windows.iter_mut().filter(|(e, _)|resized_windows.contains(e)) {
            let (width, height) = (
                window.physical_width() as f32,
                window.physical_height() as f32,
            );
            let width_scaling = width / ASPECT_RATIO.0;
            let height_scaling = height / ASPECT_RATIO.1;
            if width_scaling <= height_scaling {
                window.resolution.set_scale_factor_override(Some(width_scaling as f64));
            } else {
                window.resolution.set_scale_factor_override(Some(height_scaling as f64));
            }
        }
    // }
}
