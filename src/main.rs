use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

#[group]
#[commands(ping)]
struct General;

use std::env;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
fn tarkov(ctx: &mut Context, msg: &Message) -> CommandResult {
    let response = match msg.content.as_ref() {
        "woods" => "https://gamepedia.cursecdn.com/escapefromtarkov_gamepedia/d/d9/Woods_v1.1_lowres.jpg?version=c556f944c9a53d972eb4685911e8f6c1",
        _ => "Not found"
    };

    msg.channel_id.say(&ctx.http, response);

    Ok(())
}