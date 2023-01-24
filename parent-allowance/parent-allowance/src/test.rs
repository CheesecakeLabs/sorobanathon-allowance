#![cfg(test)]


use crate::context::{DataKey, State};
use crate::contract::{token, ParentAllowance, ParentAllowanceClient};
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{
    testutils::{Accounts, Ledger, LedgerInfo},
    AccountId, BytesN, Env, IntoVal,
};

fn create_token_contract(
    env: &Env,
    admin: &AccountId,
    name: &str,
    symbol: &str,
    decimal: u32,
) -> (BytesN<32>, token::Client) {
    let token_contract_id = env.register_contract_wasm(None, token::WASM);
    let token_client = token::Client::new(env, token_contract_id.clone());

    token_client.initialize(
        &Identifier::Account(admin.clone()),
        &decimal,
        &name.into_val(env),
        &symbol.into_val(env),
    );

    (token_contract_id.clone(), token_client)
}

fn updates_contract_time(env: &Env, contract_id: BytesN<32>, time: u64) -> ParentAllowanceClient {
    env.ledger().set(LedgerInfo {
        timestamp: time,
        protocol_version: 1,
        sequence_number: 10,
        network_passphrase: Default::default(),
        base_reserve: 10,
    });
    return ParentAllowanceClient::new(&env, &contract_id);
}

//Make sure the contract cannot be initialized more than once
#[test]
#[should_panic(expected = "Status(ContractError(1)")]
fn test_invalid_contract_initialize_panics_when_contract_is_initialized() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ParentAllowance);
    let client = ParentAllowanceClient::new(&env, contract_id.clone());

    let admin = env.accounts().generate();
    //let admin_id = Identifier::Account(admin.clone());
    let step_period = 10;
    let start_period = 10;
    let end_period = 10;

    env.as_contract(&contract_id, || {
        env.storage().set(DataKey::State, State::Initiated)
    });

    client.initialize(
        &admin,
        &contract_id,
        &step_period,
        &start_period,
        &end_period,
    );
}

//Test the initalized start, step and end periods
#[test]
fn test_valid_initialized_periods() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ParentAllowance);
    let mut client = ParentAllowanceClient::new(&env, contract_id.clone());

    //set the initial state for the ledger
    client = updates_contract_time(&env, contract_id.clone(), 1669726146);

    let admin = env.accounts().generate();
    //let admin_id = Identifier::Account(admin.clone());
    let step_period = 86400;
    let start_period = 0;
    let end_period = 0;

    client.initialize(
        &admin,
        &contract_id,
        &step_period,
        &start_period,
        &end_period,
    );

    assert_eq!(1669726146, client.get_start());

    assert_eq!(86400, client.get_step());

    assert_eq!(0, client.get_end());
}

//verify the initialization parameters were stored correctly and the
//Children allowance is set properly for multiple kids
#[test]
fn test_valid_children_registration() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ParentAllowance);
    let client = ParentAllowanceClient::new(&env, contract_id.clone());

    let admin = env.accounts().generate();
    //let admin_id = Identifier::Account(admin.clone());
    let step_period = 10;
    let start_period = 10;
    let end_period = 10;

    client.initialize(
        &admin,
        &contract_id,
        &step_period,
        &start_period,
        &end_period,
    );

    //register child a allowance and verify
    let child_a_account = env.accounts().generate();
    //let child_a_account_id = Identifier::Account(child_a_account.clone());
    let child_a_allowance: i128 = 10;

    client.with_source_account(&admin).set_allow(&child_a_account, &child_a_allowance);

    assert_eq!(child_a_allowance, client.get_allow(&child_a_account));

    //register child b allowance and verify
    let child_b_account = env.accounts().generate();
    //let child_b_account_id = Identifier::Account(child_b_account.clone());
    let child_b_allowance: i128 = 20;

    client.with_source_account(&admin).set_allow(&child_b_account, &child_b_allowance);

    assert_eq!(child_b_allowance, client.get_allow(&child_b_account));
}

