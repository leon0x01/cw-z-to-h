#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use crate::state::{Config, CONFIG};

/*////////////////////////////////////////////////////////////
 In this file we write logic what contract need to perform
/////////////////////////////////////////////////////////// */
// It is just like api to write put/ post and query the data
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

const CONTRACT_NAME: &str = "crates.io:zero-to-hero";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION"); // 0.1.0 


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
   set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // this will be error, if the user pass invalid address
   let validate_admin_address = deps.api.addr_validate(&msg.admin_address)?;

   let config = Config {
        admin_address: validate_admin_address,
   };

   CONFIG.save(deps.storage, &config)?;

   // Returns the result
   Ok(Response::new().add_attribute("action", "instantiate"))
  
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
