
use crate::context::{DataKey, State};
use crate::errors::Error;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{panic_with_error, vec, AccountId, Address, BytesN, Env, Vec};

//
// Write functions
//

pub fn write_state(env: &Env, state: State) {
    env.storage().set(DataKey::State, state);
}

pub fn write_admin(env: &Env, admin: AccountId) {
    env.storage().set(DataKey::Admin, admin);
}

pub fn write_token_address(env: &Env, token_address: BytesN<32>) {
    env.storage().set(DataKey::TokenAddr, token_address);
}

pub fn write_allowance(env: &Env, child_account: AccountId, allowance: i128) {
    env.storage().set(DataKey::Allowance(child_account), allowance);
}

pub fn write_start_period(env: &Env, start_period: u64) {
    env.storage().set(DataKey::StrtPeriod, start_period);
}

pub fn write_end_period(env: &Env, end_period: u64) {
    env.storage().set(DataKey::EndPeriod, end_period);
}

pub fn write_step_period(env: &Env, step_period: u64) {
    env.storage().set(DataKey::StpPeriod, step_period);
}

pub fn write_withdrawn_allowance(env: &Env, child_account: AccountId, amount: i128) {
    env.storage().set(DataKey::WithdAllow(child_account), amount);
}



//
// Read functions
//

pub fn read_state(env: &Env) -> State {
    env.storage()
        .get(DataKey::State)
        .unwrap_or(Ok(State::NotInititd))
        .unwrap()
}

pub fn read_admin(env: &Env) -> AccountId {
    env.storage().get_unchecked(DataKey::Admin).unwrap()
}

pub fn read_allowance(env: &Env, child_account: AccountId) -> i128 {
    env.storage().get_unchecked(DataKey::Allowance(child_account)).unwrap()
}

pub fn read_start_period(env: &Env) -> u64 {
    env.storage().get_unchecked(DataKey::StrtPeriod).unwrap()
}

pub fn read_end_period(env: &Env) -> u64 {
    env.storage().get_unchecked(DataKey::EndPeriod).unwrap()
}

pub fn read_step_period(env: &Env) -> u64 {
    env.storage().get_unchecked(DataKey::StpPeriod).unwrap()
}

pub fn read_token_address(env: &Env) -> BytesN<32> {
    env.storage().get_unchecked(DataKey::TokenAddr).unwrap()
}

pub fn read_withdrawn_allowance(env: &Env, child_account: AccountId) -> i128 {
    env.storage().get(DataKey::WithdAllow(child_account)).unwrap_or(Ok(0)).unwrap()
}


//
// Aux Functions
//

pub fn calculate_allowance_available(env: &Env, 
                                     start_period: u64, 
                                     step_period: u64, 
                                     child_allowance: i128, 
                                     withdrawn_allowance: i128) -> i128{
    let seconds_elapsed = env.ledger().timestamp() - start_period;
    return ((seconds_elapsed / step_period)as i128 * child_allowance) - withdrawn_allowance;
}

fn to_account(address: Address) -> Result<AccountId, Error> {
    match address {
        Address::Account(id) => Ok(id),
        _ => Err(Error::InvalidInvoker),
    }
}