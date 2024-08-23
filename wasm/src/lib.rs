mod execute;
mod msgs;
mod state;
mod msg_initiate_token_withdrawal;

use crate::msgs::{InstantiateMsg, 
    // QueryMsg, MigrateMsg
};
use crate::state::Contract;

#[cfg(not(feature = "library"))]
pub mod entry {
  use self::msgs::ExecuteMsg;

  use super::*;

  use cosmwasm_std::{entry_point, Empty};
  use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};


  #[entry_point]
  pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
  ) -> StdResult<Response> {
    let tract = Contract::default();
    tract.instantiate(deps, env, info, msg)
  }

  #[entry_point]
  pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
  ) -> StdResult<Response> {
    let tract = Contract::default();
    tract.execute(deps, env, info, msg)
  }

  #[entry_point]
  pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    Ok(Response::new())
  }
}
