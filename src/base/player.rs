use bevy_time::{Time, Timer, TimerMode};
use std::borrow::Cow;
use valence::{
    prelude::*,
    protocol::WritePacket,
    protocol::packets::play::{ParticleS2c, SubtitleS2c, TitleS2c},
    rand::{self, Rng},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnConfetti>()
        .add_systems(Update, (welcome_system, confetti_spawner_system, confetti_system));
    }
}

fn welcome_system(
    mut clients: Query<(Entity, &mut Client, &Username), Added<Client>>,
    mut writer: EventWriter<SpawnConfetti>,
) {
    for (entity, mut client, username) in &mut clients {
        let welcome_text = Text::text(format!("Welcome, {}!", username)).color(Color::LIGHT_PURPLE);
        client.write_packet(&TitleS2c { title_text: Cow::Owned(welcome_text) });

        let subtitle = Text::text("Remember you can join our discord! discord.gg/snaily");
        client.write_packet(&SubtitleS2c { subtitle_text: Cow::Owned(subtitle) });

        writer.send(SpawnConfetti { player: entity });
    }
}

#[derive(Event)]
struct SpawnConfetti {
    player: Entity,
}

#[derive(Component)]
struct ConfettiEmitter {
    player: Entity,
    timer: Timer,
}

fn confetti_spawner_system(
    mut commands: Commands,
    mut ev: EventReader<SpawnConfetti>,
) {
    for SpawnConfetti { player } in ev.read() {
        commands.spawn(ConfettiEmitter {
            player: *player,
            timer: Timer::from_seconds(3.0, TimerMode::Once),
        });
    }
}

fn confetti_system(
    mut commands: Commands,
    mut emitters: Query<(Entity, &mut ConfettiEmitter)>,
    players: Query<&Position>,
    mut clients: Query<&mut Client>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for (ent, mut emitter) in &mut emitters {
        emitter.timer.tick(time.delta());

        if let Ok(pos) = players.get(emitter.player) {
            for mut client in &mut clients {
                let r: f32 = rng.r#gen();
                let g: f32 = rng.r#gen();
                let b: f32 = rng.r#gen();

                client.write_packet(&ParticleS2c {
                    particle: Cow::Owned(Particle::Dust {
                        rgb: Vec3::new(r, g, b),
                        scale: 0.35,
                    }),
                    long_distance: false,
                    position: pos.0 + DVec3::new(0.0, 1.5, 0.0),
                    offset: Vec3::new(0.7, 1.0, 0.7),
                    max_speed: 0.15,
                    count: 20,
                });
            }
        }

        if emitter.timer.finished() {
            commands.entity(ent).despawn();
        }
    }
}