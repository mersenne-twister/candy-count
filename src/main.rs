#![feature(int_roundings)] // enable use of ceiling division unstable feature

// global imports
use {
    bevy::{
        prelude::*,
        render::color::Color,
        window::{EnabledButtons, PrimaryWindow, WindowMode, WindowResolution},
    },
    bevy_xpbd_2d::prelude::*,
    rand::Rng,
};

// wasm imports
#[cfg(target_family = "wasm")]
use web_sys::window;

mod layers;

const NUM_CANDY: u32 = 10;

#[derive(Resource)]
struct Secret {
    number: u32,
}
impl Secret {
    fn new() -> Self {
        Self {
            // TODO: have one for <1000 and >1000, and use a bournoulli distribution waited torwards
            // the former to pick which one.
            number: rand::thread_rng().gen_range(Secret::MIN..=Secret::MAX),
        }
    }
    const MAX: u32 = 1000;
    const MIN: u32 = 300;
}

#[derive(Resource)]
struct Guess {
    guess: u64,
}
impl Guess {
    fn default() -> Self {
        Self { guess: 0 }
    }
}

#[derive(Component)]
struct ThumbsUp;

#[derive(Component)]
struct SadFace;

#[derive(Resource)]
struct Guesses {
    guesses_left: u32,
}
impl Guesses {
    fn default() -> Self {
        Self {
            guesses_left: Self::MAX_GUESSES,
        }
    }
    const MAX_GUESSES: u32 = 10;
}

#[derive(Event)]
struct AttemptGuess;

fn main() {
    let mut app = App::new();

    app // setup window
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.))) //set background color
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Candy Count".into(),
                        resolution: WindowResolution::new(3860., 2160.)
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
        .insert_resource(Gravity(Vec2::NEG_Y * 25.))
        .insert_resource(Time::<Fixed>::from_seconds(0.017))
        .add_systems(Startup, setup)
        .insert_resource(Secret::new())
        .insert_resource(Guess::default())
        .insert_resource(Guesses::default())
        .add_event::<AttemptGuess>()
        .add_systems(Update, (input, guess, last_guess))
        .add_systems(FixedUpdate, (move_thumb, fade_face));

    // #[cfg(not(target_family = "wasm"))]
    // app.add_systems(Startup, set_scale);

    // #[cfg(target_family = "wasm")]
    // app.add_systems(Update, update_canvas_size);

    app.run();
}

/// a bad attempt at setting the resolution that just causes an arbitrary crash
#[cfg(not(target_family = "wasm"))]
fn set_scale(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window.get_single_mut().unwrap();

    println!("window dims: {}, {}", window.physical_width(), window.physical_height());

    // let width_ratio = window.physical_width() as f64 / 320.;
    // let height_ratio = window.physical_height() as f64 / 180.;

    // let scale_factor_override = f64::min(width_ratio, height_ratio);

    let scale_factor_override = window.physical_width() as f64 / 320.;

    println!("scat-fact: {}", scale_factor_override);

    // window.resolution.set_scale_factor(scale_factor_override);
    window.resolution.set_scale_factor_override(Some(scale_factor_override));
    // causes crash:
    //     wgpu error: Validation Error
    // Caused by:
    //     In Device::create_texture
    //       note: label = `main_texture_sampled`
    //     Not enough memory left
}

/// attempt to run full screen in browser, instead screw up aspect ratio and cause insane lag
#[cfg(target_family = "wasm")]
fn update_canvas_size(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    (|| {
        let mut window = window.get_single_mut().ok()?;
        let browser_window = web_sys::window()?;
        let width = browser_window.inner_width().ok()?.as_f64()?;
        let height = browser_window.inner_height().ok()?.as_f64()?;
        window
            .resolution
            .set(width as f32, height as f32);
        Some(())
    })();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, secret: Res<Secret>) {
    commands.spawn(Camera2dBundle::default());

    spawn_text(&mut commands, &asset_server);

    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/background.png"),
        transform: Transform::from_translation((0., 0., layers::BACKGROUND).into()),
        ..default()
    });

    //spawn the jar, with a collider
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/jar-front.png"),
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
                texture: asset_server.load("sprites/jar-back.png"),
                transform: Transform::from_translation((0., 0., -2.).into()),
                ..default()
            });
            children.spawn(Collider::segment((-49., 70.).into(), (-49., -70.).into()));
            children.spawn(Collider::segment((-49., -65.).into(), (0., -72.).into()));
            children.spawn(Collider::segment((0., -72.).into(), (50., -65.).into()));
            children.spawn(Collider::segment((50., -70.).into(), (49., 70.).into()));
        });

    spawn_candy(secret.number as i32, &mut commands, &asset_server)
}

fn input(
    mut text_inputs: EventReader<ReceivedCharacter>,
    key_input: Res<Input<KeyCode>>,
    mut guess: ResMut<Guess>,
    mut guess_text: Query<&mut Text>,
    mut attempt_guess_writer: EventWriter<AttemptGuess>,
) {
    for input in text_inputs.read() {
        if let Some(num) = input.char.to_digit(10) {
            println!("{}, {}", input.char, guess.guess);

            if guess.guess < (u64::MAX - num as u64) / 10 {
                guess.guess = guess.guess * 10 + num as u64;
                guess_text.single_mut().sections[1].value.push(input.char);
            }
        }
    }

    if key_input.just_pressed(KeyCode::Return) {
        attempt_guess_writer.send(AttemptGuess);
        // guess_text.single_mut().sections[1].value.clear();
    }

    if key_input.just_pressed(KeyCode::Back) {
        guess.guess /= 10;
        guess_text.single_mut().sections[1].value.pop();
    }
}

