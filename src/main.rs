use bevy::{prelude::*, render::RenderPlugin};

#[derive(Component)]
struct SpinAttack {
    damage: u16,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, (HelloWorld, HelloWorld2))
        .add_systems(Update, add_SpinAttack)
        .run();
}

fn add_SpinAttack(mut commands: Commands) {
    commands.spawn((SpinAttack { damage: 5 }));
}

fn HelloWorld(query: Query<&SpinAttack>) {
    let df = 5;
    for sa in &query {
        println!("SpinAttack damage = {}", sa.damage);
    }
}
fn HelloWorld2() {
    println!("2");
}
