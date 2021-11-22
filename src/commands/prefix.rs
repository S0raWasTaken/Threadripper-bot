use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::{
    data_structs::Prefixes,
    messages::{prefix_changed, MISSING_ARGS_PREFIX, NO_CHANGES_PREFIX},
    DEFAULT_PREFIX,
};

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
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn prefix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg
        .guild_id
        .ok_or("Couldn't get guild_id")?
        .as_u64()
        .to_owned();
    let mut data = ctx.data.write().await;
    let db = data
        .get_mut::<Prefixes>()
        .ok_or("Couldn't get prefixes database")?;

    args.trimmed();

    if let Some(prefix) = args.current() {
        let mut prefix_db = db.get_data(true)?;
        let old_prefix = prefix_db.get(&guild_id);
        if prefix == old_prefix.unwrap_or(&String::from(DEFAULT_PREFIX)) {
            msg.reply_ping(&ctx.http, NO_CHANGES_PREFIX).await?;
        } else {
            prefix_db.insert(guild_id, String::from(prefix));
            db.put_data(prefix_db, true)?;
            db.save()?;

            msg.reply_ping(&ctx.http, prefix_changed(prefix)).await?;
        }
    } else {
        msg.reply_ping(&ctx.http, MISSING_ARGS_PREFIX).await?;
    }
    Ok(())
}
