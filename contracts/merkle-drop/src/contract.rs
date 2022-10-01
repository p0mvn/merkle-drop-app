#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, BankMsg
};
use serde::de::Error;
use serde_json;
use cw2::set_contract_version;
use merkle::{Tree, proof::Proof, hash::Hash};
use base64;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetMerkleRootResponse, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};
use crate::execute::{verify_proof};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:merkle-drop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        merkle_root: msg.merkle_root,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Claim { proof, amount } => claim(deps, info, proof, amount),
    }
}

pub fn claim(
    deps: DepsMut,
    info: MessageInfo,
    proof_str: String,
    amount: Coin,
) -> Result<Response, ContractError> {

    let sender = info.sender.as_str();
    let to_prove = format!("{}{}", sender, amount.to_string());

    let root_encoded = CONFIG.load(deps.storage).unwrap().merkle_root;

    verify_proof(&root_encoded, &to_prove, &proof_str)?;

    Ok(Response::new().add_attribute("method", "verify_proof"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_root(deps)?),
    }
}

fn query_root(deps: Deps) -> StdResult<GetMerkleRootResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(GetMerkleRootResponse {
        root: config.merkle_root,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    // TEST_ROOT test merkel root that was generated from "testdata/uosmo_only.csv" using merkle-drop-cli
    const TEST_ROOT: &str = "bd9c439f3903b3dbc92bad230df593d434aada80f26e8124d77d2f92fbaa6238";

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            merkle_root: String::from(TEST_ROOT),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetMerkleRootResponse = from_binary(&res).unwrap();
        assert_eq!(TEST_ROOT, value.root);
    }
}
