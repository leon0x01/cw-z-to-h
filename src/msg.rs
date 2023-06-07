use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Poll;

// How to we communicate with the contract?

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin_address: String, // validate the ADDr later on
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg { // ExecuteMsg::CreatePoll { "Do you love me"}
   CreatePoll {
     question: String, 
   },
   Vote {
        question: String, // What question we are responding to?
        choice: String, //  what is our answer ? choices yes or no
   }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetPoll {
        question: String,
    }
}

// This is what we return for out GetPoll route

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetPollResponse {
   pub poll: Option<Poll>, // Option means it can either be null (None) or a Pll
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
