use std::{env::var, sync::Arc};

use anyhow::Result;
use clap::{
    App,
    AppSettings::{ColorNever, DisableVersion},
    Arg,
};
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
        let has_perm = role.to_role_cached(cache).await.map_or(false, |r| {
            r.has_permission(perm) || r.has_permission(Permissions::ADMINISTRATOR)
        });

        if has_perm {
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

pub fn parse_command(cmd: &str) -> Option<App<'static, 'static>> {
    // Commands
    match cmd {
        "smc" | "setmedia" | "setmediachannel" | "media" | "mediachannel" | "set_media_channel"  => {
            Some(
                App::new("NAME: Set Media Channel")
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
            )

        },

        "rmc" | "rmmedia" | "remove_media_channel" => {
            Some(
                App::new("NAME: Remove Media Channel")
                    .setting(ColorNever)
                    .setting(DisableVersion)
                    .about("\nABOUT: Stops considering the channel a Threadded Media Channel")
            )

        },

        "ping" => {
            Some(
                App::new("NAME: Ping")
                    .setting(ColorNever)
                    .setting(DisableVersion)
                    .about("\nABOUT: Pings discord.com")
            )
        },
        "help" | "man" => {
            Some(
                App::new("NAME: Help")
                    .setting(ColorNever)
                    .setting(DisableVersion)
                    .about("\nABOUT: Asks for help")
                    .arg(
                        Arg::with_name("command")
                            .takes_value(true)
                            .index(1)
                            .help("Specifies a command to get help")
                    )
            )
        },

        "prefix" => {
            Some(
                App::new("NAME: Prefix")
                    .setting(ColorNever)
                    .setting(DisableVersion)
                    .about("\nABOUT: Modifies the guild prefix")
                    .arg(
                        Arg::with_name("prefix")
                            .required(true)
                            .takes_value(true)
                            .index(1)
                    )
            )
        },

        "clear" | "rm" | "cls" => {
            Some(
                App::new("NAME: Clear")
                    .setting(ColorNever)
                    .setting(DisableVersion)
                    .about("\nABOUT: Bulk deletes messages in a channel")
                    .arg(
                        Arg::with_name("ammount")
                            .required(true)
                            .help("The ammount of messages to bulk delete (<100)")
                            .index(1),
                    )
                    .arg(
                        Arg::with_name("@mention/ID")
                            .long("user")
                            .short("u")
                            .takes_value(true)
                            .help("Specify a user to delete messages"),
                    )
                    .arg(
                        Arg::with_name("from_message")
                            .long("message")
                            .short("m")
                            .takes_value(true)
                            .help("Specify a message to start counting by"),
                    )
                    .arg(
                        Arg::with_name("after")
                            .conflicts_with("before")
                            .requires("from_message")
                            .long("after")
                            .short("a")
                            .help("Selects messages after the selected message"),
                    )
                    .arg(
                        Arg::with_name("before")
                            .conflicts_with("after")
                            .requires("from_message")
                            .long("before")
                            .short("b")
                            .help("Selects messages before the selected message"),
                    )

            )
        }

        _ => None,
    }
}
