use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_address : Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll{
    pub question: String,
    pub yes_vote: u64,
    pub no_vote: u64,
}

pub const CONFIG: Item<Config> = Item::new("config"); // This is stored on chain

// String => poll value
pub const POLLS: Map<String, Poll> = Map::new("polls"); // stores the poll variable with string index
