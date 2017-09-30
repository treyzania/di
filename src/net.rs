
use std::env;

use std::fs;
use std::path::PathBuf;

use discord;
use discord::Discord;
use discord::Connection as DiscordConn;
use discord::model::Event as DiscordEvent;
use discord::model::PossibleServer;

use interface;
use interface::DiState;

pub fn event_loop(di: &mut DiState, mut conn: DiscordConn, re: discord::model::ReadyEvent) {

    use self::EventHandleResult::*;

    for s in re.servers {
        match s {
            PossibleServer::Online(ref serv) => for c in serv.channels {
                match di.add_channel(&serv, &c) {
                    Ok(_) => {},
                    Err(m) => println!("when adding CID {}, error {}", c.id, m)
                }
            },
            PossibleServer::Offline(off) => println!("server ID {} is offline, ignoring", off.0)
        }
    }

    loop {

        match conn.recv_event() {
            Ok(e) => match handle_event(di, e) {
                Loop => {}, // Do nothing.
                Exit => break
            },
            Err(ee) => println!("event recv error: {}", ee)
        }

    }

}

pub struct InitError(pub &'static str);

pub fn init_discord() -> Result<Discord, InitError> {

    // I am not proud of this.
    match env::var("DI_AUTH_MODE") {
        Ok(v) => match v.as_ref() {
            "bot" => match env::var("DI_BOT_TOKEN") {
                Ok(token) => postprocess_discord_init(Discord::from_bot_token(token.as_ref())),
                Err(_) => Err(InitError("failed to get bot token"))
            },
            "user" => match (env::var("DI_EMAIL"), env::var("DI_PASSWORD")) {
                (Ok(name), Ok(pass)) => postprocess_discord_init(Discord::new(name.as_ref(), pass.as_ref())), // FIXME Make this better.
                _ => Err(InitError("failed to get user credentials"))
            },
            _ => Err(InitError("invalid auth mode"))
        },
        Err(_) => Err(InitError("failed to get auth mode, is it set?"))
    }

}

fn postprocess_discord_init(rd: discord::Result<Discord>) -> Result<Discord, InitError> {
    match rd {
        Ok(d) => Ok(d),
        Err(_) => Err(InitError("discord failed to auth"))
    }
}

enum EventHandleResult {
    Loop,
    Exit
}

fn handle_event(di: &mut DiState, event: DiscordEvent) -> EventHandleResult {

    use self::EventHandleResult::*;

    match event {
        DiscordEvent::MessageCreate(m) => {

            Loop
        }
        _ => Loop
    }

}
