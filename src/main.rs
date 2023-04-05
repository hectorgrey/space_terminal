use bevy::prelude::*;

#[derive(Component)]
struct Mass(f32);

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Accelleration(Vec3);

fn main() {
    App::new()
        .add_system(hello_world)
        .run()
}

fn hello_world() {
    println!("Hello World!");
}

fn accellerate(mut query: Query<(&Mass, &mut Accelleration)>, force: Vec3) {
    for (mass, mut accel) in query.iter_mut() {
        accel.0 += force / mass.0;
    }
}