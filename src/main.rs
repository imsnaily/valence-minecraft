use bevy_time::TimePlugin;
use valence::{command::AddCommand, prelude::*, ServerSettings};
use bevy_state::{app::StatesPlugin};
use std::num::NonZero;

pub mod world;
pub mod player;
pub mod plugins;

use plugins::commands::admin::{handle_admin_command, AdminCommand};

#[derive(Debug, Default, Resource)]
pub struct LastTickTime(pub std::time::Duration);

pub fn main() {
    App::new()
        .insert_resource(ServerSettings { tick_rate: NonZero::new(20).unwrap(), ..Default::default() })
        .insert_resource(LastTickTime::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(TimePlugin)
        .add_plugins(StatesPlugin)
        .add_systems(Startup, world::setup)
        .add_systems(
            Update,
            (
                handle_admin_command,
                player::init_clients
            ),
        )
        .add_command::<AdminCommand>()
        .run();
}