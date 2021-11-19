use anyhow::Result;
use serenity::{client::Context, model::channel::Message};

use crate::data_structs::MediaChannel;

pub async fn msg_handler(ctx: Context, msg: Message) -> Result<()> {
    let data = ctx.data.read().await;
    if let Some(db) = data.get::<MediaChannel>() {
        let tmc_db = db.get_data(true)?;
        let channel_id = msg.channel_id.as_u64();

        // TODO
        if let Some(_channel_options) = tmc_db.get(channel_id) {}
    }

    Ok(())
}

pub fn error_handler(result: Result<()>) {
    match result {
        Ok(_) => (),
        Err(why) => eprintln!("{}", why),
    }
}
