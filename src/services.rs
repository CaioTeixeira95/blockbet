use crate::data::DataKey;
use soroban_sdk::Env;
use soroban_auth::Identifier;

pub fn has_admin(env: &Env) -> bool {
    env.storage().has(DataKey::Admin)
}

pub fn write_admin(env: &Env, admin: Identifier) {
    env.storage().set(DataKey::Admin, admin)
}
