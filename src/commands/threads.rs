use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::{ChannelType, Message},
};

use crate::{
    data_structs::{ChannelOptions, MediaChannel},
    messages::{CHANNEL_REMOVED_DB, FORBIDDEN_COMMAND_IN_THREAD, NOT_TMC, TMC_SUCCESS},
    multi_handler::parse_command,
};

#[command]
#[aliases("smc", "setmedia", "setmediachannel", "media", "mediachannel")]
#[only_in(guilds)]
#[required_permissions("MANAGE_CHANNELS")]
async fn set_media_channel(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let channel_type = msg
        .channel_id
        .to_channel(&ctx.http)
        .await?
        .guild()
        .ok_or("No channel")?
        .kind;

    if channel_type == ChannelType::PublicThread || channel_type == ChannelType::PrivateThread {
        msg.reply_ping(&ctx.http, FORBIDDEN_COMMAND_IN_THREAD)
            .await?;
        return Ok(());
    }
    let args = String::from("smc ") + args.rest();

    let matches = parse_command("smc")
        .ok_or("Command not found, somehow?")?
        .get_matches_from_safe(args.trim().split(' ').collect::<Vec<_>>());

    match matches {
        Ok(matches) => {
            // TODO: Actually do stuff here
            let channel_options = ChannelOptions {
                admin_talk: matches.is_present("admin_talk"),
                mod_talk: matches.is_present("mod_talk"),
                member_talk: matches.is_present("member_talk"),
            };

            let mut data = ctx.data.write().await;
            let db = data
                .get_mut::<MediaChannel>()
                .ok_or("Couldn't access the TMC database")?;

            let mut tmc_db = db.get_data(true)?;
            let channel_id = msg.channel_id.as_u64().to_owned();

            tmc_db.insert(channel_id, channel_options);
            db.put_data(tmc_db, true)?;
            db.save()?;

            msg.reply_ping(&ctx.http, TMC_SUCCESS).await?;
        }
        Err(why) => {
            msg.reply_ping(&ctx.http, format!("```yml\n{}```", why.message))
                .await?;
        }
    }

    Ok(())
}

#[command]
#[required_permissions("MANAGE_CHANNELS")]
#[only_in(guilds)]
#[aliases("rmc", "rmmedia")]
pub async fn remove_media_channel(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let channel_type = msg
        .channel_id
        .to_channel(&ctx.http)
        .await?
        .guild()
        .ok_or("No channel")?
        .kind;

    if channel_type == ChannelType::PublicThread || channel_type == ChannelType::PrivateThread {
        msg.reply_ping(&ctx.http, FORBIDDEN_COMMAND_IN_THREAD)
            .await?;
        return Ok(());
    }

    let args = String::from("rmc ") + args.rest();

    let matches = parse_command("rmc")
        .ok_or("Command not found, somehow?")?
        .get_matches_from_safe(args.trim().split(' ').collect::<Vec<_>>());

    match matches {
        Ok(_) => {
            let mut data = ctx.data.write().await;
            let db = data
                .get_mut::<MediaChannel>()
                .ok_or("Couldn't access the TMC database")?;

            let mut tmc_db = db.get_data(true)?;
            let channel_id = msg.channel_id.as_u64().to_owned();

            if tmc_db.contains_key(&channel_id) {
                tmc_db.remove_entry(&channel_id);
                db.put_data(tmc_db, true)?;
                db.save()?;

                msg.reply_ping(&ctx.http, CHANNEL_REMOVED_DB).await?;
            } else {
                msg.reply_ping(&ctx.http, NOT_TMC).await?;
            }
        }
        Err(why) => {
            msg.reply_ping(&ctx.http, format!("```yml\n{}```", why.message))
                .await?;
        }
    }

    Ok(())
}
