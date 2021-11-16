use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

// TODO
#[command]
async fn media(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(())
}
