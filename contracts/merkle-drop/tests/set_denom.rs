mod test_env;
use merkle_drop::msg::{ExecuteMsg, GetSubDenomResponse, QueryMsg};
use osmosis_testing::{Module, Wasm};
use test_env::*;

test_set_denom!(
    set_denom_valid_owner
    should succeed
);

// ======= helpers ========

#[macro_export]
macro_rules! test_set_denom {
    ($test_name:ident should succeed) => {
        #[test]
        fn $test_name() {
            test_set_denom_success_case()
        }
    };
}

fn test_set_denom_success_case() {
    let TestEnv {
        app,
        contract_address,
        owner,
        valid_sender: _,
    } = TestEnv::new();

    let subdenom = String::from(VALID_SUBDENOM);

    let set_subdenom_msg = ExecuteMsg::SetSubDenom {
        subdenom: subdenom.clone(),
    };

    let wasm = Wasm::new(&app);
    let res = wasm.execute(&contract_address, &set_subdenom_msg, &[], &owner);

    // check if execution succeeded
    assert!(res.is_ok(), "{:?}", res.unwrap_err());

    let get_subdenom_query = QueryMsg::GetSubdenom {};

    let q_res = wasm
        .query::<QueryMsg, GetSubDenomResponse>(&contract_address, &get_subdenom_query)
        .unwrap();

    assert_eq!(q_res.subdenom, subdenom.clone());
}
