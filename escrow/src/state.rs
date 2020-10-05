use crate::error::ContractError;
use cosmwasm_std::{CanonicalAddr, Env, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub arbiter: CanonicalAddr,
    pub recipient: CanonicalAddr,
    pub source: CanonicalAddr,
    pub end_height: Option<u64>,
    pub end_time: Option<u64>,
}

impl State {
    pub fn is_expired(&self, env: &Env) -> ContractError {
        if let Some(end_height) = self.end_height {
            return if env.block.height > end_height {
                ContractError::EscrowExpiredHeight { end_height }
            } else {
                ContractError::EscrowNotExpired {}
            };
        }

        if let Some(end_time) = self.end_time {
            return if env.block.time > end_time {
                ContractError::EscrowExpiredTime { end_time }
            } else {
                ContractError::EscrowNotExpired {}
            };
        }
        ContractError::EscrowNotExpired {}
    }
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
