extern crate chrono;
extern crate cronos_cron;

pub mod delegate;
pub mod errors;
pub mod events;
pub mod id;
pub mod pda;
pub mod responses;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

#[program]
pub mod cronos_scheduler {
    use super::*;

    pub fn task_new(ctx: Context<TaskNew>, ixs: Vec<InstructionData>) -> Result<()> {
        task_new::handler(ctx, ixs)
    }

    pub fn task_update(ctx: Context<TaskUpdate>, ixs: Vec<InstructionData>) -> Result<()> {
        task_update::handler(ctx, ixs)
    }

    pub fn admin_config_update(
        ctx: Context<AdminConfigUpdate>,
        settings: ConfigSettings,
    ) -> Result<()> {
        admin_config_update::handler(ctx, settings)
    }

    pub fn admin_fee_collect(ctx: Context<AdminFeeCollect>) -> Result<()> {
        admin_fee_collect::handler(ctx)
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn admin_queue_new(ctx: Context<AdminQueueNew>, schedule: String) -> Result<()> {
        admin_queue_new::handler(ctx, schedule)
    }

    pub fn admin_queue_cancel(ctx: Context<AdminQueueCancel>) -> Result<()> {
        admin_queue_cancel::handler(ctx)
    }

    pub fn yogi_fund(ctx: Context<YogiFund>, amount: u64) -> Result<()> {
        yogi_fund::handler(ctx, amount)
    }

    pub fn yogi_new(ctx: Context<YogiNew>) -> Result<()> {
        yogi_new::handler(ctx)
    }

    pub fn yogi_sign(ctx: Context<YogiSign>, ix: InstructionData) -> Result<()> {
        yogi_sign::handler(ctx, ix)
    }

    pub fn queue_begin(ctx: Context<QueueBegin>) -> Result<()> {
        queue_begin::handler(ctx)
    }

    pub fn queue_cancel(ctx: Context<QueueCancel>) -> Result<()> {
        queue_cancel::handler(ctx)
    }

    pub fn queue_new(ctx: Context<QueueNew>, schedule: String) -> Result<()> {
        queue_new::handler(ctx, schedule)
    }

    pub fn queue_exec(ctx: Context<QueueExec>) -> Result<()> {
        queue_exec::handler(ctx)
    }
}
