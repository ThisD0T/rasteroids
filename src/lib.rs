use bevy::transform;
use bevy::{prelude::*, pbr::GlobalLightMeta, ecs::query::WorldQuery, time::Stopwatch};

use std::sync::Arc;
use std::time::Duration;

use rand;
use rand::{
    Rng,
    thread_rng,
};

use bevy_inspector_egui::{
    Inspectable,
};


#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Playing,
    GameOver,
}

pub const HEALTH_SIZE: f32 = 20.0;

pub const MAX_SPEED: f32 = 18.0;
pub const MAP_SIZE: f32 = 1500.0;
pub const BOUNDARY_BOUNCE_MULT: f32 = 0.15;

pub const ASTEROID_SPEED: f32 = 0.5;

pub struct SpriteList;

#[derive(Component)]
pub struct AsteroidSize{
    size: f32
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
pub struct AsteroidCollider;

#[derive(Component)]
pub struct BulletCollider;

#[derive(Component)]
pub struct PhysFlag;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct DepotSize {
    size: f32,
}

#[derive(Component)]
pub struct Score {
    pub score: u32,
}

#[derive(Component)]
pub struct FuelText;

#[derive(Component, Default)]
pub struct PhysicsVars{
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

#[derive(Component)]
pub struct AsteroidTimer {
    pub timer: Timer
}

#[derive(Component)]
pub struct PlayerStats {
    pub health: u32,
    pub fuel: f32,
}

#[derive(Component)]
pub struct PlayerFuelStopwatch {
    pub stopwatch: Stopwatch
}

#[derive(Bundle, Default)]
pub struct GameObject{
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub physics_vars: PhysicsVars,
}


impl GameObject{
    fn new(
        assets: Res<AssetServer>,
        custom_size: Option<Vec2>,
        transform: Transform,
        image: String,
) -> GameObject{
        let image = assets.load(&image);
        GameObject {
            sprite_bundle: SpriteBundle {

                sprite: Sprite {
                custom_size: custom_size,
                ..Default::default()
                },
                transform: transform,
                texture: image,
                ..Default::default()
            },
            physics_vars: PhysicsVars {
                velocity: Vec3::splat(0.0),
                acceleration: Vec3::splat(0.0),
            },
        }
    }
}

// this is convoluded
pub fn gen_player_sprite(
    assets: Res<AssetServer>,
) -> Vec<Handle<Image>> {
    let player_sprite: Handle<Image> = assets.load("player.png");
    let mut res_list = Vec::new();
    res_list.push(player_sprite);
    return res_list;
}

pub fn apply_phys(
    mut object_phys: Query<(&mut PhysicsVars, &mut Transform), &PhysFlag>,
    //mut object_transform: Query<&mut Transform, &PhysFlag>,
    time: Res<Time>,
) {
    for (mut phys, mut obj_transform) in &mut object_phys {
        
        // messy garbage to work around an error I don't understand at all
        let mut temp_phys_vel = phys.velocity;
        let temp_phys_accel = phys.acceleration;

        temp_phys_vel = temp_phys_vel + temp_phys_accel;
        temp_phys_vel = Vec3::clamp(temp_phys_vel, Vec3::splat(-MAX_SPEED), Vec3::splat(MAX_SPEED));

        phys.velocity = temp_phys_vel;
        obj_transform.translation = obj_transform.translation + phys.velocity;

        phys.acceleration = Vec3::splat(0.0);
    }
}

pub fn check_borders(
    mut phys_obj_query: Query<(&mut PhysicsVars, &mut Transform), Without<AsteroidTimer>>,
    mut player_query: Query<(&mut PhysicsVars, &mut Transform), With<AsteroidTimer>>,
) {
    for (mut phys, mut obj_transform) in &mut phys_obj_query {

        if obj_transform.translation.x > MAP_SIZE/2.0 {
            obj_transform.translation.x = MAP_SIZE/2.0;
            phys.velocity.x *= -1.0;
        } else if obj_transform.translation.x < -MAP_SIZE/2.0 {
            obj_transform.translation.x = -MAP_SIZE/2.0;
            phys.velocity.x *= -1.0;
        }

        if obj_transform.translation.y > MAP_SIZE/2.0 {
            obj_transform.translation.y = MAP_SIZE/2.0;
            phys.velocity.y *= -1.0;
        } else if obj_transform.translation.y < -MAP_SIZE/2.0 {
            obj_transform.translation.y = -MAP_SIZE/2.0;
            phys.velocity.y *= -1.0;
        }
    }

    for (mut phys, mut obj_transform) in &mut player_query {

        if obj_transform.translation.x > MAP_SIZE/2.0 {
            obj_transform.translation.x = MAP_SIZE/2.0;
            phys.velocity.x *= -BOUNDARY_BOUNCE_MULT;
        } else if obj_transform.translation.x < -MAP_SIZE/2.0 {
            obj_transform.translation.x = -MAP_SIZE/2.0;
            phys.velocity.x *= -BOUNDARY_BOUNCE_MULT;
        }

        if obj_transform.translation.y > MAP_SIZE/2.0 {
            obj_transform.translation.y = MAP_SIZE/2.0;
            phys.velocity.y *= -BOUNDARY_BOUNCE_MULT;
        } else if obj_transform.translation.y < -MAP_SIZE/2.0 {
            obj_transform.translation.y = -MAP_SIZE/2.0;
            phys.velocity.y *= -BOUNDARY_BOUNCE_MULT;
        }
    }
}

fn make_timers () -> Timer {
    let timer = Timer::from_seconds(0.0, false);
    return timer
}

pub fn spawn_asteroid(
    mut commands: Commands,
    num_asteroids: u32,
    assets: Res<AssetServer>,
) {

    let mut rng = rand::thread_rng();

    for asteroid in 0..num_asteroids {

    let asteroid_pos = Vec3::new(rng.gen_range(-MAP_SIZE..MAP_SIZE)/2.0, rng.gen_range(-MAP_SIZE..MAP_SIZE)/2.0, 0.0);
    let asteroid_texture: Handle<Image> = assets.load("asteroid.png");
    let asteroid_size_float = rng.gen_range(15.0..30.0);
    let asteroid_size = Vec2::splat(asteroid_size_float);

    commands.spawn_bundle(GameObject{
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(asteroid_size),
                ..Default::default()
            },
            transform: Transform {
                translation: asteroid_pos,
                ..Default::default()
            },
            texture: asteroid_texture,
            ..Default::default()
            },
            ..Default::default()
        })
        .insert(PhysicsVars{
            velocity: Vec3::new(rng.gen_range(-ASTEROID_SPEED..ASTEROID_SPEED), rng.gen_range(-ASTEROID_SPEED..ASTEROID_SPEED), 0.0),
            ..Default::default()
        })
        .insert(PhysFlag)
        .insert(AsteroidCollider)
        .insert(AsteroidSize{size: asteroid_size_float});
    };
    
}

