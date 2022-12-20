use crate::{data::{DataKey, Match, Bet}, errors::Error};
use soroban_sdk::{Env, map, panic_with_error};
use soroban_auth::{Identifier, Signature};

pub fn has_admin(env: &Env) -> bool {
    env.storage().has(DataKey::Admin)
}

pub fn is_admin(env: &Env, identifier: Identifier) -> bool {
    let admin: Identifier = env.storage().get_unchecked(DataKey::Admin).unwrap();
    admin == identifier
}

pub fn write_admin(env: &Env, admin: Identifier) {
    env.storage().set(DataKey::Admin, admin)
}

pub fn write_bet(env: &Env, match_object: Match, user: Identifier, bet: Bet) {
    let mut bets = env.storage()
        .get(DataKey::Match(match_object))
        .unwrap_or(Ok(map![env, (user.clone(), bet)]))
        .unwrap();
    
    bets.set(user, bet);

    env.storage().set(DataKey::Match(match_object), bets)
}

pub fn verify_and_consume_nonce(env: &Env, auth: &Signature, expected_nonce: i128) {
    if auth == &Signature::Invoker && expected_nonce != 0 {
        panic_with_error!(env, Error::NonZeroNonce)
    }

    let nonce = read_admin_nonce(env);

    if nonce != expected_nonce {
        panic_with_error!(env, Error::InvalidNonce)
    }

    set_admin_nonce(env, &nonce + 1)
}

pub fn read_admin_nonce(env: &Env) -> i128 {
    env.storage().get(DataKey::Nonce).unwrap_or(Ok(0)).unwrap()
}

pub fn set_admin_nonce(env: &Env, nonce: i128) {
    env.storage().set(DataKey::Nonce, nonce)
}
