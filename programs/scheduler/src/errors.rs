use anchor_lang::prelude::*;

#[error_code]
pub enum CronosError {
    #[msg("Delegate addresses cannot be initialized accounts")]
    DelegateDataNotEmpty,

    #[msg("An task's inner ix failed to execute")]
    InnerIxFailed,
    #[msg("An inner instructure wants to mutate state owned by the scheduler")]
    InnerIxReentrancy,

    #[msg("The queue is current executing another task")]
    InvalidTask,
    #[msg("The dynamic account list is not the expect size")]
    InvalidDynamicAccounts,
    #[msg("The exec response value could not be parsed")]
    InvalidExecResponse,
    #[msg("The cron expression is invalid")]
    InvalidSchedule,
    #[msg("Your yogi cannot provide all required signatures for this instruction")]
    InvalidSignatory,
    #[msg("The queue does not have the right status for this operation")]
    InvalidQueueStatus,

    #[msg("Your are not the admin authority")]
    NotAdmin,
    #[msg("You are not the owner of this yogi")]
    NotYogiOwner,

    #[msg("The queue is not due")]
    QueueNotDue,
}
