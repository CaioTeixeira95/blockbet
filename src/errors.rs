use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    AdminAlreadySet = 1,
    NotAuthorized = 2,
    NonZeroNonce = 3,
    InvalidNonce = 4,
}
