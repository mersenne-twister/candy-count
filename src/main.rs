use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowMode, WindowResolution},
};
use bevy_xpbd_2d::prelude::*;

mod layers;

fn main() {
    App::new()
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
            // PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    //spawn the jar, with a collider
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("jar.png"),
                transform: Transform {
                    translation: Vec3::new(0., 5., layers::JAR),
                    ..default()
                },
                ..default()
            },
            RigidBody::Static,
            // Collider::Comp
            // Collider::segment((-49., 50.).into(), (-49., -70.).into()),
            // Collider::segment((-49., -65.).into(), (0., -70.).into()),
            // Collider::segment((0., -70.).into(), (49., -65.).into()),
            // Collider::segment((49., -70.).into(), (49., 50.).into())
        ))
        .with_children(|children| {
            children.spawn(Collider::segment((-49., 50.).into(), (-49., -70.).into()));
            children.spawn(Collider::segment((-49., -65.).into(), (0., -72.).into()));
            children.spawn(Collider::segment((0., -72.).into(), (49., -65.).into()));
            children.spawn(Collider::segment((49., -70.).into(), (49., 50.).into()));
        });

    for x in -10..10 {
        for y in 80..100 {
            commands.spawn((
                SpriteBundle {
                    texture: random_candy(&asset_server),
                    transform: Transform::from_translation(Vec3::new(0., 50., layers::MARBLES)),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(1.7),
            ));
        }
    }
}

fn random_candy(asset_server: &Res<AssetServer>) -> Handle<Image> {
    asset_server.load("candy1.png")
}
