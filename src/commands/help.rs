use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::{
    messages::{COMMAND_LIST, COMMAND_NOT_FOUND, WEIRD_SUCCESS},
    multi_handler::parse_command,
};

#[command]
#[only_in(guilds)]
#[aliases("man")]
async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let args = String::from("help ") + args.rest();

    let matches = parse_command("help")
        .ok_or("Command not found, somehow?")?
        .get_matches_from_safe(args.trim().split(' ').collect::<Vec<_>>());

    match matches {
        Ok(matches) => {
            if let Some(command) = matches.value_of("command") {
                let why = {
                    if let Some(app) = parse_command(command) {
                        if let Err(why) = app.get_matches_from_safe(vec![command, "--help"]) {
                            format!("```yml\n{}```", why)
                        } else {
                            String::from(WEIRD_SUCCESS)
                        }
                    } else {
                        String::from(COMMAND_NOT_FOUND)
                    }
                };
                msg.reply_ping(&ctx.http, why).await?;
            } else {
                msg.reply_ping(&ctx.http, COMMAND_LIST).await?;
            }
        }
        Err(why) => {
            msg.reply_ping(&ctx.http, format!("```yml\n{}```", why))
                .await?;
        }
    }

    Ok(())
}
