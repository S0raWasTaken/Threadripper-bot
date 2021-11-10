use anyhow::Result;
use serenity::{client::Context, model::channel::Message};

// Todo: message handler for non commands
pub async fn msg_handler(_ctx: Context, _message: Message) -> Result<()> {
    Ok(())
}

pub fn error_handler(result: Result<()>) {
    match result {
        Ok(_) => (),
        Err(why) => eprintln!("{}", why),
    }
}
