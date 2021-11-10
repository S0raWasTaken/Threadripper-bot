mod commands;
mod multi_handler;

use anyhow::Result;
use dotenv::dotenv;
use multi_handler::{error_handler, msg_handler};
use serenity::{
    async_trait,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{collections::HashSet, env::var};

use commands::ping::*;

struct Handler;

#[group]
#[commands(ping)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        let result = msg_handler(ctx, message).await;
        error_handler(result);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is in!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let token = var("DISCORD_TOKEN")?;
    let http = Http::new_with_token(&token);

    let mut owners = HashSet::new();
    owners.insert(http.get_current_application_info().await?.owner.id);

    let framework = StandardFramework::new().configure(|c| {
        // TODO: implement dynamic prefixes
        c.owners(owners)
            .ignore_bots(true)
            .prefix(">>")
    }).group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}
