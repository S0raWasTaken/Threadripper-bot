use clap::{
    App,
    AppSettings::{ColorNever, DisableVersion},
    Arg,
};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

/*
>>smc --member_talk --mod_talk --admin_talk
>>smc -Mma
>>smc
>>smc --help
*/

#[command]
#[aliases("smc", "setmedia", "setmediachannel", "media", "mediachannel")]
async fn set_media_channel(ctx: &Context, msg: &Message) -> CommandResult {
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
            msg.reply_ping(&ctx.http, format!("```js\n{:#?}```", matches))
                .await?;
        }
        Err(why) => {
            msg.reply_ping(&ctx.http, format!("```yml\n{}```", why.message))
                .await?;
        }
    }

    Ok(())
}
