use crate::context::State;
use crate::errors::Error;
use crate::services::*;

use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, panic_with_error, AccountId, Address, BytesN, Env, Vec};

pub mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_contract.wasm");
}


pub trait ParentAllowanceTrait {
    fn initialize(
        env: Env,
        admin: AccountId,
        token_address: BytesN<32>,
        step_period: u64,              //The interval in seconds for each allowance to be available. They stack up over time.
        start_period: u64,             //The exact timestamp to when the allowance starts to be accrued. '0' stats right away. 
        end_period: u64,               //The exact timestamp to when the allowance stops to be accrued. '0' runs indefinitely.
    );

    // Defines an allowance amount for a specific child account to be accrued at each step_period
    fn setAllow(env: Env, child_account: AccountId, allowance: i128);

    // Check the current allowance for a child account
    fn getAllow(env: Env, child_account: AccountId) -> i128;

    // Get the amount of allowance already withdrawn by a given child account
    fn getWthdrwn(env: Env, child_account: AccountId) ->  i128;

    // Get the amount of allowance available for a given child account
    fn getAlwAv(env: Env, child_account: AccountId) -> i128;

    // Get the start_period
    fn getStartP(env: Env) -> u64;

    // Get the step_period
    fn getStepP(env: Env) -> u64;

    // Get the end_period
    fn getEndP(env: Env) -> u64;

    // Withdraws an amount of allowance to a given child account if available
    fn withdraw(env: Env, child_account: AccountId, draw_amount: i128) -> Result<(), Error>;
  

}

pub struct ParentAllowance;

//TODO
// add return result / error
#[contractimpl]
impl ParentAllowanceTrait for ParentAllowance {
    fn initialize(
        env: Env,
        admin: AccountId,
        token_address: BytesN<32>,
        step_period: u64,
        start_period: u64,
        end_period: u64
    ) {
        if read_state(&env) != State::NotInititd {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }

        // The step_period defines the interval for each withdraw to be performed. 
        // Setting as 0 would cause a division by 0 so it is not accepted.
        if step_period == 0 {
            panic_with_error!(&env, Error::InvalidArguments);
        }

        write_state(&env, State::Initiated);
        write_admin(&env, admin);
        write_token_address(&env, token_address);
        write_step_period(&env, step_period);
        
        //stores the end_period. When set to 0, there is no final date and the contract just keeps on going.
        write_end_period(&env, end_period);

        //when start_period is set as 0, the allowance distribution starts right away
        //otherwise, it is programmed to start at the informed timestamp
        if start_period == 0 {
            write_start_period(&env, env.ledger().timestamp());
        }
        else {
            write_start_period(&env, start_period);
        }        

    }

    fn setAllow(env: Env, child_account: AccountId, allowance: i128){
        write_allowance(&env, child_account, allowance);

    }


    fn getAllow(env: Env, child_account: AccountId) -> i128{
        read_allowance(&env, child_account)
    }


    fn getStartP(env: Env) -> u64{
        read_start_period(&env)
    }

    fn getStepP(env: Env) -> u64{
        read_step_period(&env)
    }

    fn getEndP(env: Env) -> u64{
        read_end_period(&env)
    }

    fn getAlwAv(env: Env, child_account: AccountId) -> i128{

        let start_period = read_start_period(&env);
        let step_period = read_step_period(&env);
        let child_allowance = read_allowance(&env, child_account.clone());
        let withdrawn_allowance = read_withdrawn_allowance(&env, child_account.clone());

        return calculate_allowance_available(&env, 
            start_period, 
            step_period, 
            child_allowance, 
            withdrawn_allowance);

    }

    fn getWthdrwn(env: Env, child_account: AccountId) ->  i128{
        read_withdrawn_allowance(&env, child_account)
    }



    //TODO:
    // use invoker as child_account
    fn withdraw(env: Env, child_account: AccountId, draw_amount: i128) -> Result<(), Error>{

        // This is a simple check to ensure the `withdraw` function has not been
        // invoked by a contract. For our purposes, it *must* be invoked by a
        // user account.
        match env.invoker() {
            Address::Account(id) => id,
            _ => panic_with_error!(&env, Error::InvalidInvoker),
        };


        // Verifies if we're past the start_period already
        // Allowance only starts to run after the start_period
        let start_period = read_start_period(&env);
        if  env.ledger().timestamp() < start_period {
            panic_with_error!(&env, Error::AllowancePeriodNotSarted);
        }

        // Verifies if we're under the end_period or there isn't an end_period
        // Allowance only accrues up until the end_period or indefinitely if end_period = 0
        let end_period = read_end_period(&env);
        if  env.ledger().timestamp() > end_period && end_period > 0 {
            panic_with_error!(&env, Error::AllowancePeriodEnded);
        }


        let child_allowance = read_allowance(&env, child_account.clone());
        let token_address = read_token_address(&env);
        let step_period = read_step_period(&env);
        let withdrawn_allowance = read_withdrawn_allowance(&env, child_account.clone());
        let parent_account = read_admin(&env);

        let token_client = token::Client::new(&env, token_address);


        //calculate allowance
        let amount_available = calculate_allowance_available(&env, 
                                                             start_period, 
                                                             step_period, 
                                                             child_allowance, 
                                                             draw_amount);

        //Verifies if the child is trying to withdraw an amount within the allowance already available
        if amount_available < 0{
            panic_with_error!(&env, Error::InsufficientAllowance);
        }

        //update withdrawn value
        write_withdrawn_allowance(&env, child_account.clone(), (draw_amount + withdrawn_allowance));

        //Transfer the withdrawn value from the parent account to the child account
        token_client.xfer_from(
            &Signature::Invoker,
            &(0 as i128),
            &Identifier::Account(parent_account),
            &Identifier::Account(child_account),
            &draw_amount,
        );

        Ok(())

    }

}
