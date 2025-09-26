use valence::{command, command_macros, prelude::*};
use command::handler::CommandResultEvent;
use command_macros::Command;

#[derive(Command, Debug, Clone)]
#[paths("admin")]
pub enum AdminCommand {
    #[paths = "tps"]
    TPS,
}

fn format_tps(tps: f64) -> String {
let color = if tps > 18.0 {
        "a"
    } else if tps > 16.0 {
        "e"
    } else {
        "c"
    };

    let prefix = if tps > 20.0 { "*" } else { "" };
    let formatted_tps = f64::min(tps, 20.0);

    format!("§{}{}{:.2}", color, prefix, formatted_tps)
}

pub fn handle_admin_command(
    mut entities: Query<(Entity, &Position, &mut Client)>,
    mut events: EventReader<CommandResultEvent<AdminCommand>>,
    server: Res<Server>
) {
    for event in events.read() {
        let caller = event.executor;
        let mut player_client = entities.get_mut(caller).unwrap().2;
        let message = format!("Server Executor: {}", &caller);
        println!("{}", message); // menuda puta gilipollez

        match &event.result {
            AdminCommand::TPS => {
                let tps = server.tick_rate().get() as f64;
                let message = Text::text("Server TPS: ").color(Color::GOLD) + Text::text(format_tps(tps));
                player_client.send_chat_message(message);
            }
        }
    }
}