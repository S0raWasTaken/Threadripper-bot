use std::process::{Command, Output};

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::multi_handler::parse_command;

#[command]
async fn ping(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let args = String::from("ping ") + args.rest();

    let matches = parse_command("ping")
        .ok_or("Command not found, somehow?")?
        .get_matches_from_safe(args.trim().split(' ').collect::<Vec<_>>());

    match matches {
        Ok(_) => {
            let child: Output = Command::new("/usr/local/bin/special_ping.sh").output()?;

            if let Ok(child_stdout) = String::from_utf8(child.stdout) {
                msg.reply_ping(
                    &ctx.http,
                    format!("```diff\n{}```", child_stdout.replace("\n", "")),
                )
                .await?;
            }
        }
        Err(why) => {
            msg.reply_ping(&ctx.http, why).await?;
        }
    }

    Ok(())
}
