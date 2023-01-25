use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use clockwork_macros::TryFromData;

use crate::state::*;

pub const SEED_THREAD: &[u8] = b"thread";

/// Tracks the current state of a transaction thread on Solana.
#[account]
#[derive(Debug, TryFromData)]
pub struct Thread {
    /// The owner of this thread.
    pub authority: Pubkey,
    /// The bump, used for PDA validation.
    pub bump: u8,
    /// The cluster clock at the moment the thread was created.
    pub created_at: ClockData,
    /// The context of the thread's current execution state.
    pub exec_context: Option<ExecContext>,
    /// The number of lamports to payout to workers per execution.
    pub fee: u64,
    /// The id of the thread, given by the authority.
    pub id: Vec<u8>,
    /// The instructions to be executed.
    pub instructions: Vec<InstructionData>,
    /// The name of the thread.
    pub name: String,
    /// The next instruction to be executed.
    pub next_instruction: Option<InstructionData>,
    /// Whether or not the thread is currently paused.
    pub paused: bool,
    /// The maximum number of execs allowed per slot.
    pub rate_limit: u64,
    /// The triggering event to kickoff a thread.
    pub trigger: Trigger,
}

impl Thread {
    /// Derive the pubkey of a thread account.
    pub fn pubkey(authority: Pubkey, id: Vec<u8>) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_THREAD, authority.as_ref(), id.as_slice()],
            &crate::ID,
        )
        .0
    }
}

impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.authority.eq(&other.authority) && self.id.eq(&other.id)
    }
}

impl Eq for Thread {}

/// Trait for reading and writing to a thread account.
pub trait ThreadAccount {
    /// Get the pubkey of the thread account.
    fn pubkey(&self) -> Pubkey;

    /// Allocate more memory for the account.
    fn realloc(&mut self) -> Result<()>;
}

impl ThreadAccount for Account<'_, Thread> {
    fn pubkey(&self) -> Pubkey {
        Thread::pubkey(self.authority, self.id.clone())
    }

    fn realloc(&mut self) -> Result<()> {
        // Realloc memory for the thread account
        let data_len = 8 + self.try_to_vec()?.len();
        self.to_account_info().realloc(data_len, false)?;
        Ok(())
    }
}

/// The execution context of a particular transaction thread.
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecContext {
    /// Index of the next instruction to be executed.
    pub exec_index: usize,

    /// Number of execs since the last tx reimbursement.
    pub execs_since_reimbursement: u64,

    /// Number of execs in this slot.
    pub execs_since_slot: u64,

    /// Slot of the last exec
    pub last_exec_at: u64,

    /// Context for the triggering condition
    pub trigger_context: TriggerContext,
}

/// The triggering conditions of a thread.
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, PartialEq)]
pub enum Trigger {
    /// Allows a thread to be kicked off whenever the data of an account changes.
    Account {
        /// The address of the account to monitor.
        address: Pubkey,
        /// The byte offset of the account data to monitor.
        offset: usize,
        /// The size of the byte slice to monitor (must be less than 1kb)
        size: usize,
    },

    /// Allows a thread to be kicked off according to a one-time or recurring schedule.
    Cron {
        /// The schedule in cron syntax. Value must be parsable by the `clockwork_cron` package.
        schedule: String,

        /// Boolean value indicating whether triggering moments may be skipped if they are missed (e.g. due to network downtime).
        /// If false, any "missed" triggering moments will simply be executed as soon as the network comes back online.
        skippable: bool,
    },

    /// Allows a thread to be kicked off as soon as it's created.
    Immediate,
}

/// The event which allowed a particular transaction thread to be triggered.
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriggerContext {
    /// A running hash of the observed account data.
    Account {
        /// The account's data hash.
        data_hash: u64,
    },

    /// A cron execution context.
    Cron {
        /// The threshold moment the schedule was waiting for.
        started_at: i64,
    },

    /// The immediate trigger context.
    Immediate,
}

/// The properties of threads which are updatable.
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ThreadSettings {
    pub fee: Option<u64>,
    pub instructions: Option<Vec<InstructionData>>,
    pub name: Option<String>,
    pub rate_limit: Option<u64>,
    pub trigger: Option<Trigger>,
}
