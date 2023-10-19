use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{ParticleCount, Particle, PARTICLE_SIZE, PARTICLE_COLOR, BasicTimer};

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_particle);
    }
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
            sprite: Sprite { custom_size: Some(Vec2::new(PARTICLE_SIZE, PARTICLE_SIZE)),
                color: PARTICLE_COLOR, ..default() },
            transform: Transform { translation: cursor_position_3d, ..default() },
            ..default()
        },

        // Particle {
        //     name: "basic".to_string()
        // }
    
    )).insert(Particle{name: "Basic".to_string()});

    // Increase particle count
    particle_count.count += 1;

}


fn particle_gravity(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut particles: Query<(Entity, &mut Particle)>
) {

    // Adds gravity to particles

    let this_window = window_query.single();
    let floor_border = -this_window.height() / 2.;

    // for (particle_entity, mut particle) in &mut particles {

    //     // TODO: Jopcik popcik kaut kā gravitēt vajag?

    // }

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