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

    pub fn vote_for_cars_ferrari(ctx : Context<Vote>) -> Result<()> {
        
        let vot_for_car = &mut ctx.accounts.cars;
        vot_for_car.ferrari += 1;
        Ok(())
    }

    pub fn vote_for_cars_mclarre(ctx : Context<Vote>) -> Result<()> {
        
        let vot_for_car = &mut ctx.accounts.cars;
        vot_for_car.mclarren += 1;
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

#[derive(Accounts)]
pub struct Vote <'info>{
    #[account(mut)]
    signer : Signer<'info>,

    #[account(
        mut,
        seeds = [b"vote",signer.key().as_ref()],
        bump  
    )]
    cars : Account <'info , Cars>
}

#[account] 
pub struct Cars {
    pub ferrari : u64,
    pub mclarren : u64,
}