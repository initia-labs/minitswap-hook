use cosmwasm_std::{Addr, CosmosMsg, Binary, Coin as CosmosCoin, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use protobuf::{Message, MessageField};
use crate::msgs::{InstantiateMsg, ExecuteMsg};
use crate::state::Contract;
use crate::msg_initiate_token_withdrawal::{Coin, MsgInitiateTokenWithdrawal};

impl<'a> Contract {
    pub fn instantiate(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: InstantiateMsg
    ) -> StdResult<Response> {
        Ok(Response::new())
    }

    pub fn execute(
        &self,
        _deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg
    ) -> StdResult<Response> {
        match msg {
            ExecuteMsg::MinitswapHook { receiver } => self.withdraw(env, info, receiver)
        }
    }

    fn withdraw(&self, env: Env, info: MessageInfo, recevier: Addr) -> StdResult<Response> {
        if info.funds.len() != 1 {
            return Err(StdError::generic_err("invalid funds length"))
        };

        let special_fields = ::protobuf::SpecialFields::new();
        let CosmosCoin { denom, amount } = &info.funds[0];

        let init_withdrawal = MsgInitiateTokenWithdrawal {
            sender: env.contract.address.to_string(),
            to: recevier.to_string(),
            amount: MessageField::some(Coin { denom: denom.clone(), amount: amount.to_string(), special_fields: special_fields.clone() }),
            special_fields
        };

        let msg =  CosmosMsg::Stargate {
            type_url: "/opinit.opchild.v1.MsgInitiateTokenWithdrawal".to_string(),
            value: Binary::from(init_withdrawal.write_to_bytes().unwrap()),
        };

        Ok(Response::new().add_message(msg))
    }
}