pub fn bullet_collision_check(
    mut commands: Commands,
    mut asteroid_query: Query<(Entity, &Transform, &AsteroidSize), Without<BulletCollider>>,
    mut bullet_query: Query<&mut Transform, With<BulletCollider>>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
    mut score_query: Query<&mut Score, With<PhysFlag>>,
) {
    let mut score = score_query.single_mut();
    let current_score = score.score;

    for (entity, mut asteroid_transform, mut asteroid_size) in asteroid_query.iter_mut() {
        for mut transform in bullet_query.iter_mut() {
            if Vec3::distance(transform.translation, asteroid_transform.translation) < asteroid_size.size{
                commands.entity(entity).despawn();
                score.score += 1;
            }
        }
    }
}

pub fn player_health(
    mut commands: Commands,
    mut asteroid_query: Query<(Entity, &Transform, &AsteroidSize), Without<PlayerStats>>,
    mut player_query: Query<(Entity, &mut Transform, &mut PlayerStats), Without<BulletCollider>>,
    assets: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
) {
    let player_texture: Handle<Image> = assets.load("player.png");

    let (entity, player_transform, mut player_stats) = player_query.single_mut();

    for (asteroid_entity, asteroid_transform, asteroid_size) in asteroid_query.iter_mut() {
        if Vec3::distance(player_transform.translation, asteroid_transform.translation) < asteroid_size.size {
            commands.entity(asteroid_entity).despawn();
            player_stats.health -= 1;
            if player_stats.health < 1 {
                state.set(GameState::GameOver).expect("Failed to change states");
            }
        }
    }
}

