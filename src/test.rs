#![cfg(test)]
use crate::{
    contract::{BetContract, BetContractClient},
    data::{Bet, DataKey, Match},
};
use soroban_auth::{testutils, Identifier};
use soroban_sdk::{symbol, Env, Map};

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

    env.as_contract(&contract_id, || assert!(env.storage().has(DataKey::Admin)));
}

#[test]
#[should_panic(expected = "Status(ContractError(2)")]
fn add_bet_not_authorized() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BetContract);

    let (user, user_signature) = testutils::ed25519::generate(&env);
    let (admin, _) = testutils::ed25519::generate(&env);

    let nonce: i128 = 1;
    let match_object = Match { id: 1 };
    let bet = Bet { amount: 10 };

    let signature = testutils::ed25519::sign(
        &env,
        &user_signature,
        &contract_id,
        symbol!("add_bet"),
        (user.clone(), nonce, match_object, user.clone(), bet),
    );

    env.as_contract(&contract_id, || {
        env.storage().set(DataKey::Admin, admin.clone());
        env.storage().set(DataKey::Nonce, nonce)
    });

    let client = BetContractClient::new(&env, &contract_id);

    client.add_bet(&signature, &nonce, &match_object, &user, &bet);
}

#[test]
#[should_panic(expected = "Status(ContractError(4)")]
fn add_bet_invalid_nonce() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BetContract);

    let (user, _) = testutils::ed25519::generate(&env);
    let (admin, admin_signature) = testutils::ed25519::generate(&env);

    let nonce: i128 = 2;
    let match_object = Match { id: 1 };
    let bet = Bet { amount: 10 };

    let signature = testutils::ed25519::sign(
        &env,
        &admin_signature,
        &contract_id,
        symbol!("add_bet"),
        (admin.clone(), nonce, match_object, user.clone(), bet),
    );

    env.as_contract(&contract_id, || {
        env.storage().set(DataKey::Admin, admin.clone());
        env.storage().set(DataKey::Nonce, 1 as i128)
    });

    let client = BetContractClient::new(&env, &contract_id);

    client.add_bet(&signature, &nonce, &match_object, &user, &bet);
}

#[test]
fn add_bet_successfully() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BetContract);

    let (user, _) = testutils::ed25519::generate(&env);
    let (user2, _) = testutils::ed25519::generate(&env);

    let (admin, admin_signature) = testutils::ed25519::generate(&env);
    let nonce: i128 = 1;
    let match_object = Match { id: 1 };
    let bet = Bet { amount: 10 };

    let signature = testutils::ed25519::sign(
        &env,
        &admin_signature,
        &contract_id,
        symbol!("add_bet"),
        (admin.clone(), nonce, match_object, user.clone(), bet),
    );

    env.as_contract(&contract_id, || {
        env.storage().set(DataKey::Admin, admin.clone());
        env.storage().set(DataKey::Nonce, nonce)
    });

    let client = BetContractClient::new(&env, &contract_id);

    client.add_bet(&signature, &nonce, &match_object, &user, &bet);

    env.as_contract(&contract_id, || {
        assert!(env.storage().has(DataKey::Match(match_object)));

        let bets: Map<Identifier, Bet> = env
            .storage()
            .get_unchecked(DataKey::Match(match_object))
            .unwrap();

        assert!(bets.contains_key(user.clone()));
        assert_eq!(bets.contains_key(user2.clone()), false)
    });

    let nonce = 2;
    let signature = testutils::ed25519::sign(
        &env,
        &admin_signature,
        &contract_id,
        symbol!("add_bet"),
        (admin.clone(), nonce, match_object, user2.clone(), bet),
    );

    client.add_bet(&signature, &nonce, &match_object, &user2, &bet);

    env.as_contract(&contract_id, || {
        assert!(env.storage().has(DataKey::Match(match_object)));

        let bets: Map<Identifier, Bet> = env
            .storage()
            .get_unchecked(DataKey::Match(match_object))
            .unwrap();

        assert!(bets.contains_key(user));
        assert!(bets.contains_key(user2))
    });
}
