use khaldron::{Cauldron, CauldronEvent, Ingredient};
use std::collections::HashMap;
use std::{collections::HashSet, time::Duration};

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    log::{Level, LogPlugin},
    prelude::*,
    time::common_conditions::on_timer,
    window::{PrimaryWindow, WindowResized, WindowResolution},
};

const ASPECT_RATIO: (f32, f32) = (800.0, 450.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Khaldron"),
                        resolution: WindowResolution::new(1600.0, 900.0)
                            .with_scale_factor_override(2.),
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
        .add_event::<CauldronEvent>()
        .add_startup_system(general_game_startup)
        .add_startup_system(game_startup)
        .add_startup_system(ui_setup_from_scene)
        .add_system(node_heirarchy_report)
        .add_system(scale_for_window)
        .add_system(ingredient_game_input)
        .add_system(khaldron::evaluate_chemistry_rules)
        .add_system(ingredient_game_report.run_if(on_timer(Duration::from_secs(3))))
        // .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.))
        .run();
}

fn ingredient_game_input(
    mut keyboard_event: EventReader<KeyboardInput>,
    mut cauldron_events: EventWriter<CauldronEvent>,
    ingredients: Query<(Entity, &Ingredient, &Name)>,
    cauldron: Query<Entity, With<Cauldron>>,
) {
    for keyboard_event in keyboard_event.iter() {
        if keyboard_event.state != ButtonState::Pressed || keyboard_event.key_code.is_none() {
            continue;
        }
        let ingredient_by_name: HashMap<&str, Entity> = ingredients
            .iter()
            .map(|(e, _, name)| (name.as_str(), e))
            .collect();
        // println!("{:?} pressed", keyboard_event.key_code);
        match keyboard_event.key_code.unwrap() {
            KeyCode::A => {
                for (_entity, ingredient, name) in ingredients.iter() {
                    println!("{} is a {:?} ingredient", name, ingredient.color);
                }
            }
            KeyCode::S => {
                if let Some(cauldron_entity) = cauldron.iter().next() {
                    println!("+ Add 1L of Pepper");
                    cauldron_events.send(CauldronEvent::Add {
                        cauldron: cauldron_entity,
                        ingredient: *ingredient_by_name.get("Pepper").unwrap(),
                        liters: 1.0f32,
                    });
                }
            }
            KeyCode::D => {
                if let Some(cauldron_entity) = cauldron.iter().next() {
                    println!("+ Add 1L of Eye of Newt");
                    cauldron_events.send(CauldronEvent::Add {
                        cauldron: cauldron_entity,
                        ingredient: *ingredient_by_name.get("Eye of Newt").unwrap(),
                        liters: 1.0f32,
                    });
                }
            }
            KeyCode::F => {
                if let Some(cauldron_entity) = cauldron.iter().next() {
                    println!("+ Add 1L of A Rock");
                    cauldron_events.send(CauldronEvent::Add {
                        cauldron: cauldron_entity,
                        ingredient: *ingredient_by_name.get("A Rock").unwrap(),
                        liters: 1.0f32,
                    });
                }
            }
            KeyCode::R => {
                println!("+ Stirring Counter Clockwise");
                if let Some(cauldron_entity) = cauldron.iter().next() {
                    cauldron_events.send(CauldronEvent::StirCounterClockwise(cauldron_entity));
                }
            }
            KeyCode::T => {
                println!("+ Stirring Clockwise");
                if let Some(cauldron_entity) = cauldron.iter().next() {
                    cauldron_events.send(CauldronEvent::StirClockwise(cauldron_entity));
                }
            }
            _ => {}
        }
    }
}

fn ingredient_game_report(
    cauldrons: Query<(Entity, &Cauldron)>,
    ingredients: Query<&Name, With<Ingredient>>,
) {
    for (_id, cauldron) in cauldrons.iter() {
        println!(
            "--- Cauldron Report ---\n * Temperature: {:?}",
            cauldron.temperature
        );
        for (ingredient_id, volume) in cauldron.ingredients.iter() {
            println!(
                " * {}L of {}",
                volume,
                ingredients.get(*ingredient_id).unwrap().as_str()
            );
        }
    }
}

fn general_game_startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn game_startup(mut commands: Commands) {
    khaldron::add_default_ingredients(&mut commands);
    commands.spawn(Cauldron::default());
}

fn ui_setup_from_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene: Handle<DynamicScene> = asset_server.load("ui/ui.scn.ron");
    commands.spawn(Name::new("Extra Entity"));
    commands.spawn(Name::new("Extra Entity 2"));
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

fn scale_for_window(
    mut windows: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    mut resize_events: EventReader<WindowResized>,
) {
    // let window_ids: HashSet<WindowId> = resize_events.iter().map(|e| e.id).collect();

    let resized_windows: HashSet<Entity> = resize_events.iter().map(|e| e.window).collect();

    // for window_id in window_ids {

    // TODO iter_many_mut?
    // if let Some(window) = windows.get_mut(window_id) {
    for (_, mut window) in windows
        .iter_mut()
        .filter(|(e, _)| resized_windows.contains(e))
    {
        let (width, height) = (
            window.physical_width() as f32,
            window.physical_height() as f32,
        );
        let width_scaling = width / ASPECT_RATIO.0;
        let height_scaling = height / ASPECT_RATIO.1;
        if width_scaling <= height_scaling {
            window
                .resolution
                .set_scale_factor_override(Some(width_scaling as f64));
        } else {
            window
                .resolution
                .set_scale_factor_override(Some(height_scaling as f64));
        }
    }
    // }
}
