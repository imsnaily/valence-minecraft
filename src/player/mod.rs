use valence::prelude::*;

const SPAWN_Y: i32 = 64;

#[allow(clippy::type_complexity)]
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