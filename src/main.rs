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
                texture: asset_server.load("jar-front.png"),
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
            children.spawn(SpriteBundle {
                texture: asset_server.load("jar-back.png"),
                transform: Transform::from_translation((0., 0., -2.).into()),
                ..default()
            });
            children.spawn(Collider::segment((-49., 50.).into(), (-49., -70.).into()));
            children.spawn(Collider::segment((-49., -65.).into(), (0., -72.).into()));
            children.spawn(Collider::segment((0., -72.).into(), (49., -65.).into()));
            children.spawn(Collider::segment((49., -70.).into(), (49., 50.).into()));
        });

        spawn_candy(30, &commands, &asset_server)

    // for x in -10..10 {
    //     for y in 0..40 {
    //         commands.spawn((
    //             SpriteBundle {
    //                 texture: random_candy(&asset_server),
    //                 transform: Transform::from_translation(Vec3::new(
    //                     x as f32 * 4.5,
    //                     (y as f32 * 4.) + 60.,
    //                     layers::MARBLES,
    //                 )),
    //                 ..default()
    //             },
    //             RigidBody::Dynamic,
    //             Collider::ball(1.7),
    //         ));
    //     }
    // }
}

fn spawn_candy(amount: u32, commands: &Commands, asset_server: &Res<AssetServer>) {
    for y in 0..((amount / 20) + 1) { // integer division intended
        for x in -10..(
            if (y == amount / 20) { // we never get to amount/20+1, amount/20 is the last
                ((amount - (amount % 20)) - 10) // isolate the remainder, and subtract 10 since we start at -10
            } else {
                10
            }
        ) {
            commands.spawn(( // possible indirection needed
                SpriteBundle {
                texture: random_candy(&asset_server),
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * 4.5,
                    (y as f32 * 4.) + 60.,
                    layers::MARBLES,
                )),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(1.7),
        ))
        }
    }
}

fn random_candy(asset_server: &Res<AssetServer>) -> Handle<Image> {
    let candy_num = rand::thread_rng().gen_range(0..=(NUM_CANDY - 1)); // gen_range is inclusive
    asset_server.load(format!("candy{}.png", candy_num))
}