pub fn setup_text (
    mut commands: Commands,
    assets: Res<AssetServer>,
) {

    let health_text = commands.spawn().id();
    commands.entity(health_text)
        .insert_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Health: ",
                    TextStyle {
                        font: assets.load("LemonMilk.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: assets.load("LemonMilk.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(HealthText);
    
    let score_text = commands.spawn().id();
    commands.entity(score_text)
        .insert_bundle(

            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font: assets.load("LemonMilk.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: assets.load("LemonMilk.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(ScoreText);

    let game_over_text = commands.spawn().id();
    commands.entity(game_over_text)
        .insert_bundle(
            TextBundle::from_section(

                "GAME OVER",
                TextStyle {
                    font: assets.load("LemonMilk.ttf"),
                    font_size: 100.0,
                    color: Color::RED,
                },
            )
            .with_text_alignment(TextAlignment::CENTER)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(GameOverText);

    let fuel_text = commands.spawn().id();
    commands.entity(fuel_text)
        .insert_bundle(
            
            TextBundle::from_sections([
                TextSection::new(
                    "Fuel: ",
                    TextStyle {
                        font: assets.load("LemonMilk.ttf"),
                        font_size: 30.0,
                        color: Color::ORANGE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: assets.load("LemonMilk.ttf"),
                    font_size: 30.0,
                    color: Color::ORANGE,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(FuelText);
}

pub fn update_health_text(
    mut query: Query<&mut Text, With<HealthText>>,
    mut player_query: Query<&mut PlayerStats, With<PhysFlag>>,
    mut game_over_query: Query<&mut Visibility, With<GameOverText>>,
) {
    let mut text = query.single_mut();
    let player_stats = player_query.single_mut();
    let health = player_stats.health;

    let mut game_over = game_over_query.single_mut();
    game_over.is_visible = false;

    text.sections[1].value = format!("{health}");

    if health < 1 {
        game_over.is_visible = true;
    }
}

pub fn score_text(
    mut query: Query<&mut Text, With<ScoreText>>,
    mut score_query: Query<&mut Score, With<PhysFlag>>,
) {
    let mut text = query.single_mut();
    let mut score_struct = score_query.single_mut();
    let mut score = score_struct.score;

    text.sections[1].value = format!("{score}");
}

pub fn update_fuel_text(
    mut text_query: Query<&mut Text, With<FuelText>>,
    mut fuel_query: Query<&mut PlayerStats, With<PhysFlag>>,
) {
    let mut text = text_query.single_mut();
    let mut fuel_struct = fuel_query.single_mut();
    let mut fuel = fuel_struct.fuel;

    text.sections[1].value = format!("{fuel}");
}

/*
pub fn player_health_text(
    mut commands: Commands,

)
*/

pub fn make_depot(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    let depot_size_float = rng.gen_range(40.0..70.0);
    let depot_size = Vec2::splat(depot_size_float);
    let depot_texture = assets.load("fuel_depot.png");
    let depot_position = Vec3::new(rng.gen_range(-MAP_SIZE/2.0..MAP_SIZE/2.0), rng.gen_range(-MAP_SIZE/2.0..MAP_SIZE/2.0), 0.0);

    let fuel_depot = commands.spawn().id();
    commands.entity(fuel_depot)
        .insert_bundle(SpriteBundle{
            sprite: Sprite {
                custom_size: Some(depot_size),
                ..Default::default()
            },
            transform: Transform {
                translation: depot_position,
                ..Default::default()
            },
            texture: depot_texture,
            ..Default::default()
        })
        .insert(DepotSize {
            size: depot_size_float
        });
}

pub fn fuel_cycle(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut PlayerStats), With<PhysFlag>>,
    mut depot_query: Query<(Entity, &DepotSize, &mut Transform), Without<PhysFlag>>,
    mut time: Res<Time>,
    assets: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
) {
    let mut player_fueled: bool = false;

    let (depot_entity, depot_size, mut depot_transform) = depot_query.single_mut(); 
    let (player_transform, mut player_stats) = player_query.single_mut();
            
            player_stats.fuel -= time.delta_seconds();

            if player_stats.fuel < 0.0 {
                state.set(GameState::GameOver).expect("Failed to change states");
            }

            if Vec3::distance(depot_transform.translation, player_transform.translation) < depot_size.size {
                commands.entity(depot_entity).despawn();
                player_fueled = true;
                player_stats.fuel = 20.0;
            }

    if player_fueled == true {
        make_depot(commands, assets);
    }
}

pub fn game_over(
    mut game_over_query: Query<&mut Visibility, With<GameOverText>>,
) {
    let mut game_over = game_over_query.single_mut();
    game_over.is_visible = true;
}


