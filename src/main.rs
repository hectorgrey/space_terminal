use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Description(String);

#[derive(Component)]
struct Location;

fn main() {
    App::new()
        .add_startup_system(spawn_starting_location)
        .add_system(hello_world)
        .run()
}

fn hello_world(query: Query<(&Name, &Description, With<Location>)>) {
    for (name, desc, _) in query.iter() {
        println!("Hello, {}", name.0);
        println!("{}", desc.0);
    }
}

fn spawn_starting_location(mut commands: Commands) {
    commands.spawn((
        Location,
        Name("Earth".into()),
        Description("The bright blue marble is visible through the windows.".into())
    ));
    commands.spawn((
        Location,
        Name("The Sun".into()),
        Description("A bright ball of incandescent gas is visible through the windows.\nSuch a view could potentially blind any who saw it".into())
    ));
}