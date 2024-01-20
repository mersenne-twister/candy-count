use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowMode, WindowResolution},
};
use bevy_xpbd_2d::prelude::*;

mod layers;

fn main() {
    App::new()
        .add_plugins(
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
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server.load("jar.png"),
        transform: Transform {
            translation: Vec3::new(0., 5., layers::JAR),
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("table.png"),
        transform: Transform {
            translation: Vec3::new(0., 0., layers::TABLE),
            ..default()
        },
        ..default()
    });
}
