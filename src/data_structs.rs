use std::collections::BTreeMap;

use rustbreak::{backend::FileBackend, deser::Yaml, Database};
use serde::{Deserialize, Serialize};
use serenity::prelude::TypeMapKey;

pub struct Prefixes;
pub struct MediaChannel;

// TODO
// pub struct ThreadOptions {}

#[derive(Clone, Serialize, Deserialize)]
pub struct ChannelOptions {
    pub mod_talk: bool,
    pub admin_talk: bool,
    pub member_talk: bool,
}

impl TypeMapKey for Prefixes {
    type Value = Database<BTreeMap<u64, String>, FileBackend, Yaml>;
}

impl TypeMapKey for MediaChannel {
    type Value = Database<BTreeMap<u64, ChannelOptions>, FileBackend, Yaml>;
}