// Test if the contract is running correctly by adding two children,
// waiting some time and performing withdraw with both.
#[test]
fn test_valid_sequence_withdraw() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ParentAllowance);
    let mut client = ParentAllowanceClient::new(&env, contract_id.clone());

    //set the initial state for the ledger
    client = updates_contract_time(&env, contract_id.clone(), 1669726146);

    let admin = env.accounts().generate();

    let (payment_tkn_id, payment_tkn) =
        create_token_contract(&env, &admin, &"USD Coin", &"USDC", 8);

    // We use the `admin` account to mint 1,000,000,000 Stroops of our token (that
    // is equal to 100 units of the asset).
    payment_tkn.with_source_account(&admin).mint(
        &Signature::Invoker,
        &0,
        &Identifier::Account(admin.clone()),
        &1000000000,
    );

    //verify balance minted
    assert_eq!(
        payment_tkn.balance(&Identifier::Account(admin.clone()),),
        1000000000
    );

    // We invoke the token contract's `approve` function as the `u1` account,
    // allowing our AllowanceContract to spend tokens out of the `u1` balance.
    // We are giving the contract a 500,000,000 Stroop (== 50 units) allowance.
    payment_tkn.with_source_account(&admin).incr_allow(
        &Signature::Invoker,
        &0,
        &Identifier::Contract(contract_id.clone()),
        &500000000,
    );

    // We invoke the token contract's `allowance` function to ensure everything
    // has worked up to this point.
    assert_eq!(
        payment_tkn.allowance(
            &Identifier::Account(admin.clone()),
            &Identifier::Contract(contract_id.clone()),
        ),
        500000000
    );

    let step_period = 86400; // 1 day in seconds
    let start_period = 0; // starts right aways
    let end_period = 0; // no end date

    client.initialize(
        &admin,
        &payment_tkn_id,
        &step_period,
        &start_period,
        &end_period,
    );

    //register child a
    let child_a_account = env.accounts().generate();
    let child_a_allowance: i128 = 100;
    client.with_source_account(&admin).set_allow(&child_a_account, &child_a_allowance);

    //register child b
    let child_b_account = env.accounts().generate();
    let child_b_allowance: i128 = 150;
    client.with_source_account(&admin).set_allow(&child_b_account, &child_b_allowance);

    //after 1 day + 1000 seconds
    //child a withdraws 50
    client = updates_contract_time(&env, contract_id.clone(), 1669726146 + (86400 + 1000));

    let child_a_withdraw_amount: i128 = 50;
    client.withdraw(&child_a_account, &child_a_withdraw_amount);
    assert_eq!(
        50,
        client.get_wthdr(&child_a_account),
        "child A withdraws 50, must have 50 withdrawn "
    );
    assert_eq!(
        50,
        client.get_aval(&child_a_account),
        "child A withdraws 50, must have 50 left"
    );

    //verify that parent account balance has been updated
    assert_eq!(
        payment_tkn.balance(&Identifier::Account(admin.clone()),),
        1000000000 - child_a_withdraw_amount
    );

    // //after 2 days + 1000 seconds
    // //child b withdraws 70
    client = updates_contract_time(&env, contract_id.clone(), 1669726146 + ((86400 * 2) + 1000));

    let child_b_withdraw_amount: i128 = 70;
    client.withdraw(&child_b_account, &child_b_withdraw_amount);
    assert_eq!(
        230,
        client.get_aval(&child_b_account),
        "child B withdraws 70, must have 230 left"
    );

    //verify that parent account balance has been updated
    assert_eq!(
        payment_tkn.balance(&Identifier::Account(admin.clone()),),
        1000000000 - child_a_withdraw_amount - child_b_withdraw_amount
    );
}

