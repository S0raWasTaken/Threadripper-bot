use std::sync::Arc;

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

use crate::data_structs::MediaChannel;

pub async fn msg_handler(ctx: Context, msg: Message) -> CommandResult {
    let data = ctx.data.read().await;
    if let Some(db) = data.get::<MediaChannel>() {
        let tmc_db = db.get_data(true)?;
        let channel_id = msg.channel_id;

        if let Some(channel_options) = tmc_db.get(channel_id.as_u64()) {
            match (
                channel_options.admin_talk,
                channel_options.mod_talk,
                channel_options.member_talk,
            ) {
                (false, false, true) => {
                    if !msg.attachments.is_empty() {
                        channel_id
                            .create_public_thread(&ctx.http, msg.id, |c| {
                                c.name("Test").kind(PublicThread)
                            })
                            .await?;
                    }
                }

                (false, true, _) => (),
                (true, _, _) => (),
                (_, _, _) => (),
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

#[allow(dead_code)]
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
