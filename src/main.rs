mod alla;
mod bis;
mod util;

use std::env;
use std::str;

use regex::Regex;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use serenity::model::user::OnlineStatus;

use alla::Alla;
use bis::Bis;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let spaces: Regex = Regex::new(" +").unwrap();
        let msg_parts: Vec<&str> = spaces.split(msg.content.as_str()).collect();

        let response = match msg_parts[0] {
            "!alla" => Some(Alla::accept_raw(msg_parts[1..].to_vec())),
            "!zam" => Some(Alla::accept_raw(msg_parts[1..].to_vec())),
            "!bis" => Some(Bis::accept_raw(msg_parts[1..].to_vec())),
            _ => None,
        };

        if let Some(to_send) = response {
            if let Err(why) = msg.channel_id.say(&ctx.http, to_send) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        
        println!("{} is connected!", ready.user.name);
        let game = Game::playing("!zam !bis");
        context.set_presence(Some(game), OnlineStatus::Online);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
