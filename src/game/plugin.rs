use crate::app_state::AppState;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game).with_system(create_basic_scene.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(control_player.system())
                .with_system(update_position.system()),
        )
        .add_system_set(SystemSet::on_exit(AppState::Game).with_system(cleanup.system()));
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Component)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

pub struct Scene {
    _width: usize,
    _height: usize,
    pub entities: HashMap<Pos, Vec<Entity>>,

    // TODO: rewrite with 2D array
    pub tile_collisions: HashMap<Pos, bool>,
}

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Char;

#[derive(Component)]
pub struct PlayerControl;

#[derive(Component)]
pub struct AiControl;

#[derive(Component)]
pub struct Collision;

#[derive(Component)]
pub struct Health {
    pub current: u16,
    pub max: u16,
}

#[derive(Component)]
pub struct Modifiers {
    pub offense: u16,
    pub defense: u16,
}

fn create_basic_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // create scene
    let tile_factory = TileFactory::new(&asset_server, &mut materials);

    let mut tile_collisions = HashMap::new();
    let scene_size: usize = 5;
    for i in 0..scene_size {
        for j in 0..scene_size {
            if i == 0 || j == 0 || i == scene_size - 1 || j == scene_size - 1 {
                tile_factory.spawn_wall(&mut commands, Pos { x: i, y: j });
                tile_collisions.insert(Pos { x: i, y: j }, true);
            } else {
                tile_factory.spawn_floor(&mut commands, Pos { x: i, y: j });
                tile_collisions.insert(Pos { x: i, y: j }, false);
            }
        }
    }

    let scene = Scene {
        _width: scene_size,
        _height: scene_size,
        entities: HashMap::new(),
        tile_collisions,
    };

    let player_image = asset_server.load("images/player.png");
    // create player
    let player = commands
        .spawn()
        .insert(Char)
        .insert(PlayerControl)
        .insert(Health {
            current: 10,
            max: 10,
        })
        .insert(Pos { x: 3, y: 3 })
        .insert(Collision)
        .insert_bundle(SpriteBundle {
            texture: player_image,
            ..Default::default()
        })
        .id();

    let mut entities = HashMap::new();
    entities.insert(&Pos { x: 3, y: 3 }, player);
    commands.insert_resource(scene);
}

pub struct TileFactory {
    pub wall_material: Handle<Image>,
    pub floor_material: Handle<Image>,
}

impl TileFactory {
    pub fn new(
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            wall_material: asset_server.load("images/wall.png"),
            floor_material: asset_server.load("images/floor.png"),
        }
    }

    pub fn spawn_wall(&self, commands: &mut Commands, pos: Pos) -> Entity {
        commands
            .spawn_bundle(SpriteBundle {
                texture: self.wall_material.clone(),
                ..Default::default()
            })
            .insert_bundle((Tile, pos, Collision))
            .id()
    }

    pub fn spawn_floor(&self, commands: &mut Commands, pos: Pos) -> Entity {
        commands
            .spawn_bundle(SpriteBundle {
                texture: self.floor_material.clone(),
                ..Default::default()
            })
            .insert_bundle((Tile, pos, Collision))
            .id()
    }
}

fn control_player(
    mut player_query: Query<&mut Pos, With<PlayerControl>>,
    scene: Res<Scene>,
    inputs: Res<Input<KeyCode>>,
) {
    // TODO: check how new single_mut works
    let mut position = player_query.single_mut();
    let mut new_pos = position.clone();

    if inputs.is_changed() {
        match inputs.get_just_released().next() {
            Some(KeyCode::Up | KeyCode::W) => new_pos.y += 1,
            Some(KeyCode::Down | KeyCode::S) => new_pos.y -= 1,
            Some(KeyCode::Left | KeyCode::A) => new_pos.x -= 1,
            Some(KeyCode::Right | KeyCode::D) => new_pos.x += 1,
            _ => {}
        }
    }

    // !!!: Changed detection is triggered by DerefMut
    // meaning even if we rewrite position with equal value Changed will trigger
    if scene.tile_collisions.get(&new_pos) == Some(&false) && *position != new_pos {
        use bevy::log::*;
        info!("player moved");
        *position = new_pos;
    }
}

const TILE_SIZE: usize = 32;

fn update_position(mut query: Query<(&mut Transform, &Pos), Changed<Pos>>, scene: Res<Scene>) {
    // offset shows distance from border to the center of the scene
    let offset_x = (scene._width as f32 - 1.0) * (TILE_SIZE as f32) / 2.0;
    let offset_y = (scene._height as f32 - 1.0) * (TILE_SIZE as f32) / 2.0;

    for (mut transform, grid_position) in query.iter_mut() {
        // in bevy for 2D x=0,y=0 points to the center of the screen
        // we subtract offset so that center of the scene matches center of the screen
        transform.translation.x = (grid_position.x * TILE_SIZE) as f32 - offset_x;
        transform.translation.y = (grid_position.y * TILE_SIZE) as f32 - offset_y;
    }
}

pub fn cleanup() {}
