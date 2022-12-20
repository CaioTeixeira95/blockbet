use crate::{
    data::{Bet, Match},
    errors::Error,
    services::{has_admin, is_admin, verify_and_consume_nonce, write_admin, write_bet},
};
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, panic_with_error, Env};

pub struct BetContract;

#[contractimpl]
impl BetContract {
    pub fn init(env: Env, admin: Identifier) {
        if has_admin(&env) {
            panic_with_error!(env, Error::AdminAlreadySet)
        }

        write_admin(&env, admin);
    }

    pub fn add_bet(
        env: Env,
        admin: Signature,
        nonce: i128,
        match_object: Match,
        user: Identifier,
        bet: Bet,
    ) {
        let admin_identifier = admin.identifier(&env);

        if !is_admin(&env, admin_identifier) {
            panic_with_error!(env, Error::NotAuthorized)
        }

        verify_and_consume_nonce(&env, &admin, nonce);

        write_bet(&env, match_object, user, bet)
    }
}
