use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NotStarted = 3,
    NotAuthorized = 4,
    InvalidArguments = 5,
    InvalidInvoker = 6,
    InsufficientAllowance = 7 ,
    AllowancePeriodEnded = 8,
    AllowancePeriodNotSarted = 9
}