use std::convert::TryFrom;

use clap::{
    App,
    AppSettings::{ColorNever, DisableVersion},
    Arg,
};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::MessageId, Permissions},
    utils::parse_mention,
};

use crate::{
    ensure,
    messages::{
        INVALID_AMMOUNT, INVALID_MENTION, MISSING_PERM_OR_TO, OVERFLOWED_AMMOUNT,
        VALID_ID_FROM_MSG, ZERO_MESSAGES,
    },
    multi_handler::member_perm,
};

// TODO: Clear command to Thread Owner
#[command]
#[aliases("cls", "rm")]
async fn clear(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let member = msg.member(&ctx.http).await?;
    let perm = member_perm(&member, &ctx.cache, Permissions::MANAGE_MESSAGES).await?;
    let args = String::from("clear ") + args.rest();

    let matches = App::new("NAME: Clear")
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
        .get_matches_from_safe(args.trim().split(' ').collect::<Vec<_>>());

    match matches {
        Ok(matches) => {
            ensure! {
                perm,
                {msg.reply_ping(&ctx.http, MISSING_PERM_OR_TO).await?;},
            }
            if let Ok(ammount) = matches
                .value_of("ammount")
                .ok_or("No matches, somehow...")?
                .parse::<u64>()
            {
                ensure! {
                    ammount <= 100 && ammount > 0,
                    {msg.reply_ping(&ctx.http, OVERFLOWED_AMMOUNT).await?;},
                    (if let Some(mid) = matches.value_of("from_message") {
                        mid.parse::<u64>().is_ok()
                    } else {
                        true
                    }),
                    {msg.reply_ping(&ctx.http, VALID_ID_FROM_MSG).await?;}
                }
                let messages = msg
                    .channel_id
                    .messages(&ctx.http, |retriever| {
                        if let Some(message) = matches.value_of("from_message") {
                            if let Ok(message_id) = message.parse::<u64>() {
                                if let Ok(message_id) = MessageId::try_from(message_id) {
                                    if matches.is_present("after") {
                                        retriever.after(message_id).limit(ammount)
                                    } else {
                                        retriever.before(message_id).limit(ammount)
                                    }
                                } else {
                                    retriever.before(msg.id).limit(0)
                                }
                            } else {
                                retriever.before(msg.id).limit(0)
                            }
                        } else {
                            retriever.before(msg.id).limit(ammount)
                        }
                    })
                    .await?;
                ensure! {
                    !messages.is_empty(),
                    {
                        msg.reply_ping(&ctx.http, ZERO_MESSAGES).await?;
                    }
                }
                if let Some(member_mention) = matches.value_of("@mention/ID") {
                    match parse_mention(member_mention).ok_or(INVALID_MENTION) {
                        Ok(mention) => {
                            let filtered = messages
                                .iter()
                                .filter(|mes| mes.author.id == mention)
                                .collect::<Vec<_>>();

                            if let Err(why) =
                                msg.channel_id.delete_messages(&ctx.http, &filtered).await
                            {
                                msg.reply_ping(&ctx.http, why).await?;
                            } else {
                                msg.reply_ping(
                                    &ctx.http,
                                    format!("`{}` messages were purged!", filtered.len()),
                                )
                                .await?;
                            }
                        }
                        Err(why) => {
                            msg.reply_ping(&ctx.http, why).await?;
                        }
                    }
                } else if let Err(why) = msg.channel_id.delete_messages(&ctx.http, &messages).await
                {
                    msg.reply_ping(&ctx.http, why).await?;
                } else {
                    msg.reply_ping(
                        &ctx.http,
                        format!("`{}` messages were purged", messages.len()),
                    )
                    .await?;
                }
            } else {
                msg.reply_ping(&ctx.http, INVALID_AMMOUNT).await?;
            }
        }
        Err(why) => {
            msg.reply_ping(&ctx.http, format!("```yml\n{}```", why.message))
                .await?;
        }
    }

    Ok(())
}
