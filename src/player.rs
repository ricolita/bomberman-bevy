use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    speed: f32
}

#[derive(Component)]
pub struct Frame(f64);


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(player_movemet)
            .add_system(animation);
    }
}

fn spawn_player(mut commands: Commands, imgs: Res<Handle<TextureAtlas>>) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::new(0.1, 0.18)); 
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: imgs.clone(),
        transform: Transform {
            translation: Vec3::new(0., 0., 900.),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Name::new("Player1")).insert(Player {speed: 1. }).insert(Frame(0.5));
}

fn player_movemet(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
    ) {
    let (player, mut transform) = player_query.single_mut();
    
    let mut direct = Vec3::ZERO;

    if keyboard.pressed(KeyCode::A) {
        direct.x += -1.;
    }
    
    if keyboard.pressed(KeyCode::D) {
        direct.x += 1.;
    }

    if keyboard.pressed(KeyCode::W) {
        direct.y += 1.;
    }

    if keyboard.pressed(KeyCode::S) {
        direct.y += -1.;
    }
    
    direct = direct.normalize();
    if direct.is_nan() {
        direct = Vec3::ZERO;
    }
    
    transform.translation +=  direct * player.speed * time.delta_seconds(); 
}

fn animation(
    mut sprite_query: Query<(&Player,&mut TextureAtlasSprite, &mut Frame)>,
    keyboard: Res<Input<KeyCode>>,
) {
    let (player, mut curret_sprite, mut frame) = sprite_query.single_mut();


    if keyboard.pressed(KeyCode::W) {
        frame.0 = if 3.3 <= frame.0 && frame.0 + 0.15 < 6. { frame.0 + 0.15 } else { 3.3 };
    } else if keyboard.pressed(KeyCode::S) {
        frame.0 = if 0.3 <= frame.0 && frame.0 + 0.15 < 3. { frame.0 + 0.15 } else { 0.3 };
    } else if keyboard.pressed(KeyCode::A) {
        frame.0 = if 6.3 <= frame.0 && frame.0 + 0.15 < 9. { frame.0 + 0.15 } else { 6.3 };
    } else if keyboard.pressed(KeyCode::D) {  
        frame.0 = if 9.3 <= frame.0 && frame.0 + 0.15 < 12. { frame.0 + 0.15 } else { 9.3 };
    }

    if keyboard.just_released(KeyCode::A) {
        frame.0 = 6.5;
    }
    if keyboard.just_released(KeyCode::D) {
        frame.0 = 9.5;
    }

    if keyboard.just_released(KeyCode::W) {
        frame.0 = 3.5;
    }

    if keyboard.just_released(KeyCode::S) {
        frame.0 = 0.5;
    } 

    curret_sprite.index = frame.0 as usize;
}
