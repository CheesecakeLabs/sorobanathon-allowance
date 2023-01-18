use soroban_sdk::{AccountId, contracttype};
//use soroban_auth::Identifier;


#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub enum State {
    NotInititd,
    Initiated,
    Started,
    Finished,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,      // AccountId
    Allowance(AccountId),  // i128
    WithdAllow(AccountId), // i128
    StpPeriod,  // u64
    StrtPeriod,  // u64
    EndPeriod,  // u64
    TokenAddr,  // BytesN
    State,      // enum State
    // TicktPrice, // i128
    // Users,      // Vec<Identifier>
}