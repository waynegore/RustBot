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
#[commands(tarkov)]
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
    } else {
        println!("Started up!");
    }
}

#[command]
fn tarkov(ctx: &mut Context, msg: &Message) -> CommandResult {
    let split_msg = msg.content.split(' ').collect::<Vec<&str>>();

    let mut response;

    if split_msg.len() > 1 {
        let location = split_msg[1];
        response = match location.as_ref() {
            "factory" => "https://gamepedia.cursecdn.com/escapefromtarkov_gamepedia/8/83/Factory_0.8.png?version=91f04c0c3f62388c86e3fbebdd0abcdf",
            "customs" => "https://gamepedia.cursecdn.com/escapefromtarkov_gamepedia/c/c8/Customs_Nuxx_20190106_1.2.png?version=a3b44edf49616eaad2736c6523c977b0",
            "woods" => "https://gamepedia.cursecdn.com/escapefromtarkov_gamepedia/d/d9/Woods_v1.1_lowres.jpg?version=c556f944c9a53d972eb4685911e8f6c1",
            "shoreline" => "https://gamepedia.cursecdn.com/escapefromtarkov_gamepedia/e/e1/Actual_caches_37_map_shoreline.jpg?version=e589dbcf739214d11dde16957b82c817",
            "interchange" => "https://gamepedia.cursecdn.com/escapefromtarkov_gamepedia/0/06/InterchangeMapLorathor.jpg?version=2880f134180975c58d5c4dcef4327ef4",
            "lab" => "https://gamepedia.cursecdn.com/escapefromtarkov_gamepedia/0/0b/TheLabMapFull.png?version=8743e690fbd315e114f51540347eca29",
            "reserve" => "https://gamepedia.cursecdn.com/escapefromtarkov_gamepedia/c/c0/ReserveMap3d.jpg?version=69dfeba044fd40717fdd61afa6bc82d2",
            _ => "Not found"
        };
        msg.reply(ctx, response);
    } else {
        msg.reply(ctx, "Usage: ~tarkov map-name");
    }


    Ok(())
}
