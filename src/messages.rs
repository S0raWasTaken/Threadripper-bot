// messages and stuff, all in one file

// commands/threads.rs
pub const FORBIDDEN_COMMAND_IN_THREAD: &str = "This command cannot be used inside of a thread";
pub const TMC_SUCCESS: &str = "This channel is now a Threadded Media Channel!";
pub const CHANNEL_REMOVED_DB: &str = "Channel removed from the database!";
pub const NOT_TMC: &str = "This channel is not a TMC, no changes made.";

// commands/prefix.rs
pub const NO_CHANGES_PREFIX: &str = "No changes made to the guild prefix";
pub const MISSING_ARGS_PREFIX: &str = "Missing argument: `<prefix>`";
pub fn prefix_changed(prefix: &str) -> String {
    format!("Guild prefix changed to: `{}`\nNote: ||If you messed up, you can always call me by mentioning me||", prefix)
}

// multi_handler.rs
pub const MESSAGE_DELETE_NO_FLAGS: &str =
"Messages in that channel are not allowed
||Tip: If you think you should be able to send messages there, check `>>smc --help`||
||Also, feel free to mute this conversation, since I'll only show warnings here and never important stuff.||";

pub const MESSAGE_DELETE_HAS_FLAGS: &str =
    "Messages in that channel are not allowed with your current permissions.
||Feel free to mute this conversation, I'll only show warnings here and never important stuff.||
a";

// commands/moderation.rs
pub const MISSING_PERM_OR_TO: &str = "Missing `MANAGE_MESSAGES` permission or Thread ownership";
pub const OVERFLOWED_AMMOUNT: &str = "Argument `AMMOUNT` requires a number above 0 and below 101";
pub const VALID_ID_FROM_MSG: &str = "Option `FROM_MESSAGE` requires a valid message ID";
pub const ZERO_MESSAGES: &str = "No messages to purge.";
pub const INVALID_AMMOUNT: &str = "Argument `AMMOUNT` requires a valid number";
pub const INVALID_MENTION: &str = "Option `USER` requires a valid mention or ID";
