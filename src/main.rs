use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Candy Count".into(),
                resolution: (320., 180.).into(),
                mode: Fullscreen,
                resizable: false,
                enabled_buttons: EnabledButtons {
                    maximize: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .run();
}
