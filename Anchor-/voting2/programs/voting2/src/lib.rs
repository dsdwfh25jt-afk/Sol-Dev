use anchor_lang::prelude::*;
declare_id!("27N9MNfuVWgFvg74M5WLSrYZYMRfYoEGPL8UzjPAVWB4");

#[program]
pub mod voting2 {
    use super::*;

    pub fn initializePoll(ctx: Context<InitializePoll>,
            poll_id : u64,
            description: String,
            poll_start : u64,
            poll_end : u64,
        ) -> Result<()> {
        
            let poll = &mut ctx.account.poll_account;
            poll_account.poll_id = poll_id;
            poll_account.poll_start = poll_start;
            poll_account.poll_end = poll_end;
            poll_account.description = description;
        Ok(())
    }

    // pub fn initializeCandidate(ctx: Context<Initialize>) -> Result<()> {
    //     initialize::handler(ctx)
    // }

    // pub fn vote(ctx: Context<Vote>) -> Result<()> {
    //     initialize::handler(ctx)
    // }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll <'info> {
    #[account(mut)]
    pub signer : Signer<'info>,

    #[account(
        init ,
        payer = signer,
        space = 8 + Poll::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll_account : Account<'info, Poll>, 

    pub system_program : Program<'info , System>,
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id : u64,

    #[max_len(256)]
    pub discriiption : String,

    pub poll_start : u64,

    pub poll_end : u64,

    pub candidate_amount : u64,
}