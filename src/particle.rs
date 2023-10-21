use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{ParticleCount, Particle, PARTICLE_SIZE, PARTICLE_COLOR_BASIC,
    PARTICLE_COLOR_COMPLEX, BasicTimer, SelectedType, GRAVITY};

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_particle, particle_gravity_system));
    }
}

pub fn spawn_particle(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    // particle_query: Query<&Particle>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut particle_count: ResMut<ParticleCount>,
    selected_type: Res<SelectedType>
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
        // println!("Found cursor at position: {:?}", cursor_position_2d);

    } else {
        // println!("Cursor out of window focus");
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

    let particle_color;
    if selected_type.particle_type.to_string() == "Basic" {
        particle_color = PARTICLE_COLOR_BASIC;
    } else {
        particle_color = PARTICLE_COLOR_COMPLEX;
    }

    commands.spawn((
        
        SpriteBundle {
            sprite: Sprite { custom_size: Some(Vec2::new(PARTICLE_SIZE, PARTICLE_SIZE)),
                color: particle_color, ..default() },
            transform: Transform { translation: cursor_position_3d, ..default() },
            ..default()
        },

        // Particle {
        //     name: "basic".to_string()
        // }
    
    )).insert(Particle{name: selected_type.particle_type.to_string()});

    // Increase particle count
    particle_count.count += 1;

}


fn particle_gravity_system(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut particles: Query<(Entity, &mut Transform), With<Particle>>
) {

    // Adds gravity to particles

    let this_window = window_query.single();        // Get the window
    let floor_border = -this_window.height() / 2.;      // Find the bottom of the window (floor)

    // Loop through our entities
    for (p_entity, mut p_transform) in &mut particles {

        // Get current position
        let mut current_position = &mut p_transform.translation;
        
        // Add a constant GRAVITY modifier, to let the particle sink to the bottom
        if current_position.y - (PARTICLE_SIZE/2.) > floor_border {
            current_position.y -= GRAVITY;
        }

    }

}


fn particle_collision_system(
    mut particles: Query<(Entity, &mut Transform), With<Particle>>
) {

    //    Ideas:
    //
    // 1) Naiive approach - two for loops for checking all particles.
    //
    // 2) Array / List / HashMap for keeping all taken positions at that moment. Updates in gravity system.
    //    Then we can just check against the table for all positions and dont have to loop through all particles
    //    twice. 

    for (p_entity, mut p_transform) in &mut particles {

    } 

}


fn print_particles(time: Res<Time>, mut timer: ResMut<BasicTimer>, query: Query<&Particle>) {

    // Prints out the particles every 2 secods - not used now

    if timer.0.tick(time.delta()).just_finished() {

        let mut particle_count: u8 = 0;

        for particle in &query {
            // println!("Particle: {:?}", particle.name);
            particle_count += 1;
        }

        println!("Number of particles: {}", particle_count)

    }

}