extern crate discord;
extern crate libc;

use std::env;

use discord::Discord;
use discord::Connection as DiscordConn;
use discord::model::Event as DiscordEvent;

mod io;

fn main() {

    match init_discord() {
        Ok(d) => match d.connect() {
            Ok((conn, re)) => event_loop(conn, re),
            Err(_) => println!("failed to connect")
        },
        Err(InitError(r)) => println!("failed to initialize: {}", r)
    }

}

fn event_loop(mut conn: DiscordConn, re: discord::model::ReadyEvent) {

    loop {

        match conn.recv_event() {
            Ok(e) => match handle_event(e) {
                Loop => {}, // Do nothing.
                Exit => break
            },
            Err(ee) => println!("event recv error: {}", ee)
        }

    }

}

struct InitError(&'static str);

fn init_discord() -> Result<Discord, InitError> {

    // I am not proud of this.
    match env::var("DI_AUTH_MODE") {
        Ok(v) => match v.as_ref() {
            "bot" => match env::var("DI_BOT_TOKEN") {
                Ok(token) => postprocess_discord_init(Discord::from_bot_token(token.as_ref())),
                Err(_) => Err(InitError("failed to get bot token"))
            },
            "user" => match (env::var("DI_EMAIL"), env::var("DI_PASSWORD")) {
                (Ok(name), Ok(pass)) => postprocess_discord_init(Discord::new(name.as_ref(), pass.as_ref())),
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

fn handle_event(event: DiscordEvent) -> EventHandleResult {

    use EventHandleResult::*;

    match event {
        DiscordEvent::MessageCreate(m) => {
            println!("message: {}", m.content);
            Loop
        }
        _ => Loop
    }

}
