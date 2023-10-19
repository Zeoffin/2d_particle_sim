#![allow(unused)]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use particle::ParticlePlugin;

// Constants for text rendering
const TEXT_FONT_SIZE: f32 = 20.0;
const TEXT_COLOR: Color = Color::rgb(1.0, 0.45, 0.09);
const TEXT_PADDING: Val = Val::Px(5.0);

const PARTICLE_COLOR: Color = Color::rgb(0.62, 0.53, 0.32);
const PARTICLE_SIZE: f32 = 5.0;

const GRAVITY: f32 = 0.2;

mod particle;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, InitialPlugin, ParticlePlugin))
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

        let systems_to_add = (update_particle_count,
            bevy::window::close_on_esc);

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


fn update_particle_count(particle_count: Res<ParticleCount>, mut particle_query: Query<&mut Text>) {

    // System for updating particle count text

    let mut text = particle_query.single_mut();
    text.sections[1].value = particle_count.count.to_string();

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