mod commands;
mod data_structs;
mod messages;
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
use tokio::sync::RwLockWriteGuard;

use commands::{moderation::*, ping::*, prefix::*, threads::*};
use data_structs::{MediaChannel, Prefixes};

struct Handler;

#[group]
#[commands(ping, logprefixes, prefix)]
struct General;

#[group]
#[commands(set_media_channel, remove_media_channel)]
struct ThreadManagement;

#[group]
#[commands(clear)]
struct Moderation;

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
    let application_id = var("APPLICATION_ID")?.parse()?;
    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .on_mention(Some(bot_id))
                .ignore_webhooks(true)
                .delimiters(vec![", ", ",", " "])
                .allow_dm(false)
                .with_whitespace(true)
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
        .group(&GENERAL_GROUP)
        .group(&THREADMANAGEMENT_GROUP)
        .group(&MODERATION_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .application_id(application_id)
        .await?;

    {
        let mut data: RwLockWriteGuard<TypeMap> = client.data.write().await;
        data.insert::<Prefixes>(FileDatabase::load_from_path_or_default(
            "./guild_prefixes.yml",
        )?);
        data.insert::<MediaChannel>(FileDatabase::load_from_path_or_default(
            "./media_channels.yml",
        )?);
    }

    if let Err(why) = client.start().await {
        eprintln!("{}", why);
    }
    Ok(())
}
