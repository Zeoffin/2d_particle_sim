use std::iter;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{ParticleCount, Particle, PARTICLE_SIZE, PARTICLE_COLOR_BASIC,
    PARTICLE_COLOR_COMPLEX, BasicTimer, SelectedType, GRAVITY};

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_particle,
            particle_gravity_system,
            particle_collision_system)
        );
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

    println!("Window h: {}   w: {}", this_window.height(), this_window.width());

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
    
    )).insert(Particle{
        id: particle_count.count,
        name: selected_type.particle_type.to_string(),
        rest_state: false
    });

    // Increase particle count
    particle_count.count += 1;

}


fn particle_gravity_system(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut particles: Query<(&mut Particle, &mut Transform)>
    // mut particles: Query<(Entity, &mut Transform), With<Particle>>   // With/ Queries nicely explained -> https://bevy-cheatbook.github.io/programming/queries.html
) {

    // Adds gravity to particles
    let this_window = window_query.single();                               // Get the window
    let floor_border = (-this_window.height() / 2.) + (PARTICLE_SIZE/2.);      // Find the bottom of the window (floor)

    // Loop through our entities
    for (mut particle, mut p_transform) in &mut particles {

        if !particle.rest_state {

            // println!("Particle {} rest state: {}", particle.id, particle.rest_state);

            // Get current position
            let mut current_position = &mut p_transform.translation;

            let next_position = current_position.y - PARTICLE_SIZE;     // This way we have a grid system

            // Add a constant GRAVITY modifier, to let the particle sink to the bottom
            if next_position > floor_border {
                current_position.y -= GRAVITY;

            // If the next step is over the limits, set the position to be the floor border
            } else if next_position <= floor_border {
                current_position.y = floor_border;
                particle.rest_state = true;
            }

        }

    }

}


fn particle_collision_system(
    mut particles_query: Query<(&mut Particle, &mut Transform)>,            // Particle for which collision implemented
    window_query: Query<&Window, With<PrimaryWindow>>                       // Window
) {

    //    Ideas:
    //
    // 1) Naiive approach - two for loops for checking all particles.
    //
    // 2) Array / List / HashMap for keeping all taken positions at that moment. Updates in gravity system.
    //    Then we can just check against the table for all positions and dont have to loop through all particles
    //    twice. 

    let mut particles = particles_query.iter_combinations_mut();


    // 

    // Loop through each possible pair combination of particles. Ineffective but 
    while let Some(
        [(mut particle_1, mut p_transform_1),
        (mut particle_2, mut p_transform_2)]
    ) = particles.fetch_next() {

        // Get translation values
        let mut particle_1_position = &mut p_transform_1.translation;
        let mut particle_2_position = &mut p_transform_2.translation;

        // Here, we check if particle_1 is higher up than particle_2, and is not in rest state
        if particle_1_position.y > particle_2_position.y && !particle_1.rest_state {

            check_for_down_particle(
                particle_1, particle_1_position,
                particle_2, particle_2_position);

        // Same as in previous statement, but check for other pairing possibility
        } else if particle_2_position.y > particle_1_position.y && !particle_2.rest_state {

            check_for_down_particle(
                particle_2, particle_2_position,
                particle_1, particle_1_position)
                
        }

    }

}


fn check_for_down_particle(
    mut particle_1: Mut<Particle>,
    mut particle_1_position: &mut Vec3,
    mut particle_2: Mut<Particle>,
    mut particle_2_position: &mut Vec3,
) {

    // Check if difference in y position matches with its dimensions
    if particle_1_position.y - particle_2_position.y <= PARTICLE_SIZE {

        // Get the diffence on x axis
        let x_difference = particle_1_position.x - particle_2_position.x;
        let x_absolute_difference = x_difference.abs();
        let x_allowed_limit: f32 = 0.;


        // Check if less than half of a particle is on top of another one
        if x_absolute_difference >= x_allowed_limit && x_absolute_difference < PARTICLE_SIZE {

            // Make the particle fall down
            // If difference is a negative number, we move particle to the left. Else right.
            if x_difference <= 0. {

                particle_1_position.x -= PARTICLE_SIZE + x_difference;  // To the left side
            
            } else {
            
                particle_1_position.x += PARTICLE_SIZE - x_difference;  // To the right side
            
            }

            // Unset rest state
            particle_1.rest_state = false;

        // Check if more than half of the particle is on another one
        } else if x_difference.abs() < x_allowed_limit {

            // If thats the case, then the particle has landed on top of another one. Set rest state.
            particle_1_position.y = particle_2_position.y + PARTICLE_SIZE;
            particle_1.rest_state = true;

        }

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