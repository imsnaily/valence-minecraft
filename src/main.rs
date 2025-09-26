use bevy_time::TimePlugin;
use valence::{prelude::*, ServerSettings};
use bevy_state::{app::StatesPlugin};
use std::num::NonZero;

pub mod base;
pub mod world;
pub mod plugins;

use base::{
    player::PlayerPlugin
};

#[derive(Debug, Default, Resource)]
pub struct LastTickTime(pub std::time::Duration);

pub fn init_clients(
    mut clients: Query<
        (
            &mut Position,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    const SPAWN_Y: i32 = 64;

    for (
        mut pos,
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut game_mode,
    ) in &mut clients {
        let layer = layers.single();
        pos.0 = [0.0, f64::from(SPAWN_Y) + 1.0, 0.0].into();
        layer_id.0 = layer;

        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);

        *game_mode = GameMode::Creative;
    }
}

pub fn main() {
    App::new()
        .insert_resource(ServerSettings { tick_rate: NonZero::new(20).unwrap(), ..Default::default() })
        .insert_resource(LastTickTime::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(TimePlugin)
        .add_plugins(StatesPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, world::setup)
        .add_systems(
            Update,
            init_clients
        )
        // .add_command::<AdminCommand>()
        .run();
}