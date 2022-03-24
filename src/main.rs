use bevy::{prelude::*, render::{camera::ScalingMode, render_resource::Texture}};

pub const RESOLUTION: f32 = 16. / 9.;


mod player;
mod tilemap;

use player::PlayerPlugin; 


fn main() {
    let compriment = 900.;
    App::new()
        .insert_resource(WindowDescriptor {
            title: "beby bevy beby".to_string(),
            height: compriment,
            width: RESOLUTION * compriment,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.left = -1.0;
    camera.orthographic_projection.right = 1.0;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

fn load_assets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let image = assets.load("player.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image, 
        Vec2::new(19., 23.), 
        3, 
        4, 
        Vec2::new(0.,1.)
    );
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(atlas_handle);    
}


