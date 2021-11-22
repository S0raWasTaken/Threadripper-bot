use clap::{
    App,
    AppSettings::{ColorNever, DisableVersion},
    Arg,
};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::{ChannelType, Message},
};

use crate::{
    data_structs::{ChannelOptions, MediaChannel},
    messages::{CHANNEL_REMOVED_DB, FORBIDDEN_COMMAND_IN_THREAD, NOT_TMC, TMC_SUCCESS},
};

#[command]
#[aliases("smc", "setmedia", "setmediachannel", "media", "mediachannel")]
#[required_permissions("MANAGE_CHANNELS")]
async fn set_media_channel(ctx: &Context, msg: &Message) -> CommandResult {
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

    let matches = App::new("NAME: Set Media Channel")
        .setting(ColorNever)
        .setting(DisableVersion)
        .about("\nABOUT: Sets the channel to be a Threadded Media Channel (TMC)")
        .arg(
            Arg::with_name("admin_talk")
                .long("admin")
                .short("a")
                .help("Admins are able to speak in TMCs outside of threads"),
        )
        .arg(
            Arg::with_name("mod_talk")
                .long("mod")
                .short("m")
                .help("Make mods be able to speak in TMCs outside of threads (MANAGE_MESSAGES)"),
        )
        .arg(
            Arg::with_name("member_talk")
                .long("member")
                .short("M")
                .help(
                    "Make members be able to speak in TMCs outside of threads\nWARNING: pointless",
                ),
        )
        .get_matches_from_safe(msg.content.trim().split(' ').collect::<Vec<_>>());

    match matches {
        Ok(matches) => {
            // TODO: Actually do stuff here
            let mut channel_options = ChannelOptions::new();

            channel_options.admin_talk = matches.is_present("admin_talk");
            channel_options.mod_talk = matches.is_present("mod_talk");
            channel_options.member_talk = matches.is_present("member_talk");

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
#[aliases("rmc", "rmmedia")]
pub async fn remove_media_channel(ctx: &Context, msg: &Message) -> CommandResult {
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

    let matches = App::new("NAME: Remove Media Channel")
        .setting(ColorNever)
        .setting(DisableVersion)
        .about("\nABOUT: Stops considering the channel a Threadded Media Channel")
        .get_matches_from_safe(msg.content.trim().split(' ').collect::<Vec<_>>());

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
