#![feature(int_roundings)] // enable use of ceiling division unstsable feature

use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowMode, WindowResolution},
};
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

mod layers;

const NUM_CANDY: u32 = 10;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.))) //set background color
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Candy Count".into(),
                        resolution: WindowResolution::new(3840., 2160.)
                            .with_scale_factor_override(12.),
                        mode: WindowMode::BorderlessFullscreen,
                        resizable: false,
                        enabled_buttons: EnabledButtons {
                            maximize: false,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default(),
            // PhysicsDebugPlugin::default(), // shows hitboxes, etc
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    spawn_text(&mut commands, &asset_server);

    //spawn the jar, with a collider
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("jar-front.png"),
                transform: Transform {
                    translation: Vec3::new(80., -0., layers::JAR),
                    ..default()
                },
                ..default()
            },
            RigidBody::Static,
        ))
        .with_children(|children| {
            children.spawn(SpriteBundle {
                texture: asset_server.load("jar-back.png"),
                transform: Transform::from_translation((0., 0., -2.).into()),
                ..default()
            });
            children.spawn(Collider::segment((-49., 70.).into(), (-49., -70.).into()));
            children.spawn(Collider::segment((-49., -65.).into(), (0., -72.).into()));
            children.spawn(Collider::segment((0., -72.).into(), (50., -65.).into()));
            children.spawn(Collider::segment((50., -70.).into(), (49., 70.).into()));
        });

    spawn_candy(1000, &mut commands, &asset_server)
}

fn spawn_text(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let style = TextStyle {
        font: asset_server.load("fonts/MerriweatherSans-Regular.ttf"),
        font_size: 7.,
        color: Color::WHITE,
        ..default()
    };

    commands.spawn(
        TextBundle::from_sections([
            TextSection {
                value: "\
Hi! Welcome to Candy Count!

A random number of marbles between 500 and
1200 and just been dropped into the jar.
To Play, Type your guess into the text
field and press enter.

Your guess: "
                    .to_string(),
                style: style.clone(),
            },
            TextSection {
                value: "".to_string(), // guess
                style: TextStyle {
                    font: asset_server.load("fonts/Sentient-Bold.ttf"),
                    font_size: 7.,
                    ..default()
                },
            },
            TextSection {
                value: "".to_string(), // if the guess was too high or low
                style: style.clone(),
            },
            TextSection {
                value: "\n\nThe number is smaller than ".to_string(),
                style: style.clone(),
            },
            TextSection {
                value: "1200".to_string(),
                style: style.clone(),
            },
            TextSection {
                value: "\nThe number is larger than ".to_string(),
                style: style.clone(),
            },
            TextSection {
                value: "500".to_string(),
                style: style.clone(),
            },
            TextSection {
                value: "\nGuesses left: ".to_string(),
                style: style.clone(),
            },
            TextSection {
                value: "".to_string(),
                style: TextStyle {
                    font_size: 7.,
                    ..default()
                },
            },
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.),
            left: Val::Px(5.),
            ..default()
        }),
    );
}

fn spawn_candy(amount: i32, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    for y in 0..(amount / 20 + 1) {
        for x in -10..(if y == amount / 20 {
            // we never get to amount/20+1, amount/20 is the last
            (amount % 20) - 10 // isolate the remainder, and subtract 10 since we start at -10
        } else {
            10
        }) {
            commands.spawn((
                // possible indirection needed
                SpriteBundle {
                    texture: random_candy(asset_server),
                    transform: Transform::from_translation(Vec3::new(
                        // prevent them  from being perfectly uniform so they fall nicely
                        (x as f32 * 4.5 + (if (y % 2) == 0 { 1. } else { -1. })) + 80.,
                        (y as f32 * 4.) + 60.,
                        layers::MARBLES,
                    )),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(1.7),
            ));
        }
    }
}

fn random_candy(asset_server: &Res<AssetServer>) -> Handle<Image> {
    let candy_num = rand::thread_rng().gen_range(0..=(NUM_CANDY - 1)); // gen_range is inclusive
    asset_server.load(format!("candy{}.png", candy_num))
}
