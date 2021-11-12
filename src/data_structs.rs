use std::collections::BTreeMap;

use rustbreak::{backend::FileBackend, deser::Yaml, Database};
use serenity::prelude::TypeMapKey;

pub struct Prefixes;

impl TypeMapKey for Prefixes {
    type Value = Database<BTreeMap<u64, String>, FileBackend, Yaml>;
}
