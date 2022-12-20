use soroban_sdk::{contracttype};

#[contracttype]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Match{
    pub id: i128,
}

#[contracttype]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bet {
    pub amount: i128,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Match(Match),
    Nonce,
}
