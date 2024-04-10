use bevy::{prelude::*, render::deterministic::DeterministicRenderingConfig};
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
        fn build(&self, app: &mut App) {
                app.add_systems(Startup, setup_cameras)
                        .add_systems(Update, zoom_control_system);
        }
}

#[derive(Component)]
pub struct MainCamera;

fn setup_cameras(
        mut commands: Commands,
        mut deterministic_rendering_config: ResMut<DeterministicRenderingConfig>,
) {
        deterministic_rendering_config.stable_sort_z_fighting = true;
        commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn zoom_control_system(
        input: Res<ButtonInput<KeyCode>>,
        mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
        let mut projection = camera_query.single_mut();
    
        if input.pressed(KeyCode::Minus) {
            projection.scale += 0.2;
        }
    
        if input.pressed(KeyCode::Equal) {
            projection.scale -= 0.2;
        }
    
        projection.scale = projection.scale.clamp(0.2, 5.);
}
    

// fn setup_ambient_light(
//         mut ambient_light: ResMut<AmbientLight>
// ) {
//         ambient_light.brightness = 50.;
// }

// fn setup_lights(
//         mut commands: Commands
// ) {
//         commands.spawn(
//                 PointLightBundle {
//                         point_light: PointLight {
//                                 intensity: 10_000.,
//                                 ..default()
//                         },
//                         ..default()
//                 }
//         );
// }
