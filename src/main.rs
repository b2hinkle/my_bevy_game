use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, world_setup)
        .run();
}

fn world_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let spawn_transform: Transform = Transform::IDENTITY;

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::CLEAR_SUNRISE,
            contact_shadows_enabled: true,
            ..DirectionalLight::default()
        },
        Transform::default().looking_to(Vec3::new(0., -10., 1.), Dir3::Y),
    ));

    commands.spawn((
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("Lvl1Slime.glb"))),
        spawn_transform,
    ));

    commands.spawn((
        Camera3d {
            ..Default::default()
        },
        Transform::from_xyz(0., 0., -15.).looking_at(Vec3::ZERO, Dir3::Y),
    ));
}
