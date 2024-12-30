use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(SceneRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("ferris3d_v1.0.glb")
    )));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 10., 0.).looking_at(Vec3::ZERO, Vec3::Z),
    ));
}
