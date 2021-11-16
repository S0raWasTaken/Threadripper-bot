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
    mod_talk: bool,
    admin_talk: bool,
    member_talk: bool,
}

#[allow(dead_code)]
impl ChannelOptions {
    pub fn new() -> Self {
        Self {
            mod_talk: false,
            admin_talk: false,
            member_talk: false,
        }
    }
}

impl TypeMapKey for Prefixes {
    type Value = Database<BTreeMap<u64, String>, FileBackend, Yaml>;
}

impl TypeMapKey for MediaChannel {
    type Value = Database<BTreeMap<u64, ChannelOptions>, FileBackend, Yaml>;
}
