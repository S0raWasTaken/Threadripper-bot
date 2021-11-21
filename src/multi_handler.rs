use anyhow::Result;
use serenity::{
    client::Context,
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
                        let guild = msg.guild(&ctx.cache).await;

                        // TODO: thread name, saving threads in db and removing them when needed
                        if let Some(guild) = guild {
                            if let Some(guild_channel) = guild.channels.get(&channel_id) {
                                let _thread = guild_channel
                                    .create_public_thread(&ctx.http, &msg.id, |c| {
                                        c.name("test").kind(PublicThread)
                                    })
                                    .await?;
                            }
                        }
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
pub async fn member_perm(member: &Member, ctx: &Context, perm: Permissions) -> Result<bool> {
    for role in &member.roles {
        if role
            .to_role_cached(&ctx.cache)
            .await
            .map_or(false, |r| r.has_permission(perm))
        {
            return Ok(true);
        }
    }

    Ok(false)
}
