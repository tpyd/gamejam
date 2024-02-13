use bevy::{input::mouse::{self, MouseButtonInput}, prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}, window::CursorGrabMode};

#[derive(Resource)]
struct MouseTracker {
    last_pos: Vec2,
    hold: bool,
    door_moved: bool,
}

#[derive(Component)]
struct Door;

#[derive(Component)]
struct DoorShutSound;

#[derive(Component)]
struct DoorOpenSound;

enum MoveInstruction {
    Open,
    Close,
    Nothing
}

#[derive(Resource)]
struct DoorMoveInstruction {
    move_instruction: MoveInstruction
}

#[derive(Resource)]
struct Goal {
    rot: Quat,
    reached: bool
}

pub struct SamplePlugin;

impl Plugin for SamplePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
				backends:Some(Backends::DX12),
                ..default()
            })
		}));
    }
}

fn main() {
    App::new()
        .add_plugins(SamplePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_click_system)
        .add_systems(Update, move_door)
        .add_systems(Update, door_opened)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 5.0, 10.0),
            ..default()
        }
        .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("doorframe.png"),
            ..default()
        },
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("door.png"),
            ..default()
        },
        Door
    ));
    commands.spawn((
        AudioBundle {
            source: asset_server.load("doorshut.ogg"),
            ..default()
        },
        DoorShutSound
    ));
    commands.spawn((
        AudioBundle {
            source: asset_server.load("movedoor.ogg"),
            ..default()
        },
        DoorOpenSound
    ));
    commands.insert_resource(MouseTracker {
        last_pos: Vec2 { x: 0.0, y: 0.0 },
        hold: false,
        door_moved: false,
    });
    commands.insert_resource(DoorMoveInstruction {
        move_instruction: MoveInstruction::Nothing
    });
    commands.insert_resource(Goal {
        rot: Quat::from_xyzw(0.0, -0.8134155, 0.0, 0.58168316),
        reached: false
    });
}

// This system prints messages when you press or release the left mouse button:
fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_tracker: ResMut<MouseTracker>,
    mut door_move_instruction: ResMut<DoorMoveInstruction>,
    goal: Res<Goal>
) {
    if goal.reached {
        return
    }

    // Track position when mouse is not held
    if !mouse_tracker.hold {
        for event in cursor_moved_events.read() {
            mouse_tracker.last_pos = event.position;
        }
    }

    // Mouse is held down, check movement direction
    if mouse_button_input.pressed(MouseButton::Left) {
        for event in cursor_moved_events.read() {
            let current_pos = event.position;

            if mouse_tracker.last_pos.y < current_pos.y && !mouse_tracker.door_moved {
                mouse_tracker.door_moved = true;
                door_move_instruction.move_instruction = MoveInstruction::Open;
            }

            if mouse_tracker.last_pos.y > current_pos.y && !mouse_tracker.door_moved {
                mouse_tracker.door_moved = true;
                door_move_instruction.move_instruction = MoveInstruction::Close;
            }
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        mouse_tracker.hold = true;
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        mouse_tracker.hold = false;
        mouse_tracker.door_moved = false;
    }
}

fn move_door(
    mut door_move_instruction: ResMut<DoorMoveInstruction>,
    mut query: Query<&mut Transform, With<Door>>,
    door_open_query: Query<&AudioSink, With<DoorOpenSound>>,
    door_shut_query: Query<&AudioSink, With<DoorShutSound>>,
) {
    let mut door = query.single_mut();

    // let door_open_sound = door_open_query.single();
    // let door_shut_sound = door_shut_query.single();

    match door_move_instruction.move_instruction {
        MoveInstruction::Open => {
            info!("Open door slightly");
            door.rotate_around(Vec3::new(-84.0, 0.0, 0.0), Quat::from_rotation_y(-0.1));
            door_move_instruction.move_instruction = MoveInstruction::Nothing;
            // door_open_sound.play();
            info!("{:?}", door.rotation);
        },
        MoveInstruction::Close => {
            info!("Closing door");
            door.translation = Vec3::ZERO;
            door.rotation = Quat::IDENTITY;
            door_move_instruction.move_instruction = MoveInstruction::Nothing;
            // door_shut_sound.play();
        },
        MoveInstruction::Nothing => {},
    }
}

fn door_opened(
    query: Query<&Transform, With<Door>>,
    mut goal: ResMut<Goal>
) {
    let door = query.single();

    if door.rotation.y <= goal.rot.y {
        goal.reached = true;
        info!("You won!");
    }
}