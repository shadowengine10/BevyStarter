use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap}, 
    prelude::*
};
use std::f32::consts::*;

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_light_direction)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 1.0,10.5).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 250.0,
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .build(),
    ));
    // Props n Stuff
    commands.spawn((
        SceneRoot(asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("models/room/room.glb"),
            )),
        Transform::from_xyz(0.0, 0.0, 1.0),
        )).observe(|mut trigger: Trigger<Pointer<Click>>| {
            println!("I was just clicked!");
            // Get the underlying pointer event data
            let _click_event: &Pointer<Click> = trigger.event();
            // Stop the event from bubbling up the entity hierarchy
            trigger.propagate(false);
        }
    );

    commands.spawn((
        SceneRoot(asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("models/rock_baked/rock_baked.glb"),
            )),
        Transform::from_xyz(0.0, 0.0, 1.0),
        )).observe(|mut trigger: Trigger<Pointer<Click>>| {
            println!("fan");
            // Get the underlying pointer event data
            let _click_event: &Pointer<Click> = trigger.event();
            // Stop the event from bubbling up the entity hierarchy
            trigger.propagate(false);
        }
    );
    
    commands.spawn((
        SceneRoot(asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("models/errorwall/errorwall.glb"),
            )),
        Transform::from_xyz(4.0, 0.0, 1.0).with_rotation(Quat::from_rotation_x(-PI / 1.0)),
        ));
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_secs() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}


