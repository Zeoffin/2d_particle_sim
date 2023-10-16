use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// Constants for text rendering
const TEXT_FONT_SIZE: f32 = 20.0;
const TEXT_COLOR: Color = Color::rgb(1.0, 0.45, 0.09);
const TEXT_PADDING: Val = Val::Px(5.0);

const PARTICLE_COLOR: Color = Color::rgb(0.62, 0.53, 0.32);

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, InitialPlugin))
    .run()
}


// =======================================================================================================
// ===================================== PLUGINS =========================================================
// =======================================================================================================

// Main plugin for building the app
pub struct InitialPlugin;
impl Plugin for InitialPlugin {

    fn build(&self, app: &mut App) {
        
        println!("Initializing plugins...");

        let systems_to_add = (spawn_particle,
            update_particle_count, bevy::window::close_on_esc);

        app.insert_resource(BasicTimer(Timer::from_seconds(4.0, TimerMode::Repeating)))
            .insert_resource(ParticleCount{count:0})
            .add_systems(Startup, setup)
            .add_systems(Update, systems_to_add);

    }

}


// =======================================================================================================
// ==================================== RESOURCES ========================================================
// =======================================================================================================

#[derive(Resource)]
struct BasicTimer(Timer);

#[derive(Resource)]
pub struct ParticleCount {
    count: u16
}


// =======================================================================================================
// ==================================== COMPONENTS =======================================================
// =======================================================================================================

#[derive(Component)]
pub struct Particle {
    pub name: String
}


// =======================================================================================================
// ===================================== SYSTEMS =========================================================
// =======================================================================================================


pub fn setup(mut commands: Commands) {

    // The starup function. Initiates the camera and
    // renders the text.

    println!("Setup function...");
    
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Text render
    render_particle_count(commands);

}

pub fn spawn_particle(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    // particle_query: Query<&Particle>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut particle_count: ResMut<ParticleCount>
) {

    // Functionality for spawning a particle by left-clicking inside the render screen. 

    // World origin (0,0) is at top left corner
    //
    //   (0, 0)- - - - - >
    //   |
    //   |
    //   |
    //   |
    //   |
    //  \!/

    // Loop to check for left mouse button input
    if !input.just_pressed(MouseButton::Left) {
        return;     // Dont do anything when no input
    }

    // Get cursor position
    let cursor_position_2d: Vec2;
    let this_window = window_query.single();
    if let Some(position) = this_window.cursor_position() {
    
        cursor_position_2d = position;
        println!("Found cursor at position: {:?}", cursor_position_2d);

    } else {
        println!("Cursor out of window focus");
        return;
    }

    // Get window size center. This is needed, because when spawning SpriteBundle, the origin point is
    // the center of the screen and the coordinate system is flipped on y axis
    let window_x_center = this_window.width() / 2.0;
    let window_y_center = this_window.height() / 2.0;

    let cursor_position_3d: Vec3 = Vec3::new(
        cursor_position_2d.x - window_x_center,         // x has the same direction
        -(cursor_position_2d.y - window_y_center),      // y is flipped
        0.0);                                           // In 2d space Z is irrelevant

    // Start spawning particle at the mouse position
    println!("Spawning particle");

    commands.spawn((
        
        SpriteBundle {
            sprite: Sprite { custom_size: Some(Vec2::new(5.0, 5.0)), color: PARTICLE_COLOR, ..default() },
            transform: Transform { translation: cursor_position_3d, ..default() },
            ..default()
        },

        Particle {
            name: "basic".to_string()
        }
    
    ));

    // Increase particle count
    particle_count.count += 1;

}


fn update_particle_count(particle_count: Res<ParticleCount>, mut particle_query: Query<&mut Text>) {

    // System for updating particle count text

    let mut text = particle_query.single_mut();
    text.sections[1].value = particle_count.count.to_string();

}


fn print_particles(time: Res<Time>, mut timer: ResMut<BasicTimer>, query: Query<&Particle>) {

    // Prints out the particles every 2 secods - not used now

    if timer.0.tick(time.delta()).just_finished() {

        let mut particle_count: u8 = 0;

        for particle in &query {
            println!("Particle: {}", particle.name);
            particle_count += 1;
        }

        println!("Number of particles: {}", particle_count)

    }

}


// =======================================================================================================
// ==================================== HELPER FUNCTIONS =================================================
// =======================================================================================================

pub fn render_particle_count (mut commands: Commands) {
    
    // Standalone function for render text for particle count

    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Particle count: ",
                TextStyle {
                    font_size: TEXT_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: TEXT_FONT_SIZE,
                color: TEXT_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: TEXT_PADDING,
            left: TEXT_PADDING,
            ..default()
        }),
    );

}