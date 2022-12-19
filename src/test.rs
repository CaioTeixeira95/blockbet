#![cfg(test)]
use crate::{data::DataKey, contract::{BetContract, BetContractClient}};
use soroban_auth::testutils;
use soroban_sdk::Env;

#[test]
#[should_panic(expected = "Status(ContractError(1)")]
fn init_contract_panics_when_admin_is_already_set() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BetContract);
    let (admin, _) = testutils::ed25519::generate(&env);

    let client = BetContractClient::new(&env, contract_id.clone());

    env.as_contract(&contract_id, || {
        env.storage().set(DataKey::Admin, admin.clone())
    });

    client.init(&admin);
}

#[test]
fn init_contract_successfully() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BetContract);
    let (admin, _) = testutils::ed25519::generate(&env);
    let client = BetContractClient::new(&env, &contract_id);

    client.init(&admin);

    env.as_contract(&contract_id, || {
        assert!(env.storage().has(DataKey::Admin))
    });
}
