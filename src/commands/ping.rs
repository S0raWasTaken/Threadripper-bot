use std::process::{Command, Output};

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    /* Content of /usr/local/bin/special_ping.sh:

    #!/bin/bash
    ping -qc1 discord.com 2>&1 | awk -F'/' 'END{ print (/^rtt/? "+ OK "$5" ms":"- FAIL") }'

     */

    let child: Output = Command::new("/usr/local/bin/special_ping.sh").output()?;

    if let Ok(child_stdout) = String::from_utf8(child.stdout) {
        msg.reply_ping(
            &ctx.http,
            format!("```diff\n{}```", child_stdout.replace("\n", "")),
        )
        .await?;
    }

    Ok(())
}
