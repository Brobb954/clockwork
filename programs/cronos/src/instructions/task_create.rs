use solana_program::log::sol_log_compute_units;

use {
    crate::{state::*, errors::CronosError},
    anchor_lang::prelude::*,
    solana_program::{system_program, sysvar},
    std::mem::size_of
};


#[derive(Accounts)]
#[instruction(
    ix: InstructionData,
    schedule: TaskSchedule,
    bump: u8,
)]
pub struct TaskCreate<'info> {
    #[account(
        address = sysvar::clock::ID,
        constraint = schedule.exec_at >= clock.unix_timestamp - 10 @ CronosError::InvalidExecAtStale
    )]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = daemon.bump,
        has_one = owner,
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_TASK, 
            daemon.key().as_ref(),
            daemon.task_count.to_be_bytes().as_ref(),
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<Task>() + borsh::to_vec(&ix).unwrap().len(),
    )]
    pub task: Account<'info, Task>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<TaskCreate>,
    ix: InstructionData,
    schedule: TaskSchedule,
    bump: u8,
) -> Result<()> {
    let config = &ctx.accounts.config;
    let daemon = &mut ctx.accounts.daemon;
    let owner = &ctx.accounts.owner;
    let task = &mut ctx.accounts.task;

    let res = task.init(config, daemon, owner.key(), ix, schedule, bump);

    sol_log_compute_units();

    res
}