// Test if the contract is running correctly by adding two children,
// waiting some time and performing withdraw with both.
#[test]
#[should_panic(expected = "Status(ContractError(7)")]
fn test_invalid_sequence_withdraw_panics_when_allowance_is_insufficient() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ParentAllowance);
    let mut client = ParentAllowanceClient::new(&env, contract_id.clone());

    //set the initial state for the ledger
    client = updates_contract_time(&env, contract_id.clone(), 1669726146);

    let admin = env.accounts().generate();

    let (payment_tkn_id, payment_tkn) =
        create_token_contract(&env, &admin, &"USD Coin", &"USDC", 8);

    let step_period = 86400; // 1 day in seconds
    let start_period = 0; // starts right aways
    let end_period = 0; // no end date

    client.initialize(
        &admin,
        &payment_tkn_id,
        &step_period,
        &start_period,
        &end_period,
    );

    //register child a
    let child_a_account = env.accounts().generate();
    let child_a_allowance: i128 = 100;
    client.with_source_account(&admin).set_allow(&child_a_account, &child_a_allowance);

    //after 1 day + 1000 seconds
    //child a attempts to withdraw 110
    client = updates_contract_time(&env, contract_id.clone(), 1669726146 + (86400 + 1000));

    let child_a_withdraw_amount: i128 = 110;
    client.withdraw(&child_a_account, &child_a_withdraw_amount);
}

// Test if the contract is running correctly by adding two children,
// waiting some time and performing withdraw with both.
#[test]
#[should_panic(expected = "Status(ContractError(8)")]
fn test_invalid_withdraw_panics_when_past_end_period() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ParentAllowance);
    let mut client = ParentAllowanceClient::new(&env, contract_id.clone());

    //set the initial state for the ledger
    client = updates_contract_time(&env, contract_id.clone(), 1669726146);

    let admin = env.accounts().generate();

    let (payment_tkn_id, payment_tkn) =
        create_token_contract(&env, &admin, &"USD Coin", &"USDC", 8);

    let step_period = 86400; // 1 day in seconds
    let start_period = 0; // starts right away
    let end_period = 1669800000; // future end date

    client.initialize(
        &admin,
        &payment_tkn_id,
        &step_period,
        &start_period,
        &end_period,
    );

    //register child a
    let child_a_account = env.accounts().generate();
    let child_a_allowance: i128 = 100;
    client.with_source_account(&admin).set_allow(&child_a_account, &child_a_allowance);

    //after end period
    //child a attempts to withdraw 50
    client = updates_contract_time(&env, contract_id.clone(), 1669800000 + 1);

    let child_a_withdraw_amount: i128 = 50;
    client.withdraw(&child_a_account, &child_a_withdraw_amount);
}

// Test if the contract is running correctly by adding two children,
// waiting some time and performing withdraw with both.
#[test]
#[should_panic(expected = "Status(ContractError(9)")]
fn test_invalid_withdraw_panics_when_before_start_period() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ParentAllowance);
    let mut client = ParentAllowanceClient::new(&env, contract_id.clone());

    //set the initial state for the ledger
    client = updates_contract_time(&env, contract_id.clone(), 1669726146);

    let admin = env.accounts().generate();

    let (payment_tkn_id, payment_tkn) =
        create_token_contract(&env, &admin, &"USD Coin", &"USDC", 8);

    let step_period = 86400; // 1 day in seconds
    let start_period = 1669800000; // starts in future date
    let end_period = 0; // no end date

    client.initialize(
        &admin,
        &payment_tkn_id,
        &step_period,
        &start_period,
        &end_period,
    );

    //register child a
    let child_a_account = env.accounts().generate();
    let child_a_allowance: i128 = 100;
    client.with_source_account(&admin).set_allow(&child_a_account, &child_a_allowance);

    //before start period
    //child a attempts to withdraw 50
    client = updates_contract_time(&env, contract_id.clone(), 1669800000 - 1);

    let child_a_withdraw_amount: i128 = 50;
    client.withdraw(&child_a_account, &child_a_withdraw_amount);
}
