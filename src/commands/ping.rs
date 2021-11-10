use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let mut new_message = msg.reply_ping(&ctx.http, "Ping?").await?;

    let timestamp = new_message.timestamp.timestamp_millis() - msg.timestamp.timestamp_millis();

    new_message
        .edit(&ctx.http, |m| m.content(format!("Pong! {}ms", timestamp)))
        .await?;
    Ok(())
}
