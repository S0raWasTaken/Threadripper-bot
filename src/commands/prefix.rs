use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::data_structs::Prefixes;

#[command]
#[owners_only]
async fn logprefixes(ctx: &Context, _msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<Prefixes>().unwrap();
    db.read(|db| {
        println!("{:#?}", db);
    })?;
    Ok(())
}

#[command]
async fn prefix(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg
        .guild_id
        .ok_or("Couldn't get guild_id")?
        .as_u64()
        .to_owned();
    let mut data = ctx.data.write().await;
    let db = data
        .get_mut::<Prefixes>()
        .ok_or("Couldn't get prefixes database")?;

    let prefix = msg.content.split(' ').collect::<Vec<_>>()[1];

    db.write(|db| {
        db.insert(guild_id, String::from(prefix));
    })?;

    msg.reply_ping(&ctx.http, format!("Guild prefix changed to `{}`", prefix))
        .await?;

    match db.save() {
        Ok(_) => (),
        Err(why) => eprintln!("{:#?}", why),
    }

    Ok(())
}
