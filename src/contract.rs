#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary,};
use cw2::set_contract_version;
//use cw_multi_test::Contract;
use crate::state::{Config, CONFIG, POLLS, Poll};

/*////////////////////////////////////////////////////////////
 In this file we write logic what contract need to perform
/////////////////////////////////////////////////////////// */
// It is just like api to write put/ post and query the data
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, GetPollResponse};

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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    match msg {
        ExecuteMsg::CreatePoll { question } => execute_create_poll(deps, env, info, question), 
        ExecuteMsg::Vote { question, choice } => execute_vote(deps, env, info, question, choice),
    }  
    
}

fn execute_create_poll(
   deps: DepsMut,
   _env: Env,
   _info: MessageInfo,
   question: String,
) -> Result<Response, ContractError> {

    if POLLS.has(deps.storage, question.clone()){
        return Err(ContractError::CustomError { val: "Key has already taken".to_string() });
    }

    let poll = Poll {
        question: question.clone(),
        yes_vote: 0,
        no_vote: 0, 
    };
    POLLS.save(deps.storage, question, &poll)?;

    Ok(Response::new()
    .add_attribute("action", "create_poll"))
   }

fn execute_vote(
   deps: DepsMut,
   _env: Env,
   _info: MessageInfo,
   question: String,
   choice: String,
) -> Result<Response, ContractError> {
    // if there is no key question then we are throwing error

    if !POLLS.has(deps.storage, question.clone()){
        return Err(ContractError::CustomError { val: "Poll doesnot consist this question".to_string() });
    }

    let mut poll = POLLS.load(deps.storage, question.clone())?;

    if choice != "yes" && choice != "no" {
        return Err(ContractError::CustomError { val: "unrecognized choice".to_string() });
   } else {

    // If its yes add to the yes vote
    // IF its no add to the no vote
    if choice == "yes" {
        poll.yes_vote +=1; 
    } else {
        poll.no_vote += 1; 
    }
    
    //save the updated poll to the chain
    POLLS.save(deps.storage, question, &poll)?;
    Ok(Response::new()
    .add_attribute("action", "vote"))
   }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPoll { question }  => query_get_poll(deps, env, question)
    }
}

fn query_get_poll(deps: Deps, env: Env, question: String) -> StdResult<Binary> {
    let poll = POLLS.may_load(deps.storage, question)?;
    to_binary(&GetPollResponse { poll })
}
#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use crate::{msg::{InstantiateMsg, ExecuteMsg, QueryMsg, GetPollResponse}, contract::query, };

    

    use super::{instantiate, execute};

    use cosmwasm_std::{attr, from_binary};

    use crate::state::Poll;


    #[test]

    // Before testing we need to get the exact value of real one so 
    // To fake the real value we use mocking of necessary parameter
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string() // String, String::from("addr1")
        };
        //you can only use "?" if function return StdResult<>
        let resp = instantiate(deps.as_mut(), env, info, msg).unwrap();

        // now checking whether it is correct or not
        assert_eq!(resp.attributes, vec![
            attr("action", "instantiate")
        ]);
    }


    #[test]

    fn test_create_poll() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string() // String, String::from("addr1")
        };
        // Before execute you need to instantiate the contract 
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreatePoll { question: "Do you love Spark IBC".to_string()
     };

        let resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        assert_eq!(resp.attributes, vec![
            attr("action", "create_poll")
        ]);

        let msg = ExecuteMsg::CreatePoll { question: "Do you love Spark IBC".to_string()
     };

        let _resp = execute(deps.as_mut(), env, info, msg).unwrap_err();

    }

    #[test]
    fn test_vote() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string() // String, String::from("addr1")
        };
        // Before execute you need to instantiate the contract 
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = QueryMsg::GetPoll{
            question: "Do you love Spark IBC?".to_string(),
        };

        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let get_poll_response: GetPollResponse = from_binary(&resp).unwrap();

        assert_eq!(get_poll_response, GetPollResponse {
            poll: Some(Poll {
                question: "Do you love Spark IBC".to_string(),
                yes_vote: 0,
                no_vote: 0,
            })
        });


        // we need a poll to vote on !
        let msg = ExecuteMsg::CreatePoll { question: "Do you love Spark IBC".to_string()
     };

        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Sucess case , we vote on a poll that exist, with avalid option

        let msg = ExecuteMsg::Vote{
            question: "Do you love Spark IBC".to_string(),
            choice: "yes".to_string(),
        };
        let resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        assert_eq!(resp.attributes, vec![attr("action", "vote"),]);

        // Error-case 1: we  vote on a poll that does not exist
        let msg = ExecuteMsg::Vote{
            question: "Do you love me?".to_string(),
            choice: "yes".to_string(),
        };
        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

        //assert_eq!(resp.attributes, vec![attr("action", "vote"),]); 

        // vote on invalid choice  
         let msg = ExecuteMsg::Vote{
            question: "Do you love Spark IBC".to_string(),
            choice: "asdfasd".to_string(),
        };
        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();   
    }
}
