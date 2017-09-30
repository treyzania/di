extern crate discord;
extern crate libc;

use discord::Connection as DiscordConn;
use discord::model::Event as DiscordEvent;

mod io;

fn main() {

    match init_connection("[lel]".into()) {
        Ok((mut conn, re)) => loop {

            match conn.recv_event() {
                Ok(e) => match handle_event(e) {
                    Loop => {}, // Do nothing.
                    Exit => break
                },
                Err(ee) => println!("event recv error: {}", ee)
            }

        },
        Err(_) => println!("failed to initialize")
    }

}

fn init_connection(bot_token: String) -> Result<(DiscordConn, discord::model::ReadyEvent), ()> {

    match discord::Discord::from_bot_token(bot_token.as_ref()) {
        Ok(d) => match d.connect() {
            Ok((conn, re)) => Ok((conn, re)),
            Err(ce) => { println!("connect error: {}", ce); Err(()) }
        },
        Err(de) =>{ println!("init error: {}", de); Err(()) }
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
