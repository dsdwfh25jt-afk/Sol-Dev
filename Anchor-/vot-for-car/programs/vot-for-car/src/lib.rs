use anchor_lang::prelude::*;

declare_id!("FHXXfUX2UVCpVWn2iTTYGA23fnUCyUAXpWwARJhJFTuJ");

#[program] 

pub mod vot_for_car {
    use super::*;
    pub fn initial(ctx: Context<User>) -> Result<()> {
        let vot_of_cars = &mut ctx.accounts.cars;
        vot_of_cars.ferrari = 0;
        vot_of_cars.mclarren = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct User <'info>{
    #[account(mut)]
    signer : Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + 16 ,
        seeds = [b"vote",signer.key().as_ref()],
        bump
        
    )]
    cars : Account <'info , Cars>,
    system_program : Program<'info, System>,
}


#[account] 
pub struct Cars {
    pub ferrari : u64,
    pub mclarren : u64,
}