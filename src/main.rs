#![allow(unused)]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use particle::ParticlePlugin;

// Constants for text rendering
const TEXT_FONT_SIZE: f32 = 20.0;
const TEXT_COLOR: Color = Color::rgb(1.0, 0.45, 0.09);
const TEXT_PADDING: Val = Val::Px(5.0);

const TYPE_TEXT_PADDING: Val = Val::Px(25.);

const PARTICLE_COLOR_BASIC: Color = Color::rgb(0.62, 0.53, 0.32);
const PARTICLE_COLOR_COMPLEX: Color = Color::rgb(0., 1., 0.4);
const PARTICLE_SIZE: f32 = 5.;

const GRAVITY: f32 = 20.5;

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

        let systems_to_add = (
            update_rendered_text,
            bevy::window::close_on_esc
        );

        app.insert_resource(BasicTimer(Timer::from_seconds(4.0, TimerMode::Repeating)))
            .insert_resource(ParticleCount{count: 0})
            .insert_resource(SelectedType{particle_type: "Basic".to_string()})
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


#[derive(Resource)]
pub struct SelectedType {
    particle_type: String
}


// =======================================================================================================
// ==================================== COMPONENTS =======================================================
// =======================================================================================================

#[derive(Component)]
pub struct Particle {
    id: u16,                // Unique id for the particle
    name: String,           // The name of the particle
    rest_state: bool        // True, if particle is not moving, e.g., has no speed. False otherwise.
}

// TODO: Extending the Particle
// Implement: speed
// 

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
    render_particle_count(&mut commands);
    render_particle_type_selected(&mut commands);

}

fn update_rendered_text(
    particle_count: Res<ParticleCount>,
    mut text_query: Query<&mut Text>,
    input: Res<Input<KeyCode>>,
    mut selected_type: ResMut<SelectedType>
) {

    // System for updating particle count text

    // let mut text = particle_query.single_mut();
    // text.sections[1].value = particle_count.count.to_string();

    for mut text in text_query.iter_mut() {

        if text.sections[0].value.contains("count") {
            update_particle_count(&particle_count, text);
        } else if text.sections[0].value.contains("Type") {
            update_particle_type(text, &input, &mut selected_type);
        }

    }

}


fn update_particle_count(
    particle_count: &Res<ParticleCount>,
    mut text: Mut<Text>
) {

    text.sections[1].value = particle_count.count.to_string();

}


fn update_particle_type(
    mut text: Mut<Text>,
    input: &Res<Input<KeyCode>>,
    mut selected_type: &mut ResMut<SelectedType>
) {

    if input.just_pressed(KeyCode::Key1) {
        selected_type.particle_type = "Basic".to_string();
    } else if input.just_pressed(KeyCode::Key2) {
        selected_type.particle_type = "Complex".to_string();
    }

    text.sections[1].value = selected_type.particle_type.to_string();
}


// =======================================================================================================
// ==================================== HELPER FUNCTIONS =================================================
// =======================================================================================================

pub fn render_particle_type_selected(mut commands: &mut Commands) {

    // Function for rendering the text for displaying which particle type will be spawned

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

pub fn render_particle_count (mut commands: &mut Commands) {
    
    // Standalone function for render text for particle count

    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Type selected: ",
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
            top: TYPE_TEXT_PADDING,
            left: TEXT_PADDING,
            ..default()
        }),
    );

}