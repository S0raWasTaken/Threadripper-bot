use std::{env::var, sync::Arc};

use anyhow::Result;
use serenity::{
    client::{Cache, Context},
    framework::standard::CommandResult,
    model::{
        channel::{ChannelType::PublicThread, Message},
        guild::Member,
        Permissions,
    },
};

use crate::{
    data_structs::{MediaChannel, Prefixes},
    messages::{MESSAGE_DELETE_HAS_FLAGS, MESSAGE_DELETE_NO_FLAGS},
    DEFAULT_PREFIX,
};

pub async fn msg_handler(ctx: Context, msg: Message) -> CommandResult {
    if msg.author.bot {
        return Ok(());
    }

    let data = ctx.data.read().await;

    if let Some(prefixes_db) = data.get::<Prefixes>() {
        let prefixes = prefixes_db.get_data(true)?;
        let default_prefix = &String::from(DEFAULT_PREFIX);
        let prefix = prefixes
            .get(msg.guild_id.unwrap_or_default().as_u64())
            .unwrap_or(default_prefix);
        if msg.content.starts_with(prefix)
            || msg
                .content
                .starts_with(format!("<!@{}>", var("APPLICATION_ID")?).as_str())
        {
            return Ok(());
        }
    };

    if let Some(db) = data.get::<MediaChannel>() {
        let tmc_db = db.get_data(true)?;
        let channel_id = msg.channel_id;

        if let Some(channel_options) = tmc_db.get(channel_id.as_u64()) {
            if !msg.attachments.is_empty() {
                let author_name = msg.author.name;
                channel_id
                    .create_public_thread(&ctx.http, msg.id, |c| {
                        c.name(format!("Media from {}", author_name))
                            .kind(PublicThread)
                            .auto_archive_duration(60)
                    })
                    .await?;
            } else {
                match (
                    channel_options.admin_talk,
                    channel_options.mod_talk,
                    channel_options.member_talk,
                ) {
                    (_, _, true) => (/* Do absolutely nothing */),
                    (_, true, false) => {
                        // Check MANAGE_MESSAGES permission
                        let member = msg.member(&ctx.http).await?;
                        let has_perm =
                            member_perm(&member, &ctx.cache, Permissions::MANAGE_MESSAGES).await?;

                        if !has_perm {
                            msg.author
                                .direct_message(&ctx.http, |dm| {
                                    dm.content(MESSAGE_DELETE_HAS_FLAGS)
                                })
                                .await?;
                            msg.delete(&ctx.http).await?;
                        }
                    }
                    (true, false, false) => {
                        // Check ADMINISTRATOR permission
                        let member = msg.member(&ctx.http).await?;
                        let has_perm =
                            member_perm(&member, &ctx.cache, Permissions::ADMINISTRATOR).await?;

                        if !has_perm {
                            msg.author
                                .direct_message(&ctx.http, |dm| {
                                    dm.content(MESSAGE_DELETE_HAS_FLAGS)
                                })
                                .await?;
                            msg.delete(&ctx.http).await?;
                        }
                    }
                    (_, _, _) => {
                        /* Delete message anyway */
                        msg.author
                            .direct_message(&ctx.http, |dm| dm.content(MESSAGE_DELETE_NO_FLAGS))
                            .await?;
                        msg.delete(&ctx.http).await?;
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn error_handler(result: CommandResult) {
    match result {
        Ok(_) => (),
        Err(why) => eprintln!("{}", why),
    }
}

pub async fn member_perm(member: &Member, cache: &Arc<Cache>, perm: Permissions) -> Result<bool> {
    for role in &member.roles {
        if role
            .to_role_cached(cache)
            .await
            .map_or(false, |r| r.has_permission(perm))
        {
            return Ok(true);
        }
    }

    Ok(false)
}

#[macro_export]
macro_rules! ensure {
    ($($x:expr, $y:tt),* $(,)?) => {
        $(
            if !$x {
                $y();
                return Ok(());
            }
        )*
    };
}