fn move_thumb(mut query: Query<&mut Transform, With<ThumbsUp>>) {
    if let Ok(mut thumb) = query.get_single_mut() {
        if thumb.translation.x < 160. {
            thumb.translation.x += 0.2;
        }
    }
}

fn fade_face(mut query: Query<&mut Sprite, With<SadFace>>) {
    if let Ok(mut face) = query.get_single_mut() {
        let alpha = face.color.a();
        if alpha < 1. {
            face.color.set_a(alpha + 0.005);
        }
    }
}

fn guess(
    mut guess: ResMut<Guess>,
    mut attempt_guess_reader: EventReader<AttemptGuess>,
    mut text: Query<&mut Text>,
    mut guesses: ResMut<Guesses>,
    secret: Res<Secret>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if attempt_guess_reader.read().next().is_some()
        && (guess.guess > 0)
        && (guess.guess < u32::MAX as u64)
    {
        if guess.guess as u32 == secret.number {
            for section in &mut text.single_mut().sections {
                section.style.color = Color::NONE;
            }

            commands.spawn(TextBundle {
                text: Text::from_section(
                    "You Won!".to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/Fira_Sans/FiraSans-Bold.ttf"),
                        font_size: 15.,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_translation((0., -70., layers::END_TEXT).into()),
                ..default()
            });

            commands
                .spawn(SpriteBundle {
                    texture: asset_server.load("sprites/thumbs-up.png"),
                    transform: Transform::from_translation((-430., 0., layers::END_SPRITE).into()),
                    ..default()
                })
                .insert(ThumbsUp);

            commands.spawn(AudioBundle {
                source: asset_server.load("audio/sfx/win.wav"),
                ..default()
            });
        }

        if guesses.guesses_left == 1 && !(guess.guess as u32 == secret.number) {
            //really bad way of handling losing
            for section in &mut text.single_mut().sections {
                section.style.color = Color::NONE;
            }

            commands.spawn(TextBundle {
                text: Text::from_section(
                    "You Lost :(".to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/Fira_Sans/FiraSans-Bold.ttf"),
                        font_size: 15.,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1., 1., 1., 0.),
                        ..default()
                    },
                    texture: asset_server.load("sprites/sad-face.png"),
                    transform: Transform::from_translation((0., 0., layers::END_SPRITE).into()),
                    ..default()
                })
                .insert(SadFace);

            commands.spawn(AudioBundle {
                source: asset_server.load("audio/sfx/lose.wav"),
                ..default()
            });
        }

        text.single_mut().sections[3].value = if (guess.guess as u32) < secret.number {
            "Your guess was too low!".to_string()
        } else if (guess.guess as u32) > secret.number {
            "Your guess was too high!".to_string()
        } else {
            text.single_mut().sections[3].value.clone()
        };

        // TODO: REWORK THIS TO STORE THE ACTUAL BLOODY VALUE
        if ((guess.guess as u32) > secret.number)
            && (guess.guess
                < text.single_mut().sections[5]
                    .value
                    .parse()
                    .expect("Should only contain a number"))
        {
            text.single_mut().sections[5].value = guess.guess.to_string();
            println!(
                "parsed num: {}",
                text.single_mut().sections[5]
                    .value
                    .parse::<u64>()
                    .expect("Should only contain a number")
            );
        } else if ((guess.guess as u32) < secret.number)
            && (guess.guess
                > text.single_mut().sections[7]
                    .value
                    .parse()
                    .expect("Should only contain a number"))
        {
            text.single_mut().sections[7].value = guess.guess.to_string();
            println!(
                "parsed num: {}",
                text.single_mut().sections[7]
                    .value
                    .parse::<u64>()
                    .expect("Should only contain a number")
            );
        }

        guesses.guesses_left -= 1;
        text.single_mut().sections[9].value = guesses.guesses_left.to_string();

        text.single_mut().sections[1].value.clear();
        guess.guess = 0;
    }
}

fn last_guess(guesses: Res<Guesses>, mut text: Query<&mut Text>) {
    if guesses.guesses_left == 1 {
        text.single_mut().sections[11].value = "1 guess left!".to_string();
    }
}

fn spawn_text(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let style = TextStyle {
        font: asset_server.load("fonts/MerriweatherSans-Regular.ttf"),
        font_size: 7.,
        color: Color::WHITE,
    };

    commands.spawn(
        TextBundle::from_sections([
            TextSection {
                value: "\
Hi! Welcome to Candy Count!

A random number of marbles between 300
and 1000 has just been dropped into the
jar.
To Play, Type your guess into the text
field and press enter.
Press backspace to delete the last typed digit.

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
                value: "\n".to_string(), // newline between guess and high/low message
                style: style.clone(),
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
                value: "1000".to_string(),
                style: style.clone(),
            },
            TextSection {
                value: "\nThe number is larger than ".to_string(),
                style: style.clone(),
            },
            TextSection {
                value: "300".to_string(),
                style: style.clone(),
            },
            TextSection {
                value: "\nGuesses left: ".to_string(),
                style: style.clone(),
            },
            TextSection {
                value: Guesses::MAX_GUESSES.to_string(),
                style: TextStyle {
                    font_size: 7.,
                    ..default()
                },
            },
            TextSection {
                value: "\n\n".to_string(), // separator
                style: style.clone(),
            },
            TextSection {
                value: "".to_string(), // will show "1 guess left!"
                style: TextStyle {
                    font: asset_server.load("fonts/MerriweatherSans-ExtraBoldItalic.ttf"),
                    font_size: 7.,
                    color: Color::RED,
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
    asset_server.load(format!("sprites/candy{}.png", candy_num))
}
