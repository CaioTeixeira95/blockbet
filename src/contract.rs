use crate::{errors::Error, services::{has_admin, write_admin}};
use soroban_auth::Identifier;
use soroban_sdk::{contractimpl, Env, panic_with_error};

pub struct BetContract;

#[contractimpl]
impl BetContract {
    pub fn init(env: Env, admin: Identifier) {
        if has_admin(&env) {
            panic_with_error!(env, Error::AdminAlreadySet)
        }

        write_admin(&env, admin);
    }
}
