mod commands;
mod data_structs;
mod multi_handler;

use anyhow::Result;
use dotenv::dotenv;
use multi_handler::{error_handler, msg_handler};
use rustbreak::FileDatabase;
use serenity::{
    async_trait,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{collections::HashSet, env::var};

use commands::{ping::*, prefix::*};
use data_structs::Prefixes;

struct Handler;

#[group]
#[commands(ping, logprefixes, prefix)]
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

const DEFAULT_PREFIX: &str = ">>";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let token = var("DISCORD_TOKEN")?;
    let http = Http::new_with_token(&token);

    let mut owners = HashSet::new();
    owners.insert(http.get_current_application_info().await?.owner.id);

    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .ignore_bots(true)
                .dynamic_prefix(|ctx, msg| {
                    Box::pin(async move {
                        let data = ctx.data.read().await;
                        if let Some(db) = data.get::<Prefixes>() {
                            let guild_id = msg.guild_id.unwrap_or_default();
                            if let Ok(data) = db.borrow_data() {
                                let prefix = data
                                    .get(guild_id.as_u64())
                                    .unwrap_or(&String::from(DEFAULT_PREFIX))
                                    .to_owned();
                                Some(prefix)
                            } else {
                                Some(String::from(DEFAULT_PREFIX))
                            }
                        } else {
                            Some(String::from(DEFAULT_PREFIX))
                        }
                    })
                })
        })
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await?;

    {
        let mut data = client.data.write().await;
        data.insert::<Prefixes>(FileDatabase::load_from_path_or_default(
            "./guild_prefixes.yml",
        )?);
    }

    client.start().await?;

    Ok(())
}
